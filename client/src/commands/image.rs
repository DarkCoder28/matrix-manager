use std::fs::File;

use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::{DrawTarget, Point, Drawable}};
use tempfile::TempDir;
use tinybmp::Bmp;

use crate::state::CanvasState;


const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.3";

pub fn draw_image<T: DrawTarget<Color = Rgb888>>(command: &str, canvas: &mut T, state: &CanvasState, image_cache: &TempDir) {
    let pos_str = [&command[1..3], &command[3..5]];
    let mut pos: [i32;2] = [0;2];
    for (idx, num_str) in pos_str.iter().enumerate() {
        match num_str.parse::<i32>() {
            Ok(num) => pos[idx] = num,
            Err(_) => {
                tracing::warn!("Failed to parse integer: {}", num_str);
                return;
            }
        }
    }
    let image_hash = &command[5..10];
    let expected_image_path = image_cache.path().join(format!("{}.bmp", image_hash));
    if !expected_image_path.exists() {
        download_image(&state.server_http_uri, image_cache, image_hash);
    }
    let image_data = std::fs::read(&expected_image_path);
    if image_data.is_err() {
        tracing::error!("Unable to read image at [{:#?}]", expected_image_path);
        panic!();
    }
    let image_data = image_data.unwrap();
    let image: Bmp<'_, Rgb888> = Bmp::from_slice(&image_data).unwrap();
    let _ = Image::new(&image, Point::new(pos[0] as i32, pos[1] as i32)).draw(canvas);
}

fn download_image(server_http_uri: &str, image_cache: &TempDir, image_hash: &str) {
    let output_file_path = image_cache.path().join(format!("{}.bmp", image_hash));
    let mut output_file = File::create(&output_file_path).expect(&format!("Unable to make temporary image file at [{:#?}]", &output_file_path));
    let remote_name = format!("{}/api/get_image/{}", server_http_uri, image_hash);
    let file_stream = ureq::get(&remote_name).set("User-Agent", USER_AGENT).call();
    if (&file_stream).is_err() {
        tracing::error!(
            "Error downloading \"{}\"\n{}",
            &remote_name,
            &file_stream.unwrap_err().to_string()
        );
        panic!();
    }
    let file_stream = file_stream.unwrap();
    if file_stream.status() == 200 {
        let mut stream_reader = file_stream.into_reader();
        let _ = std::io::copy(&mut stream_reader, &mut output_file);
    } else {
        tracing::error!(
            "Error downloading \"{}\"\nHTTP Response Code {}: {}",
            &remote_name,
            (&file_stream).status(),
            (&file_stream).status_text()
        );
        panic!();
    }
}