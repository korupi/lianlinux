use std::sync::Mutex;

use colored::Colorize;
use hidapi::{HidApi, HidDevice};
use lazy_static::lazy_static;

use crate::core::devices::a100;

pub mod devices;
pub mod modes;

/// # Lian Li vendor ID
pub const LIANLI_VENDOR_ID: u16 = 0x0CF2;

lazy_static! {
    /// # Initialize global HID device list
    pub static ref DEVICE_LIST: Mutex<Vec<HidDevice>> = Mutex::new(vec![]);
}

/// # Initialize core module
///
/// Searches for Lian Li controllers in system
pub fn init() {
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(e) => {
            eprintln!("{} initializing HidApi: {}", "Error".red(), e);
            return;
        }
    };

    for device_info in api.device_list() {
        if device_info.vendor_id() != LIANLI_VENDOR_ID {
            continue;
        }

        handle_lianli_device(device_info);
    }
}

fn handle_lianli_device(device_info: &hidapi::DeviceInfo) {
    match device_info.product_id() {
        0xA100 => {
            a100::init(HidApi::new().unwrap()).expect("uh");
        }
        _ => {
            println!(
                "{} controller found (PID: {:04X})",
                "Unsupported".red(),
                device_info.product_id()
            );
        }
    }
}
