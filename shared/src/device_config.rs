use std::collections::HashMap;

use chrono::Timelike;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use log::warn;

#[cfg(not(target_arch = "wasm32"))]
use tracing::warn;

pub type DeviceConfigs = HashMap<String, DeviceConfig>;
pub type Brightnesses = Vec<Brightness>;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceConfig {
    pub name: String,
    pub size: (u8, u8), // Ex: 64x32
    pub temperature_colours: TemperatureColours,
    pub boards: Vec<String>,
    pub brightness: Brightnesses,
    pub picture_of_the_day_brightness_threshold: u8,
    pub proto_version: u64,
}
impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            name: String::from("New Device"),
            size: (64, 32),
            temperature_colours: TemperatureColours::default(),
            boards: vec![String::from("clock")],
            brightness: Vec::new(),
            picture_of_the_day_brightness_threshold: 25,
            proto_version: 0,
        }
    }
}

#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Brightness {
    pub time: String,
    pub percentage: u8
}
impl Default for Brightness {
    fn default() -> Self {
        Brightness { time: String::from("24:00"), percentage: 66}
    }
}

pub fn get_current_brightness(brightnesses: &Brightnesses) -> u8 {
    let current_time = cur_time_ms();
    for brightness in brightnesses {
        let time = parse_time_string(&brightness.time);
        if current_time < time {
            return brightness.percentage;
        }
    }
    66
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemperatureColours {
    pub freezing: i16,
    pub cold: i16,
    pub neutral: i16,
    pub warm: i16,
}
impl Default for TemperatureColours {
    fn default() -> Self {
        Self {
            freezing: 32,
            cold: 40,
            neutral: 72,
            warm: 82,
        }
    }
}
impl TemperatureColours {
    pub fn get_colour(self, temp: i32) -> String {
        if temp <= self.freezing as i32 {
            return String::from("c0FF======");
        } else if temp <= self.cold as i32 {
            return String::from("c48F======");
        } else if temp <= self.neutral as i32 {
            return String::from("cDD2======");
        } else if temp <= self.warm as i32 {
            return String::from("cD91======");
        } else {
            return String::from("cF00======");
        }
    }
}


pub fn parse_time_string(time: &str) -> u32 {
    let split = time.split(":").collect::<Vec<&str>>();
    if split.len() != 2 {
        warn!("Improperly formatted time string: \"{}\"", &time);
        return u32::MIN;
    }
    let hours = match u32::from_str_radix(&split[0], 10) {
        Ok(x) => x,
        Err(_) => {
            warn!("Error parsing hour ({}) from, string: \"{}\"", &split[0], &time);
            return u32::MIN;
        }
    };
    let minutes = match u32::from_str_radix(&split[1], 10) {
        Ok(x) => x,
        Err(_) => {
            warn!("Error parsing minute ({}) from, string: \"{}\"", &split[1], &time);
            return u32::MIN;
        }
    };
    (hours*3600000) + (minutes*60000)
}

pub fn cur_time_ms() -> u32 {
    let datetime = chrono::Local::now();
    (datetime.hour()*3600000) + (datetime.minute()*60000) + (datetime.second()*1000)
}