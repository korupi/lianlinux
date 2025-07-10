use std::vec;

use super::devices::{a100::{set_mode, set_rgb_mode}, Controller};

pub fn static_mode(controller: &Controller, color: &[u8]) {
    controller.apply_mode(0x01, Some(color), 0x250);
}

pub fn breathing_mode(controller: &Controller, color: &[u8]) {
    controller.apply_mode(0x02, Some(color), 0x500);
}

pub fn rainbow_mode(controller: &Controller) {
    controller.apply_mode(0x05, None, 0x250);
}

pub fn morph_mode(controller: &Controller) {
    controller.apply_mode(0x04, None, 0x250);
}

pub fn runway_mode(controller: &Controller, color: &[u8]) {
    if color.len() < 6 {
        eprintln!("Invalid color slice for runway_mode");
        return;
    }

    let reordered = [
        color[0], color[2], color[1], color[3], color[5], color[4]
    ];
    controller.apply_mode(0x1c, Some(&reordered), 0x250);
}
