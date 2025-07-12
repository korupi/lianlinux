use std::sync::{Arc, Mutex};

use anyhow::Context;
use config::Config;
use nix::unistd::Uid;
use serde::Deserialize;

use crate::{core::devices::Controller, daemon, utils::parse_color};

#[derive(Deserialize, Debug, Clone)]
pub struct DaemonConfig {
    pub mode: String,
    pub colors: Vec<String>,
}

pub fn init(controller: Arc<Mutex<Controller>>) -> anyhow::Result<DaemonConfig> {
    let config_path = if Uid::effective().is_root() {
        "/etc/lianlinux.toml "
    } else {
        &(std::env::var("HOME").unwrap() + "/.config/lianlinux.toml")
    };

    let settings = Config::builder()
        .add_source(config::File::with_name(config_path))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .context("Failed to load config")?
        .try_deserialize::<DaemonConfig>()
        .unwrap();

    handle_config(controller, &settings)?;

    Ok(settings)
}

fn handle_config(controller: Arc<Mutex<Controller>>, config: &DaemonConfig) -> anyhow::Result<()> {
    let color_bytes = if !config.colors.is_empty() {
        // Parse all colors into a single Vec<u8>
        parse_color(&config.colors).context("Failed to parse colors from config")?
    } else {
        // No colors provided
        vec![]
    };

    daemon::set_mode(controller, &config.mode, &color_bytes)?;
    Ok(())
}
