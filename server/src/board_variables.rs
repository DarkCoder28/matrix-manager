use chrono::{Datelike, Timelike, Weekday};
use serde_json::Value;
use shared::board_variables::{BoardVariable, TimeData};
use tracing::{info, warn};

static DEBUG: bool = false;

use crate::{
    config_manager::Config,
    state_manager::{StateWrapper, VariableCache},
};

pub trait EvaluateBoardVariable {
    async fn eval_variable(
        &self,
        variable_name: &str,
        config: &Config,
        state: StateWrapper,
    ) -> String;
    fn determine_substr_end(e_in: i16, data: &str) -> usize;
    fn weekday_to_string(day: Weekday) -> String;
    fn month_to_string(month: u32) -> String;
}
impl EvaluateBoardVariable for BoardVariable {
    #[allow(dead_code)]
    async fn eval_variable(
        &self,
        variable_name: &str,
        config: &Config,
        state: StateWrapper,
    ) -> String {
        match self {
            BoardVariable::URL(var_id, url, expiry, headers) => {
                let datetime = chrono::Utc::now();
                let mut state = state.lock().await;
                if DEBUG {
                    info!("{:#?}", &state.board_variable_values);
                }
                if let Some(var_cache) = state.board_variable_values.get(&var_id.to_string()) {
                    if DEBUG {
                        info!("Cache Debug:\nTime Entered: {}\nCurrent Time: {}\nExpiry: {}\nCurrent-Expiry: {}", &var_cache.time_entered, &datetime.timestamp(), expiry, &(datetime.timestamp()-expiry));
                    }
                    if !(var_cache.time_entered < datetime.timestamp() - expiry) {
                        // Use Cached Variable if not Expired
                        return var_cache.value.clone();
                    }
                }
                // Update Variable Cache
                info!(
                    "Updating Variable Cache for URL variable '{}'",
                    &variable_name
                );
                let mut req = ureq::get(url);
                for (header, value) in headers {
                    req = req.set(header, value);
                }
                let response = req.call();
                match response {
                    Ok(resp) => match resp.into_string() {
                        Ok(x) => {
                            state.board_variable_values.insert(
                                var_id.to_string(),
                                VariableCache {
                                    time_entered: datetime.timestamp(),
                                    value: x.clone(),
                                },
                            );
                            return x;
                        }
                        Err(e) => {
                            warn!("Error requesting url ({}):\n{}", url, e.to_string());
                            return String::new();
                        }
                    },
                    Err(e) => {
                        warn!("Error requesting url ({}):\n{}", url, e.to_string());
                        return String::new();
                    }
                }
            }
            BoardVariable::JsonURL(url_var_id, path, round_numbers, substring) => {
                let url_var = config
                    .board_variables
                    .iter()
                    .find(|var| var.1.get_url_if_id_matches_or_none(&url_var_id).is_some());
                if url_var.is_none() {
                    tracing::warn!("No URL variable exists with the ID '{}'", url_var_id);
                    return String::new();
                }
                let (url_var_name, url_var) = url_var.unwrap();
                let return_val =
                    Box::pin(url_var.eval_variable(&url_var_name, config, state.clone()));
                let return_val = return_val.await;
                let json_data = serde_json::from_str::<serde_json::Value>(&return_val);
                if json_data.is_err() {
                    tracing::warn!(
                        "Attempting to input non-json data to JsonURL. Url id: '{}'",
                        url_var_id
                    );
                    return String::new();
                }
                let mut json_data = json_data.unwrap();
                for item in path.split(".") {
                    let temp: Option<&Value>;
                    if json_data.is_array() {
                        let item_num = usize::from_str_radix(item, 10);
                        if item_num.is_err() {
                            tracing::warn!(
                                "Error parsing number '{}' for use in indexing array:\n{}",
                                item,
                                json_data.to_string()
                            );
                        }
                        let item_num = item_num.unwrap();
                        temp = json_data.get(item_num);
                    } else {
                        temp = json_data.get(item);
                    }
                    if temp.is_none() {
                        tracing::warn!(
                            "Error finding item '{}' on object with data:\n{}",
                            item,
                            json_data.to_string()
                        );
                        return String::new();
                    }
                    json_data = temp.unwrap().to_owned();
                }
                let mut data = json_data.to_string();
                if json_data.is_string() {
                    data = data.split_off(1);
                    let _ = data.split_off(data.len() - 1);
                }
                if let Some((start, end)) = substring {
                    let start = *start as usize;
                    let end = Self::determine_substr_end(*end, &data);
                    let new_str = &data[start..end];
                    if *round_numbers {
                        if let Ok(num) = new_str.parse::<f32>() {
                            let num = num.round();
                            return num.to_string();
                        }
                        tracing::warn!("Unable to parse number '{}'.", &new_str);
                    }
                    return new_str.to_string();
                } else {
                    if *round_numbers {
                        if let Ok(num) = data.parse::<f32>() {
                            let num = num.round();
                            return num.to_string();
                        }
                        tracing::warn!("Unable to parse number '{}'.", &data);
                    }
                    return data;
                }
            }
            BoardVariable::Time(time_data) => {
                let datetime = chrono::Local::now();
                match time_data {
                    TimeData::Weekday => {
                        return Self::weekday_to_string(datetime.weekday());
                    }
                    TimeData::Time => {
                        let am_pm_text = match (datetime.hour() / 12) as i32 == 0 {
                            true => "AM",
                            false => "PM",
                        };
                        let hour_text = format!(
                            "{:02}",
                            match datetime.hour() % 12 {
                                0 => 12,
                                x => x,
                            }
                        );
                        let min_text = format!("{:02}", datetime.minute());
                        return format!("{}:{} {}", &hour_text, &min_text, &am_pm_text);
                    }
                    TimeData::Date => {
                        return format!(
                            "{} {} {}",
                            Self::month_to_string(datetime.month()),
                            datetime.day(),
                            datetime.year()
                        );
                    }
                }
                // DOW
                // TIME AM/PM
                // Mon DY YEAR
            }
        }
    }

    fn determine_substr_end(e_in: i16, data: &str) -> usize {
        let length = data.len();
        if e_in == 0 {
            return length
        } else if e_in < 0 {
            return length.saturating_sub(e_in.abs() as usize)
        } else {
            if (e_in as usize) < length {
                return e_in as usize
            } else {
                return length
            }
        };
    }

    fn weekday_to_string(day: Weekday) -> String {
        match day {
            Weekday::Sun => "Sunday".to_string(),
            Weekday::Mon => "Monday".to_string(),
            Weekday::Tue => "Tuesday".to_string(),
            Weekday::Wed => "Wednesday".to_string(),
            Weekday::Thu => "Thursday".to_string(),
            Weekday::Fri => "Friday".to_string(),
            Weekday::Sat => "Saturday".to_string(),
        }
    }

    fn month_to_string(month: u32) -> String {
        match month {
            1 => "Jan".to_string(),
            2 => "Feb".to_string(),
            3 => "Mar".to_string(),
            4 => "Apr".to_string(),
            5 => "May".to_string(),
            6 => "Jun".to_string(),
            7 => "July".to_string(),
            8 => "Aug".to_string(),
            9 => "Sept".to_string(),
            10 => "Oct".to_string(),
            11 => "Nov".to_string(),
            12 => "Dec".to_string(),
            _ => "Nul".to_string(),
        }
    }
}
