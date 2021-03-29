use std::path::Path;

use std::fs;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use lazy_static::lazy_static;
use anyhow::{anyhow, Result};

use midir::{MidiOutput, MidiOutputConnection};
use midly::Smf;
use log::*;

mod midi_container;
mod time_controller;

use self::midi_container::{MidiContainer, MidiTimedEvent};
use self::time_controller::{TimeController, TimeListener, TimeListenerTrait};

enum MidiThreadMsg {
    Stop
}

#[derive(Clone, Copy)]
enum MidiState {
    Stopped,
    StartPlaying(i64),
    Playing,
    EOF
}

struct MidiWorker<'m> {
    conn_out: MidiOutputConnection,
    receiver: Arc<Mutex<Receiver<MidiThreadMsg>>>,
    time_control: TimeController,
    state: MidiState,
    events: Vec<MidiTimedEvent<'m>>,
    idx: usize,
    looping: bool
}

struct MidiThread {
    handle: JoinHandle<()>,
    sender: Sender<MidiThreadMsg>,
    time_listener: TimeListener,
}

fn midly_to_raw<'m>(kind: &'m midly::TrackEventKind<'m>) -> Option<Vec<u8>> {
    match kind.as_live_event() {
        Some(event) => {
            let mut vec = Vec::new();
            event.write(&mut vec);
            Some(vec)
        },
        None => None
    }
}

impl<'m> MidiWorker<'m> {
    fn start_playing(&mut self, pos_us: i64) -> MidiState {
        self.idx = 0;
        self.time_control.set_pos_us(pos_us as i64);
        while self.idx < self.events.len() && pos_us >= self.events[self.idx].0 as i64 {
            self.idx += 1;
        }
        if self.idx >= self.events.len() {
            self.time_control.stop();
            MidiState::EOF
        } else {
            self.time_control.start();
            MidiState::Playing
        }
    }

    fn tick(&mut self) -> MidiState {
        let pos_us = self.time_control.get_pos_us();
        // if let Some(ref mut conn_out) = self.conn_out.as_mut() {
        while self.idx < self.events.len() && pos_us >= self.events[self.idx].0 as i64 {
            if let Some(bytes) = midly_to_raw(self.events[self.idx].2) {
                self.conn_out.send(&bytes).unwrap();
            }
            self.idx += 1;
        }
        // }
        if self.idx >= self.events.len() {
            self.time_control.stop();
            MidiState::EOF
        } else {
            let next_pos = self.events[self.idx].0 as i64;
            let opt_sleep_ms = self.time_control.ms_till_pos(next_pos);
            if let Some(sleep_ms) = opt_sleep_ms {
                let sleep_ms = sleep_ms.min(20);
                trace!("sleep {} ms", sleep_ms);
                thread::sleep(Duration::from_millis(sleep_ms as u64));
            }
            MidiState::Playing
        }
    }

    fn service(&mut self) {
        let new_state = match self.state {
            MidiState::Stopped => MidiState::Stopped,
            MidiState::EOF => {
                if self.looping {
                    MidiState::StartPlaying(0)
                } else {
                    MidiState::EOF
                }
            }
            MidiState::StartPlaying(pos_us) => {
                self.start_playing(pos_us)
            }
            MidiState::Playing => {
                self.tick()
            }
        };
        self.state = new_state
    }

    fn work(&mut self) {
        self.state = MidiState::StartPlaying(0);

        loop {
            match self.receiver.lock().unwrap().try_recv() {
                Ok(MidiThreadMsg::Stop) => break,
                Err(_) => ()
            }

            self.service();
        }
    }
}

lazy_static! {
    static ref THREAD: Mutex<RefCell<Option<MidiThread>>> = Mutex::new(RefCell::new(None));
}

#[derive(Clone, Debug)]
pub struct MidiPortName {
    pub index: usize,
    pub name: String
}

pub fn get_ports() -> Result<Vec<MidiPortName>>{
    let midi_out = MidiOutput::new("midplay")?;

    let out_ports = midi_out.ports();
    Ok(out_ports.iter()
             .enumerate()
             .map(|(i, p)| MidiPortName { index: i, name: midi_out.port_name(p).unwrap() })
             .collect())
}

pub fn play_midi<P: AsRef<Path>>(path: P, port: &MidiPortName) -> Result<()> {
    if is_midi_playing() {
        stop_midi()?
    }

    let midi_out = MidiOutput::new("midplay")?;
    let out_ports = midi_out.ports();
    let out_port = out_ports.get(port.index).ok_or_else(|| anyhow!("Invalid port {}", port.index))?;
    let conn_out = midi_out.connect(out_port, "midir-test").map_err(|_| anyhow!("Failed to open MIDI output"))?;

    let (tx, rx) = mpsc::channel::<MidiThreadMsg>();

    let time_controller = TimeController::new();
    let time_listener = time_controller.new_listener();

    let path_str = String::from(path.as_ref().to_str().unwrap());

    let handle = thread::spawn(move|| {
        let bytes = fs::read(path_str).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let container = MidiContainer::from_buf(&smf).unwrap();
        let events = container
            .iter()
            .timed(&container.header().timing)
            .collect::<Vec<_>>();

        let mut worker = MidiWorker {
            conn_out: conn_out,
            receiver: Arc::new(Mutex::new(rx)),
            time_control: time_controller,
            state: MidiState::Stopped,
            events: events,
            idx: 0,
            looping: true
        };

        worker.work();
    });

    let thread = MidiThread {
        handle: handle,
        sender: tx,
        time_listener: time_listener
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
