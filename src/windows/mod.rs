mod wide;
mod mci;
mod utf;

pub fn play_midi(filename: &str) {
    unsafe {
        let s = format!("open {} alias music", filename);
        mci::mci_command(&s);
        mci::mci_command("play music");
    }
}

pub fn stop_midi() {
    unsafe {
        mci::mci_command("stop music");
    }
}
