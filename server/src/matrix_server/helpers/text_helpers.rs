use std::path::PathBuf;

use bdf::Bitmap;
use shared::{boards::{ColourOption, ElementColour}, device_config::DeviceConfig};

use crate::config_manager::ConfigWrapper;

pub(crate) async fn draw_text(config:ConfigWrapper, device_config: &DeviceConfig, board_name: &str, x: Option<u8>, y: u8, colour: &ColourOption, font: &Option<String>, text: String) -> String {
    let mut instructions = String::new();
    let colour = match colour {
        ColourOption::Default => ElementColour::default(),
        ColourOption::Specific(element_colour) => element_colour.to_owned(),
        ColourOption::ParseTemperature => {
            let mut col = ElementColour::default();
            let re = regex::Regex::new(r"(?P<temp>\d+)").unwrap();
            let cap = re.captures(&text);
            if cap.is_some() {
            let cap = cap.unwrap();
                let temp = &cap["temp"];
                if let Ok(temp) = temp.parse::<i32>() {
                    col = device_config.temperature_colours.get_colour(temp);
                }
            }
            col
        },
    };
    let font_name = truncate_string(font.clone().unwrap_or(String::from("5x8")), 9);
    instructions.push_str(&colour.to_string());
    instructions.push_str(&format!("f{:=<9}", font_name));

    // Get Character Width
    let char_width = get_glyph_from_char(config.clone(), &font_name, 'A').await.width();
    
    // Center text if x_pos is None
    let x = match x {
        Some(x) => x,
        None => {
            let board_width = config.read().await.get_boards().get(board_name).unwrap().size.0 as f32;
            let text_width = char_width as f32*text.len() as f32;
            let left_margin = ((board_width-text_width)/2f32).floor() as u8;
            left_margin
        }
    };

    let mut pos_x = x as u32;
    for character in text.chars() {
        let glyph = get_glyph_from_char(config.clone(), &font_name, character).await;
        if character == '°' {
            instructions.push_str(&format!("j{:02}{:02}1====", pos_x, y));
        } else {
            instructions.push_str(&format!("t{:02}{:02}{}====", pos_x, y, character));
        }
        pos_x+=glyph.width();
    }

    instructions
}

fn truncate_string(text: String, length: usize) -> String {
    if text.len() > length {
        let new_text = String::from(text.split_at(length).0);
        tracing::warn!("Text \"{}\" too long! Truncating to {} characters ({})! This WILL cause issues!", &text, length, &new_text);
        return new_text;
    }
    text
}

async fn get_glyph_from_char(config: ConfigWrapper, font_name: &str, character: char) -> Bitmap {
    let mut font_path = PathBuf::from(config.read().await.config_path.clone()).parent().expect("Config file has no parent???").to_path_buf();
    font_path.push(format!("assets/fonts/{}.bdf", font_name));
    let font = match bdf::open(font_path.as_path()) {
        Ok(x) => x,
        Err(_) => {
            tracing::info!("Font path: {}", font_path.display());
            tracing::warn!("Error loading font {}!", font_name);
            return Bitmap::new(5, 8);
        }
    };
    match font.glyphs().get(&character) {
        Some(x) => x.map().to_owned(),
        None => Bitmap::new(5, 8)
    }
}