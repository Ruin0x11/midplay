#[cfg(windows)]
extern crate winapi;

#[cfg(target_os="windows")] mod windows;
#[cfg(target_os="windows")] use windows::*;

#[cfg(target_os="linux")] mod linux;
#[cfg(target_os="linux")] pub use linux::*;

#[cfg(target_os="macos")] mod macos;
#[cfg(target_os="macos")] pub use macos::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::prelude::*;

    fn pause() {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "Press any key to continue...").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let _ = stdin.read(&mut [0u8]).unwrap();
    }

    #[test]
    fn test_mci() {
        play_midi("gm_on.mid");
        play_midi("Operette.mid");
        pause();
        stop_midi();
    }
}

