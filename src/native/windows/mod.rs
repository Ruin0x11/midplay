use std::path::Path;
use anyhow::{anyhow, Result};

mod wide;
mod mci;
mod utf;

pub fn play_midi<P: AsRef<Path>>(path: P) -> Result<()> {
    if is_midi_playing() {
        stop_midi()?;
    }

    mci::mci_command(&format!("open {} alias midi", path.as_ref().to_str().unwrap()))?;
    mci::mci_command("play midi")?;

    Ok(())
}

pub fn is_midi_playing() -> bool {
    mci::mci_command("status midi ready").is_ok()
}

pub fn stop_midi() -> Result<()> {
    if !is_midi_playing() {
        return Ok(());
    }

    mci::mci_command("stop midi")?;
    mci::mci_command("close midi")?;

    Ok(())
}
