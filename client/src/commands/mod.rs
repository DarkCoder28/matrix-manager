#[cfg(not(target_arch = "x86_64"))]
pub mod brightness;
pub mod clear;
pub mod colour;
pub mod image;
pub mod interpret;
pub mod line;
pub mod pixel;
pub mod text;