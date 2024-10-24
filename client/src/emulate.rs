use std::{io::{Read, Write}, net::TcpStream};

use embedded_graphics::{mono_font::iso_8859_1::FONT_5X8, pixelcolor::Rgb888, prelude::{DrawTarget, RgbColor, Size}};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window};
use pico_args::Arguments;
use tempfile::TempDir;

use crate::{commands::interpret::interpret, state::CanvasState};


pub fn run_emulator(image_cache: TempDir) {
    // Get arguments
    let mut args = Arguments::from_env();
    let server_uri: String = args.value_from_str("-s").unwrap_or(String::from("192.168.1.64:12312"));
    let server_http_uri: String = args.value_from_str("-h").unwrap_or(String::from("http://192.168.1.64:12345"));
    let size_x = args.value_from_str("-x").unwrap_or(64);
    let size_y = args.value_from_str("-y").unwrap_or(32);
    let size = Size::new(size_x, size_y);
    //
    let mut socket = connect_socket(&server_uri);
    // Protocol V1
    {
        let _ = socket.write("1\n".as_bytes());
        let _ = socket.write(format!("{}\n", size.width).as_bytes());
        let _ = socket.write(format!("{}\n", size.height).as_bytes());
    }
    let mut state = CanvasState {
        colour: Rgb888::WHITE,
        font: &FONT_5X8,
        font_offset: 0,
        brightness: 100,
        server_http_uri,
    };

    let mut display = SimulatorDisplay::<Rgb888>::new(size.clone());
    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(1)
        .scale(8)
        .max_fps(15)
        .build();
    let mut window = Window::new("Matrix Emulator", &output_settings);
    
    'running: loop {
        render(&mut socket, &mut display, &mut state, &image_cache);
        window.update(&display);
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {},
            }
        }
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

fn render<T: DrawTarget<Color = Rgb888>>(socket: &mut TcpStream, display: &mut T, state: &mut CanvasState, image_cache: &TempDir) {
    let mut buf = [0; 10];
    let result = socket.read(&mut buf);
    if result.is_err() {
        tracing::error!("Connection to server closed... shutting down");
        panic!();
    }
    let command = std::str::from_utf8(&buf).unwrap();
    // info!("{}", command);
    interpret(command, display, state, image_cache)
}