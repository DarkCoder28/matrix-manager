use embedded_graphics::{geometry::Point, pixelcolor::Rgb888, prelude::*, primitives::{Line, Primitive, PrimitiveStyleBuilder}};

use crate::state::CanvasState;



pub fn draw_line<T: DrawTarget<Color = Rgb888>>(command: &str, canvas: &mut T, state: &CanvasState) {
    let pos_str = [&command[1..3], &command[3..5], &command[5..7], &command[7..9]];
    let mut pos: [u8;4] = [0;4];
    for (idx, num_str) in pos_str.iter().enumerate() {
        match num_str.parse::<u8>() {
            Ok(num) => pos[idx] = num,
            Err(_) => {
                tracing::error!("Failed to parse integer: {}", num_str);
                return;
            }
        }
    }
    tracing::info!("Drawing line from ({}, {}), to ({}, {})", pos[0], pos[1], pos[2], pos[3]);
    let _ = Line::new(Point::new(pos[0] as i32, pos[1] as i32), Point::new(pos[2] as i32, pos[3] as i32)).into_styled(PrimitiveStyleBuilder::new().stroke_width(1).stroke_color(state.colour).build()).draw(canvas);
}