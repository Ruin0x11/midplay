use std::path::Path;
use anyhow::Result;

pub fn play_midi<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(())
}

pub fn is_midi_playing() -> bool {
    false
}

pub fn stop_midi() -> Result<()> {
    Ok(())
}
