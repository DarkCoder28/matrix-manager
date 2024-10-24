use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb888, prelude::*};
use tracing::info;

pub fn clear<T: DrawTarget<Color = Rgb888>>(canvas: &mut T) {
    info!("");
    info!("Rendering...");
    let _ = canvas.clear(Rgb888::BLACK);
}