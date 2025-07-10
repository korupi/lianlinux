use crate::{core, daemon::{self, protocol::Request}};
use clap::{Parser, Subcommand};

/// # Software to control Lian Li hub lights on Linux
#[derive(Debug, Parser)]
#[clap(name = "lianlinux", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

/// # Enum with subcommands
#[derive(Debug, Subcommand)]
enum Command {
    /// Run lianlinux as a daemon
    Daemon {},

    /// Control the lights
    Light {
        /// Light mode
        mode: String,

        /// Hex color (e.g. FF0000)
        color: Option<String>
    }
}

/// # Handle command line arguments
pub async fn handle_args() {
    let args = App::parse();

    match args.command {
        Command::Daemon {  } => {
            let controller = core::init().expect("Controller initialization failed");
            let d = daemon::init(controller);
            match d {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{e}");
                }
            }
        }
        Command::Light { mode, color } => {
            // Prepare args
            let args = color
                .map(|c| vec![c]) // wrap in Vec<String>
                .or(Some(vec![])); // fallback to empty vec if None
            // Send request
            if let Err(e) = daemon::client::send_message(
                Request {
                    mode,
                    args,
            }
            ) {
                eprintln!("Error sending request: {}", e)
            }
        }
    }
}
