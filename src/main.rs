use cmd::handle_args;

extern crate hidapi;

mod core;
mod daemon;
mod cmd;

/// # Main `lianlinux` function
///
/// Initializes `core` module if ran as root and handles 
/// command line arguments asynchrously
#[tokio::main]
async fn main() {
    //let controller = core::init().expect("u");
    handle_args().await;
}
