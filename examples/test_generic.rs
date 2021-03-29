extern crate midplay;
extern crate anyhow;

use std::io;
use std::io::prelude::*;
use midplay::generic::{self, MidiPortName};
use anyhow::{anyhow, Result};

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn get_port() -> Result<MidiPortName> {
    let mut out_ports = generic::get_ports()?;
    match out_ports.len() {
        0 => Err(anyhow!("no output port found")),
        1 => {
            println!("Choosing the only available output port: {}", out_ports[0].name);
            Ok(out_ports.swap_remove(0))
        },
        _ => {
            println!("\nAvailable output ports:");
            for p in out_ports.iter() {
                println!("{}: {}", p.index, p.name);
            }
            print!("Please select output port: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let index = input.trim().parse::<usize>()?;

            if out_ports.get(index).is_none() {
                return Err(anyhow!("invalid output port selected"));
            }

            Ok(out_ports.swap_remove(index))
        }
    }
}

fn main() -> Result<()> {
    let port = get_port()?;

    println!("Playing.");
    generic::play_midi("data/macchi.mid", &port)?;
    pause();

    println!("Stopping.");
    generic::stop_midi()?;
    pause();

    println!("Playing.");
    generic::play_midi("data/longnight2.mid", &port)?;
    pause();

    println!("Stopping.");
    generic::stop_midi()?;

    Ok(())
}
