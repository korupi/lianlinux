use std::sync::{Arc, Mutex};

use super::devices::Controller;

pub fn static_mode(controller: Arc<Mutex<Controller>>, color: &[u8]) -> anyhow::Result<()> {
    // Lock the mutex to get mutable access
    let ctrl = controller.lock().map_err(|e| anyhow::anyhow!("Mutex poisoned: {}", e))?;

    ctrl.apply_mode(0x01, Some(color), 0x250);

    Ok(())
}

pub fn breathing_mode(controller: Arc<Mutex<Controller>>, color: &[u8]) -> anyhow::Result<()> {
    let ctrl = &mut controller.lock().map_err(|e| anyhow::anyhow!("Mutex poisoned: {}", e))?;
    ctrl.apply_mode(0x02, Some(color), 0x500);

    Ok(())
}

pub fn rainbow_mode(controller: Arc<Mutex<Controller>>) -> anyhow::Result<()> {
    let ctrl = &mut controller.lock().map_err(|e| anyhow::anyhow!("Mutex poisoned: {}", e))?;
    ctrl.apply_mode(0x05, None, 0x250);

    Ok(())
}

pub fn morph_mode(controller: Arc<Mutex<Controller>>) -> anyhow::Result<()> {
    let ctrl = &mut controller.lock().map_err(|e| anyhow::anyhow!("Mutex poisoned: {}", e))?;
    ctrl.apply_mode(0x04, None, 0x250);

    Ok(())
}

pub fn runway_mode(controller: Arc<Mutex<Controller>>, color: &[u8]) -> anyhow::Result<()> {
    if color.len() < 6 {
        anyhow::bail!("Invalid color slice for runway_mode (expected 6 values)");
    }

    let reordered = [
        color[0], color[2], color[1], color[3], color[5], color[4]
    ];
    
    let ctrl = &mut controller.lock().map_err(|e| anyhow::anyhow!("Mutex poisoned: {}", e))?;
    ctrl.apply_mode(0x1c, Some(&reordered), 0x250);

    Ok(())
}

