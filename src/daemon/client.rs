use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use crate::daemon::protocol::{Request, Response};

use anyhow::Context;

pub fn send_message(request: Request) -> anyhow::Result<()> {
    let mut unix_stream =
        UnixStream::connect("/tmp/lianlinux.socket").context("Could not create stream")?;

    unix_stream
        .write(&serde_json::to_string(&request).unwrap().into_bytes())
        .context("Failed at writing onto the unix stream")?;

    unix_stream
        .shutdown(std::net::Shutdown::Write)
        .context("Could not shutdown writing on the stream")?;

    let _ =read_from_stream(&mut unix_stream);

    Ok(())
}

fn read_from_stream(unix_stream: &mut UnixStream) -> anyhow::Result<()> {
    let mut response = String::new();
    unix_stream
        .read_to_string(&mut response)
        .context("Failed at reading the unix stream")?;

    println!("{}", response);
    Ok(())
}
