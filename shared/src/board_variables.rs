use std::{collections::{hash_map, HashMap}, fmt::Display, str::FromStr};

use rand::Rng;
use serde::{Deserialize, Serialize};

pub type BoardVariables = HashMap<String, BoardVariable>;

#[derive(Serialize, Deserialize, Clone)]
pub enum BoardVariable {
    URL(
        u32, /*var_id*/
        String, /*url*/
        i64,    /*expiry-secs*/
        HashMap<String, String>, /* headers */
    ),
    JsonURL(u32 /*URL var_id*/, String /*path*/, bool /* round_numbers */, Option<(u8, i16)> /*substring*/),
    Time(TimeData),
}

impl BoardVariable {
    pub fn get_all_variable_types() -> Vec<String> {
        vec![
            String::from("HTTP Request"),
            String::from("URL JSON Value Extractor"),
            String::from("DateTime"),
        ]
    }
    pub fn get_variable_type(&self) -> String {
        return match self {
            BoardVariable::URL(_id, _url, _expiry, _headers) => String::from("HTTP Request"),
            BoardVariable::JsonURL(_url_id, _json_path, _round_numbers, _substring) => String::from("URL JSON Value Extractor"),
            BoardVariable::Time(_time_data) => String::from("DateTime"),
        };
    }
    pub fn get_default_by_type(var_type: &str) -> BoardVariable {
        return match var_type {
            "HTTP Request" => BoardVariable::URL(
                get_rand(),
                String::from("https://jsonplaceholder.typicode.com/todos/"),
                30,
                hash_map::HashMap::new(),
            ),
            "URL JSON Value Extractor" => {
                BoardVariable::JsonURL(get_rand(), String::from("0.title"), false, None)
            }
            "DateTime" => BoardVariable::Time(TimeData::Time),
            _ => BoardVariable::Time(TimeData::Time),
        };
    }
    pub fn get_url_if_id_matches_or_none(&self, check_id: &u32) -> Option<String> {
        return match self {
            BoardVariable::URL(id, url, _timeout, _headers) => {
                if id.eq(check_id) {
                    Some(url.clone())
                } else {
                    None
                }
            }
            _ => None,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TimeData {
    Weekday(u8/* offset */, Option<(u8, i16)> /*substring*/),
    Time,
    Date,
}

impl Display for TimeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            TimeData::Weekday(_, _) => String::from("Weekday"),
            TimeData::Time => String::from("Time"),
            TimeData::Date => String::from("Date"),
        };
        write!(f, "{}", &out)
    }
}

impl FromStr for TimeData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Weekday" => Self::Weekday(0, None),
            "Time" => Self::Time,
            "Date" => Self::Date,
            _ => Self::Time,
        })
    }
}

impl TimeData {
    pub fn get_all_time_data_types() -> Vec<String> {
        vec!["Weekday".to_string(), TimeData::Time.to_string(), TimeData::Date.to_string()]
    }
}

fn get_rand() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
