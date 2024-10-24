use tracing::{error, info};

use crate::state::CanvasState;

pub fn set_brightness(command: &str, canvas: &mut rpi_led_panel::Canvas, state: &mut CanvasState) {
    let brightness = &command[1..4];
    match brightness.parse::<u8>() {
        Ok(b) => {
            info!("Setting brightness to: {}", b);
            state.brightness = b;
            canvas.set_brightness(b);
        },
        Err(_) => {
            error!("Failed to parse brightness: {}", brightness);
        }
    }
}