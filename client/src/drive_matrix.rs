use std::{io::{Read, Write}, net::TcpStream, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration, env};

use embedded_graphics::{mono_font::iso_8859_1::FONT_5X8, pixelcolor::Rgb888, prelude::{RgbColor, Size}};
// use pico_args::Arguments;
use rpi_led_panel::{Canvas, HardwareMapping, RGBMatrix, RGBMatrixConfig};
use tempfile::TempDir;

use crate::{commands::interpret::rgb_interpret, state::CanvasState};



pub fn start_matrix(image_cache: TempDir) {
    // Get arguments
    let server_uri: String = env::var("SERVER_URI").unwrap_or(String::from("192.168.1.64:12312"));
    let server_http_uri: String = env::var("SERVER_HTTP_URI").unwrap_or(String::from("http://192.168.1.64:12345"));
    // let mut args = Arguments::from_env();
    // let server_uri: String = args.value_from_str("-s").unwrap_or(String::from("192.168.1.64:12312"));
    // let server_http_uri: String = args.value_from_str("-h").unwrap_or(String::from("http://192.168.1.64:12345"));
    //
    let matrix_config: RGBMatrixConfig = argh::from_env();
    let (mut matrix, canvas) = RGBMatrix::new(matrix_config, 0).expect("Matrix init failed.");
    let mut canvas = *canvas;
    //
    let mut socket = connect_socket(&server_uri);
    // Protocol V1
    {
        let _ = socket.write("1\n".as_bytes());
        let _ = socket.write(format!("{}\n", canvas.width()).as_bytes());
        let _ = socket.write(format!("{}\n", canvas.height()).as_bytes());
    }
    //
    let mut state = CanvasState {
        colour: Rgb888::WHITE,
        font: &FONT_5X8,
        font_offset: 0,
        brightness: 100,
        server_http_uri,
    };
    let canvas = Arc::new(Mutex::new(canvas));
    let background_canvas = canvas.clone();
    // Background Render Thread
    thread::spawn(move || {
        loop {
            {
                let canvas = background_canvas.lock().unwrap();
                matrix.update_on_vsync(Box::new(canvas.clone()));
            }
            sleep(Duration::from_millis(1));
        }
    });
    //
    loop {
        render(&mut socket, canvas.clone(), &mut state, &image_cache);
        // canvas = matrix.update_on_vsync(canvas.clone());
        // matrix.update_on_vsync(Box::new(canvas.clone()));
        // canvas = *
    }
}

fn connect_socket(server: &str) -> TcpStream {
    match TcpStream::connect(server) {
        Ok(stream) => { return stream; }
        Err(e) => {
            tracing::error!("Error connecting to socket:\n{:#?}", e);
            panic!();
        }
    }
}

fn render(socket: &mut TcpStream, display: Arc<Mutex<Canvas>>, state: &mut CanvasState, image_cache: &TempDir) {
    let mut buf = [0; 10];
    let result = socket.read(&mut buf);
    if result.is_err() {
        tracing::error!("Connection to server closed... shutting down");
        panic!();
    }
    let command = std::str::from_utf8(&buf).unwrap();
    // info!("{}", command);
    rgb_interpret(command, &mut *display.lock().unwrap(), state, image_cache);
}
