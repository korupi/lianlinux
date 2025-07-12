use cmd::handle_args;

extern crate hidapi;

mod core;
mod daemon;
mod cmd;
mod config;
mod utils;

/// # Main `lianlinux` function
///
/// Handles command line arguments
#[tokio::main]
async fn main() {
    handle_args().await;
}
