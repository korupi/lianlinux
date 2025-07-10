use hidapi::HidDevice;

/// # Packet builder
pub fn make_packet(report_id: u8, command: u8, data: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(1 + 1 + data.len()); // REPORT_BYTE + cmd + data
    packet.push(report_id);
    packet.push(command);
    packet.extend_from_slice(data);
    packet
}

/// # Write packet to a HidDevice with all the checks
pub fn write_packet(device: &HidDevice, packet: &[u8]) -> Result<(), String> {
    device
        .write(packet)
        .map(|_| ())
        .map_err(|e| format!("Failed to write packet: {:?}", e))
}

