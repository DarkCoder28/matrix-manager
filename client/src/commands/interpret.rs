use embedded_graphics::{pixelcolor::Rgb888, prelude::DrawTarget};
use tempfile::TempDir;
use tracing::info;

use crate::state::CanvasState;

use super::{clear::clear, colour::set_colour, image::draw_image, line::draw_line, pixel::{draw_coloured_pixel, draw_pixel}, text::{draw_character, set_font}};
#[cfg(not(target_arch = "x86_64"))]
use super::brightness::set_brightness;

#[cfg(not(target_arch = "x86_64"))]
pub fn rgb_interpret(
    command: &str,
    canvas: &mut rpi_led_panel::Canvas,
    state: &mut CanvasState,
    image_cache: &TempDir,
) {
    match command.chars().nth(0).unwrap() {
        'b' => set_brightness(command, canvas, state),
        _ => interpret(command, canvas, state, image_cache),
    }
}

pub fn interpret<T: DrawTarget<Color = Rgb888>>(
    command: &str,
    canvas: &mut T,
    state: &mut CanvasState,
    image_cache: &TempDir,
) {
    match command.chars().nth(0).unwrap() {
        'x' => clear(canvas),
        'c' => set_colour(command, state),
        'l' => draw_line(command, canvas, state),
        'p' => draw_pixel(command, canvas, state),
        'q' => draw_coloured_pixel(command, canvas),
        'f' => set_font(command, state),
        't' | 'j' => draw_character(command, canvas, state),
        'i' => draw_image(command, canvas, state, image_cache),
        // 'I' => draw_image_of_day(command, canvas),
        's' => info!("Done.\n"),
        _ => info!("{}\n", command),
    }
}
