use std::path::Path;

use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use lazy_static::lazy_static;
use anyhow::{anyhow, Result};

use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};

enum MidiThreadMsg {
    Stop
}

struct MidiThread {
    handle: JoinHandle<()>,
    sender: Sender<MidiThreadMsg>
}

struct MidiWorker {
    conn_out: MidiOutputConnection,
    receiver: Arc<Mutex<Receiver<MidiThreadMsg>>>
}

fn work(mut worker: MidiWorker) {
    loop {
        match worker.receiver.lock().unwrap().try_recv() {
            Ok(MidiThreadMsg::Stop) => break,
            Err(_) => ()
        }

        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            // We're ignoring errors in here
            let _ = worker.conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            thread::sleep(Duration::from_millis(duration * 150));
            let _ = worker.conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        thread::sleep(Duration::from_millis(4 * 150));

        play_note(66, 4);
        play_note(65, 3);
        play_note(63, 1);
        play_note(61, 6);
        play_note(59, 2);
        play_note(58, 4);
        play_note(56, 4);
        play_note(54, 4);
    }
}

lazy_static! {
    static ref THREAD: Mutex<RefCell<Option<MidiThread>>> = Mutex::new(RefCell::new(None));
}

pub fn play_midi<P: AsRef<Path>>(path: P) -> Result<()> {
    if is_midi_playing() {
        stop_midi()?
    }

    let (tx, rx) = mpsc::channel::<MidiThreadMsg>();

    let midi_out = MidiOutput::new("My Test Output")?;
    let out_port = &midi_out.ports()[1];
    let conn_out = midi_out.connect(out_port, "midir-test").map_err(|_| anyhow!("Failed to open MIDI output"))?;

    let worker = MidiWorker {
        conn_out: conn_out,
        receiver: Arc::new(Mutex::new(rx))
    };

    let handle = thread::spawn(move|| {
        work(worker)
    });

    let thread = MidiThread {
        handle: handle,
        sender: tx
    };

    THREAD.lock().unwrap().replace(Some(thread));

    Ok(())
}

pub fn is_midi_playing() -> bool {
    THREAD.lock().unwrap().borrow().is_some()
}

pub fn stop_midi() -> Result<()> {
    let old = THREAD.lock().unwrap().replace(None);
    if let Some(th) = old {
        th.sender.send(MidiThreadMsg::Stop)?;
        th.handle.join().map_err(|_| anyhow!("Failed to stop thread"))?;
    }

    Ok(())
}
