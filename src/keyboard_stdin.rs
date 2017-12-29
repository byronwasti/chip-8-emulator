use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io;
use std::thread;
use std::sync::mpsc;

use peripherals::{Chip8Input, Chip8Key};

pub struct KeyboardStdin {
    key_pressed: Option<Chip8Key>,
    rx_channel: mpsc::Receiver<AsyncMsg>,
}

enum AsyncMsg {
    KeyInput(Chip8Key),
    Quit,
}

impl KeyboardStdin {
    pub fn new() -> KeyboardStdin {
        let (thread_tx, main_rx) = mpsc::channel();

        KeyboardStdin::spin_thread(thread_tx);

        KeyboardStdin {
            key_pressed: None,
            rx_channel: main_rx,
        }
    }

    fn spin_thread(thread_tx: mpsc::Sender<AsyncMsg>) {
        thread::spawn(move || {
            let stdin = io::stdin();
            let _stdout = io::stdout().into_raw_mode().unwrap();

            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('p') => {
                        thread_tx.send(AsyncMsg::Quit).unwrap();
                        break;
                    }
                    Key::Char(c) => {
                        use self::Chip8Key::*;
                        match c {
                            '1' => thread_tx.send(AsyncMsg::KeyInput(Key1)).unwrap(),
                            '2' => thread_tx.send(AsyncMsg::KeyInput(Key2)).unwrap(),
                            '3' => thread_tx.send(AsyncMsg::KeyInput(Key3)).unwrap(),
                            'q' => thread_tx.send(AsyncMsg::KeyInput(Key4)).unwrap(),
                            'w' => thread_tx.send(AsyncMsg::KeyInput(Key5)).unwrap(),
                            'e' => thread_tx.send(AsyncMsg::KeyInput(Key6)).unwrap(),
                            'a' => thread_tx.send(AsyncMsg::KeyInput(Key7)).unwrap(),
                            's' => thread_tx.send(AsyncMsg::KeyInput(Key8)).unwrap(),
                            'd' => thread_tx.send(AsyncMsg::KeyInput(Key9)).unwrap(),
                            'x' => thread_tx.send(AsyncMsg::KeyInput(Key0)).unwrap(),
                            'z' => thread_tx.send(AsyncMsg::KeyInput(KeyA)).unwrap(),
                            'c' => thread_tx.send(AsyncMsg::KeyInput(KeyB)).unwrap(),
                            '4' => thread_tx.send(AsyncMsg::KeyInput(KeyC)).unwrap(),
                            'r' => thread_tx.send(AsyncMsg::KeyInput(KeyD)).unwrap(),
                            'f' => thread_tx.send(AsyncMsg::KeyInput(KeyE)).unwrap(),
                            'v' => thread_tx.send(AsyncMsg::KeyInput(KeyF)).unwrap(),
                            'm' => debug!("Manual Debug Marker"),
                            _ => {},
                        }
                    }
                    _ => thread_tx.send(AsyncMsg::KeyInput(Chip8Key::KeyA)).unwrap(),
                }
            }
        });
    }
}

impl Chip8Input for KeyboardStdin {
    fn key_pressed(&self) -> Option<Chip8Key> {
        self.key_pressed
    }

    fn poll(&mut self) -> bool {
        // Function returns true if it is time to quit
        
        //self.key_pressed = None;

        while let Ok(value) = self.rx_channel.try_recv() {
            match value {
                AsyncMsg::Quit => return true,
                AsyncMsg::KeyInput(key) => {
                    self.key_pressed = Some(key);
                }
            }
        }

        false
    }
}

