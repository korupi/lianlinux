use std::any;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};

use anyhow::Context;
use protocol::Request;
use serde::Serialize;

use crate::core::devices::Controller;
use crate::core::modes::{breathing_mode, morph_mode, rainbow_mode, runway_mode, static_mode};

pub mod client;
pub mod protocol;

pub fn init(controller: Controller) -> anyhow::Result<()> {
    let controller = Arc::new(Mutex::new(controller));
    let socket_path = "/tmp/lianlinux.socket";

    if std::fs::metadata(socket_path).is_ok() {
        std::fs::remove_file(socket_path).with_context(|| {
            format!("could not delete previous socket at {:?}", socket_path)
        })?;
    }

    let unix_listener =
        UnixListener::bind(socket_path).context("Could not create the unix socket")?;

    loop {
        let (unix_stream, _socket_address) = unix_listener
            .accept()
            .context("Failed at accepting a connection on the unix listener")?;

        let controller = Arc::clone(&controller);

        handle_stream(unix_stream, controller)?;
    }
}

pub fn handle_stream(mut unix_stream: UnixStream, controller: Arc<Mutex<Controller>>) -> anyhow::Result<()> {
    let mut message = String::new();
    unix_stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    let request: Request = serde_json::from_str(&message)?;

    match request.mode.as_str() {
        "static" => {
            if let Some(args) = &request.args {
                match parse_color(args) {
                    Ok(color) => {
                        let _ = static_mode(controller, &color);
                    }
                    Err(e) => respond(&mut unix_stream, &format!("{e}")),
                }
            } else {
                respond(&mut unix_stream, "Missing args for static mode");
            }
        }

        "breathing" => {
            if let Some(args) = &request.args {
                match parse_color(args) {
                    Ok(color) => {
                        let _ = breathing_mode(controller, &color);
                    }
                    Err(e) => respond(&mut unix_stream, &format!("{e}")),
                }
            } else {
                respond(&mut unix_stream, "Missing args for breathing mode");
            }
        }

        "rainbow" => {
            let _ = rainbow_mode(controller);
        }

        "morph" => {
            let _ = morph_mode(controller);
        }

        "runway" => {
            if let Some(args) = &request.args {
                match parse_color(args) {
                    Ok(color) => {
                        let _ = runway_mode(controller, &color);
                    }
                    Err(e) => respond(&mut unix_stream, &format!("{e}")),
                }
            } else {
                respond(&mut unix_stream, "Missing args for runway mode");
            }
        }

        _ => {
            eprintln!("Unknown mode: {}", request.mode);
        }
    }

    Ok(())
}

fn respond(unix_stream: &mut UnixStream, message: &str) {
    let _ = unix_stream
        .write(&message.to_string().into_bytes())
        .context("Failed at writing onto the unix stream");
}

fn parse_color(args: &[String]) -> anyhow::Result<Vec<u8>> {
    if args.is_empty() {
        anyhow::bail!("No color argument provided");
    }

    let color_str = args[0].trim_start_matches('#');
    if color_str.len() != 6 {
        anyhow::bail!("Expected 6-digit hex color (like #RRGGBB), got: {}", args[0]);
    }

    let r = u8::from_str_radix(&color_str[0..2], 16)
        .context("Failed to parse red component")?;
    let g = u8::from_str_radix(&color_str[2..4], 16)
        .context("Failed to parse green component")?;
    let b = u8::from_str_radix(&color_str[4..6], 16)
        .context("Failed to parse blue component")?;

    Ok(vec![r, g, b])
}
