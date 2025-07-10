use hidapi::HidDevice;

pub mod a100;

/// # Controller structure
#[derive(Debug)]
pub struct Controller {
    device: HidDevice,
    report_byte: u8
}

/// # Implements Controller abstracted functions
impl Controller {

    /// # Initialize Controller
    pub fn new(device: HidDevice, report_byte: u8) -> Self {
        Self { device, report_byte }
    }

    pub fn write_mode_packet(&self, index: u8, data: &[u8]) {
        let mut packet = vec![self.report_byte, 0x10 + index];
        packet.extend_from_slice(data);
        let _ = self.device.write(&packet);
    }

    pub fn write_rgb_packet(&self, index: u8, mode: u8, color: &[u8]) {
        let mode_packet = vec![self.report_byte, 0x10 + index, mode];
        let mut rgb_packet = vec![self.report_byte, 0x30 + index];
        rgb_packet.extend_from_slice(&color.repeat(0x400));
        let _ = self.device.write(&mode_packet);
        let _ = self.device.write(&rgb_packet);
    }

    pub fn apply_mode(&self, mode_id: u8, color: Option<&[u8]>, pad_len: usize) {
        if let Some(color) = color {
            for i in 0..8 {
                self.write_rgb_packet(i, 0x03, color);
            }
        }

        let mut packet = vec![mode_id, 0xff];
        packet.extend(vec![0x00; pad_len]);
        for i in 0..8 {
            self.write_mode_packet(i, &packet);
        }
    }
}
