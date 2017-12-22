extern crate chip8_emulator;
extern crate structopt;

#[macro_use]
extern crate structopt_derive;

use std::io::Read;
use std::{thread, time};
use std::time::Duration;
use std::sync::mpsc;
use std::fs::File;

use structopt::StructOpt;
use chip8_emulator::core::Chip8;
use chip8_emulator::display_tui::TuiDisplay;
use chip8_emulator::keyboard_stdin::KeyboardStdin;

#[derive(StructOpt, Debug)]
#[structopt(name = "fancify")]
struct Cli {
    source: String,
}

fn main() {

    let cli = Cli::from_args();
    println!("{}", cli.source);

    let mut file = File::open(cli.source).expect("Invalid filename");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Invalid file");

    println!("{:?}", contents);

    let program = [0x63, 0x03, 0x64, 0x00, 0xF3, 0x29, 0xD3, 0x45];

    let mut chip8: Chip8<TuiDisplay, KeyboardStdin> = Chip8::new();

    let rate = Duration::new(0, 500); // 1/s

    //let (tx, rx) = mpsc::channel();

    /* 
    thread::spawn(move || {
        loop {
            let input: Option<u32> = std::io::stdin()
                .lock()
                .bytes()
                .last();
                
            println!("{:?}", input);
        }
    });
    */

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
