use embedded_graphics::{mono_font::{MonoFont, MonoTextStyle}, pixelcolor::Rgb888};

pub struct CanvasState<'a> {
    pub colour: Rgb888,
    pub font: &'a MonoFont<'a>,
    pub font_offset: u8,
    pub brightness: u8,
    pub server_http_uri: String,
}
impl<'a> CanvasState<'a> {
    pub fn text_style(&self) -> MonoTextStyle<'a, Rgb888> {
        MonoTextStyle::new(&self.font, self.colour.clone())
    }
}