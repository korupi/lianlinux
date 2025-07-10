use std::vec;
use hidapi::{HidApi, HidDevice};

use crate::core::{devices::Controller, modes, DEVICE_LIST, LIANLI_VENDOR_ID};

const PRODUCT_ID: u16 = 0xa100;

pub const REPORT_BYTE: u8 = 0xE0;

/// # Set preset controller mode
/// 
/// The Lian Li controller typically has 4 ports.
/// We need to iterate through them and set their modes.
/// Preset modes is stored in the controller itself.
pub fn set_mode(color: &[u8], device: &HidDevice) {
    for i in 0u8..8u8 {
        let mut packet: Vec<u8> = vec![REPORT_BYTE, 0x10 + i];

        packet.append(&mut color.to_vec());

        let _ = device.write(&packet[..]);
    }
}

/// # Set custom colors controller mode
///
/// The Lian Li controller typically has 4 ports.
/// We need to iterate through them and set their modes.
pub fn set_rgb_mode(color: &[u8], mode: u8, device: &HidDevice) {
    for i in 0u8..8u8 { 
        let mode_packet: Vec<u8> = vec![REPORT_BYTE, 0x10 + i, mode];
        let mut rgb: Vec<u8> = vec![];

        rgb.append(&mut color.repeat(0x400));

        let mut packet: Vec<u8> = vec![REPORT_BYTE, 0x30 + i];
        packet.append(&mut rgb);
        if let Err(e) = device.write(&mode_packet) {
            eprintln!("Failed to write packet: {:?}", e);
        }
        if let Err(e) = device.write(&packet) {
            eprintln!("Failed to write packet: {:?}", e);
        }
    }
}


/// # Initialize controller driver
///
/// Test if a controller handles write operations ok
///
pub fn init(api: HidApi) -> anyhow::Result<Controller> {
    //let mut devices_list = DEVICE_LIST.lock().unwrap();
    let device = api.open(LIANLI_VENDOR_ID, PRODUCT_ID)?;

    device.write(&[REPORT_BYTE, 0, 0, 0, 0])?; // Test write

    let controller = Controller::new(device, REPORT_BYTE); // clone if needed
    match controller.device.get_product_string() {
        Ok(Some(name)) => println!("Found a controller: {}", name),
        Ok(None) => println!("Found a controller with no product string."),
        Err(e) => println!("Found a controller, but failed to get product string: {}", e),
    }

    Ok(controller)
}
