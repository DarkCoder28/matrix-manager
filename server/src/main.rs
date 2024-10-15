#![forbid(unsafe_code)]

use std::sync::Arc;
use config_manager::ConfigWrapper;
use pico_args::Arguments;
use state_manager::State;
use tokio::sync::{Mutex, RwLock};

mod web_interface;
mod matrix_server;
mod board_variables;
mod boards;

mod config_manager;
mod state_manager;
mod font_manager;
mod image_manager;

#[tokio::main]
async fn main() {
    let logging_subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::TRACE).finish();
    tracing::subscriber::set_global_default(logging_subscriber).expect("Failed to setup logging");

    let mut args = Arguments::from_env();
    let custom_config_path: Result<String, pico_args::Error> = args.value_from_str("-c");
    let custom_config_path = match custom_config_path {
        Ok(x) => Some(x as String),
        Err(_) => None,
    };
    let running_config: ConfigWrapper = Arc::new(RwLock::new(config_manager::Config::from_async(custom_config_path).await));

    let state = Arc::new(Mutex::new(State::new()));

    let web_server = tokio::spawn(web_interface::web::run_web_server(running_config.clone(), state.clone()));
    let matrix_server = tokio::spawn(matrix_server::server::run_matrix_server(running_config.clone(), state.clone()));
    let _ = matrix_server.await;
    web_server.abort();
    let _ = web_server.await;
}
