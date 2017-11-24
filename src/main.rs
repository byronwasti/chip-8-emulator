extern crate tui;
extern crate termion;
extern crate chip8_emulator;

use std::io::Read;
use std::{thread, time};
use std::time::Duration;
use std::sync::mpsc;
use chip8_emulator::core::Chip8;

mod display;
use display::Display;

fn main() {
    let mut display = Display::new();
    let mut chip8 = Chip8::new(&[0x63, 0x03, 0x64, 0x00, 0xF3, 0x29, 0xD3, 0x45]).unwrap();

    let rate = Duration::new(0, 500); // 1/s

    //let (tx, rx) = mpsc::channel();

    //thread::spawn(move || {
        loop {
            let input: Option<u32> = std::io::stdin()
                .lock()
                .bytes()
                .last();
                
            println!("{:?}", input);
        }
    //});

    /*
    for _ in 0..4 {
        let now = time::Instant::now();

        chip8.cycle();
        let screen = chip8.get_display();
        display.draw(screen);

        if now.elapsed() < rate {
            thread::sleep(rate - now.elapsed());
        }
    }
    */
}
