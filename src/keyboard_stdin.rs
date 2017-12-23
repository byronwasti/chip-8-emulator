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
                    Key::Char('q') => {
                        thread_tx.send(AsyncMsg::Quit).unwrap();
                        break;
                    }
                    Key::Char(c) => {
                        use self::Chip8Key::*;
                        match c {
                            'a' => thread_tx.send(AsyncMsg::KeyInput(KeyA)).unwrap(),
                            'b' => thread_tx.send(AsyncMsg::KeyInput(KeyB)).unwrap(),
                            'c' => thread_tx.send(AsyncMsg::KeyInput(KeyC)).unwrap(),
                            'd' => thread_tx.send(AsyncMsg::KeyInput(KeyD)).unwrap(),
                            'e' => thread_tx.send(AsyncMsg::KeyInput(KeyE)).unwrap(),
                            'f' => thread_tx.send(AsyncMsg::KeyInput(KeyF)).unwrap(),
                            '0' => thread_tx.send(AsyncMsg::KeyInput(Key0)).unwrap(),
                            '1' => thread_tx.send(AsyncMsg::KeyInput(Key1)).unwrap(),
                            '2' => thread_tx.send(AsyncMsg::KeyInput(Key2)).unwrap(),
                            '3' => thread_tx.send(AsyncMsg::KeyInput(Key3)).unwrap(),
                            '4' => thread_tx.send(AsyncMsg::KeyInput(Key4)).unwrap(),
                            '5' => thread_tx.send(AsyncMsg::KeyInput(Key5)).unwrap(),
                            '6' => thread_tx.send(AsyncMsg::KeyInput(Key6)).unwrap(),
                            '7' => thread_tx.send(AsyncMsg::KeyInput(Key7)).unwrap(),
                            '8' => thread_tx.send(AsyncMsg::KeyInput(Key8)).unwrap(),
                            '9' => thread_tx.send(AsyncMsg::KeyInput(Key9)).unwrap(),
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
        
        self.key_pressed = None;

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

