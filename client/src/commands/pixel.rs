use embedded_graphics::{geometry::Point, pixelcolor::Rgb888, prelude::DrawTarget, Drawable, Pixel};
use tracing::{error, info};

use crate::state::CanvasState;

pub fn draw_pixel<T: DrawTarget<Color = Rgb888>>(command: &str, canvas: &mut T, state: &CanvasState) {
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
    info!(
        "Setting pixel ({}, {}) to current colour.",
        data[0], data[1]
    );
    let _ = Pixel(Point::new(data[0] as i32, data[1] as i32), state.colour).draw(canvas);
}

pub fn draw_coloured_pixel<T: DrawTarget<Color = Rgb888>>(command: &str, canvas: &mut T) {
    let data_str = [
        &command[1..3],
        &command[3..5],
        &command[5..6],
        &command[6..7],
        &command[7..8],
    ];
    let mut data: [u8; 5] = [0; 5];
    for (idx, num_str) in data_str.iter().enumerate() {
        if idx < 2 {
            match num_str.parse::<u8>() {
                Ok(num) => data[idx] = num,
                Err(_) => {
                    error!("Failed to parse integer: {}", num_str);
                    return;
                }
            }
        } else {
            match u8::from_str_radix(&num_str, 16) {
                Ok(num) => data[idx] = num,
                Err(_) => {
                    error!("Failed to parse integer: {}", num_str);
                    return;
                }
            }
        }
    }
    info!(
        "Setting pixel ({}, {}) to current colour.",
        data[0], data[1]
    );
    let _ = Pixel(
        Point::new(data[0] as i32, data[1] as i32),
        Rgb888::new(data[2] * 0x10, data[3] * 0x10, data[4] * 0x10),
    )
    .draw(canvas);
}
