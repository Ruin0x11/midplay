use std::path::Path;

mod wide;
mod mci;
mod utf;

pub fn play_midi<P: AsRef<Path>>(path: P) {
    unsafe {
        let s = format!("open {} alias music", path.as_ref().as_str());
        mci::mci_command(&s);
        mci::mci_command("play music");
    }
}

pub fn stop_midi() {
    unsafe {
        mci::mci_command("stop music");
    }
}
