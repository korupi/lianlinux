use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};

use anyhow::Context;
use protocol::Request;

use crate::config;
use crate::core::devices::Controller;
use crate::core::modes::{breathing_mode, morph_mode, rainbow_mode, runway_mode, static_mode};
use crate::utils::parse_color;

pub mod client;
pub mod protocol;

pub fn init(controller: Controller) -> anyhow::Result<()> {
    let controller = Arc::new(Mutex::new(controller));
    config::init(Arc::clone(&controller))?;
    let socket_path = "/tmp/lianlinux.socket";

    if std::fs::metadata(socket_path).is_ok() {
        std::fs::remove_file(socket_path)
            .with_context(|| format!("could not delete previous socket at {:?}", socket_path))?;
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

pub fn handle_stream(
    mut unix_stream: UnixStream,
    controller: Arc<Mutex<Controller>>,
) -> anyhow::Result<()> {
    let mut message = String::new();
    unix_stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    let request: Request = serde_json::from_str(&message)?;
    let color: Vec<u8> = vec![];

    println!("{:?}", request.args);

    if let Some(args) = &request.args {
        match parse_color(args) {
            Ok(colors) => {
                let _ = set_mode(controller, &request.mode, &colors);
            }
            Err(e) => respond(&mut unix_stream, &format!("{e}")),
        }
    } else {
        let _ = set_mode(controller, &request.mode, &color);
    }

    Ok(())
}

fn respond(unix_stream: &mut UnixStream, message: &str) {
    let _ = unix_stream
        .write(&message.to_string().into_bytes())
        .context("Failed at writing onto the unix stream");
}

pub fn set_mode(
    controller: Arc<Mutex<Controller>>,
    mode: &str,
    color: &[u8],
) -> anyhow::Result<()> {
    match mode {
        "static" => {
            static_mode(controller, color)?;
        }

        "breathing" => {
            breathing_mode(controller, color)?;
        }

        "rainbow" => {
            rainbow_mode(controller)?;
        }

        "morph" => {
            morph_mode(controller)?;
        }

        "runway" => {
            runway_mode(controller, color)?;
        }

        _ => {
            eprintln!("Unknown mode: {}", mode);
        }
    }

    Ok(())
}
