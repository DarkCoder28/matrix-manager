use embedded_graphics::pixelcolor::Rgb888;
use tracing::{error, info};

use crate::state::CanvasState;

pub fn set_colour(command: &str, state: &mut CanvasState) {
    let r = &command[1..2];
    let g = &command[2..3];
    let b = &command[3..4];
    let mut col: [u8;3] = [0;3];
    match u8::from_str_radix(r, 16) {
        Ok(r) => {
            col[0]=r;
        },
        Err(_) => {
            error!("Failed to parse colour (R,x,x): {}", r);
            return;
        }
    }
    match u8::from_str_radix(g, 16) {
        Ok(g) => {
            col[1]=g;
        },
        Err(_) => {
            error!("Failed to parse colour (x,G,x): {}", g);
            return;
        }
    }
    match u8::from_str_radix(b, 16) {
        Ok(b) => {
            col[2]=b;
        },
        Err(_) => {
            error!("Failed to parse colour (x,x,B): {}", b);
            return;
        }
    }
    col = col.map(|c| {c*0x10});
    info!("Setting colour to ({}, {}, {})", col[0], col[1], col[2]);
    state.colour = Rgb888::new(col[0], col[1], col[2]);
}