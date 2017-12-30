extern crate chip8_emulator;
extern crate structopt;
extern crate sdl2;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate structopt_derive;

use std::io::Read;
use std::fs::File;

use structopt::StructOpt;
use chip8_emulator::core::Chip8;
use chip8_emulator::sdl2_peripherals::{Display, Keyboard};

#[derive(StructOpt, Debug)]
#[structopt(name = "fancify")]
struct Cli {
    source: String,
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("Prog Start");
    let cli = Cli::from_args();
    let mut chip8 = Chip8::new();

    // Load program from file & upload to core
    let mut file = File::open(cli.source).expect("Invalid filename");
    let mut program = Vec::new();
    file.read_to_end(&mut program).expect("Invalid file");
    chip8.upload_rom(&program).expect("Invalid program length");

    // Set up chip8 core with peripherals
    let sdl_context = sdl2::init().unwrap();
    let display = Display::new(&sdl_context);
    let keyboard = Keyboard::new(&sdl_context);
    chip8.connect_keyboard(keyboard);
    chip8.connect_display(display);

    // Run indefinitely
    info!("Run");
    chip8.run();
}

