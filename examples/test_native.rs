extern crate midplay;
extern crate anyhow;

use std::io;
use std::io::prelude::*;
use midplay::native;
use anyhow::Result;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> Result<()> {
    println!("Playing.");
    native::play_midi("data/macchi.mid")?;
    pause();

    println!("Stopping.");
    native::stop_midi()?;
    pause();

    println!("Playing.");
    native::play_midi("data/longnight2.mid")?;
    pause();

    println!("Stopping.");
    native::stop_midi()?;

    Ok(())
}
