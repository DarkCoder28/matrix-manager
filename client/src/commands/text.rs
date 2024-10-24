use embedded_graphics::{geometry::Point, mono_font::iso_8859_1::{FONT_5X8, FONT_7X14_BOLD}, pixelcolor::Rgb888, prelude::*, text::Text};
use tracing::{error, info};

use crate::state::CanvasState;

pub fn set_font(command: &str, state: &mut CanvasState) {
    let mut font_name_end: usize = 10;
    if let Some(len) = command.find('=') {
        font_name_end = len;
    }
    match &command[1..font_name_end] {
        "5x8" => {
            state.font = &FONT_5X8;
            state.font_offset = 6;
        },
        "7x14B" => {
            state.font = &FONT_7X14_BOLD;
            state.font_offset = 12;
        },
        _=>{info!("Invalid font: {}", &command[1..font_name_end])},
    };
}

pub fn draw_character<T: DrawTarget<Color = Rgb888>>(command: &str, canvas: &mut T, state: &CanvasState) {
    let special_char = command.starts_with(char::from_u32(0x6A).unwrap());
    let data_str = [&command[1..3], &command[3..5]];
    let mut data: [u8; 2] = [0; 2];
    for (idx, num_str) in data_str.iter().enumerate() {
        match num_str.parse::<u8>() {
            Ok(num) => data[idx] = num,
            Err(_) => {
                error!("Failed to parse integer: {}", num_str);
                return;
            }
        }
    }
    let character =  if !special_char {command.chars().nth(5).unwrap_or('_')} else {
        match command.chars().nth(5).unwrap_or('_') {
            '1' => '°',
            _=>{error!("Unknown Character '{:#?}'", command.chars().nth(5));return;}
        }
    };
    info!("Drawing character ({}) at ({},{})", character, data[0], data[1]);
    let _ = Text::new(character.to_string().as_str(), Point::new(data[0] as i32, ((data[1]+state.font_offset) as u32) as i32), state.text_style()).draw(canvas);
}