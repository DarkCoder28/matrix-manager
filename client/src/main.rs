// #[cfg(not(target_arch = "x86_64"))]
mod drive_matrix;
#[cfg(target_arch = "x86_64")]
mod emulate;

pub mod state;
pub mod commands;

use tracing::info;

// Raspberry Pi w/ Adafruit Matrix Bonnet
#[cfg(not(target_arch = "x86_64"))]
fn main() {
    let logging_subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::TRACE).finish();
    tracing::subscriber::set_global_default(logging_subscriber).expect("Failed to setup logging");
    // Setup image folder
    let image_cache = tempfile::tempdir().expect("Failed to setup temporary directory for image caching");
    // Start Matrix
    info!("Starting matrix...");
    drive_matrix::start_matrix(image_cache);
}

// Non-Raspberry Pi
#[cfg(target_arch = "x86_64")]
fn main() {
    let logging_subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::TRACE).finish();
    tracing::subscriber::set_global_default(logging_subscriber).expect("Failed to setup logging");
    // Setup image folder
    let image_cache = tempfile::tempdir().expect("Failed to setup temporary directory for image caching");
    // Start Matrix Emulator
    info!("Starting emulator...");
    emulate::run_emulator(image_cache);
    std::process::exit(0);
}