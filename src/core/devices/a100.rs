use hidapi::HidApi;

use crate::core::{devices::Controller, LIANLI_VENDOR_ID};

const PRODUCT_ID: u16 = 0xa100;

pub const REPORT_BYTE: u8 = 0xE0;

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
