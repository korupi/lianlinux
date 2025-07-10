extern crate hidapi;

mod core;
mod packet;

/// # Main `lianlinux` function
///
/// Initializes `core` module if ran as root and handles 
/// command line arguments asynchrously
//#[tokio::main]
fn main() {
    core::init();
}
