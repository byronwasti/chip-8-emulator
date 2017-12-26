extern crate chip8_emulator;
extern crate structopt;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate structopt_derive;

use std::io::Read;
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
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("Prog Start");
    let cli = Cli::from_args();
    let mut chip8: Chip8<TuiDisplay, KeyboardStdin> = Chip8::new();

    // Load program from file & upload to core
    let mut file = File::open(cli.source).expect("Invalid filename");
    let mut program = Vec::new();
    file.read_to_end(&mut program).expect("Invalid file");
    chip8.upload_rom(&program).expect("Invalid program length");

    // Set up chip8 core with peripherals
    let keyboard = KeyboardStdin::new();
    chip8.connect_keyboard(keyboard);

    let display = TuiDisplay::new();
    chip8.connect_display(display);

    // Run indefinitely
    info!("Run");
    chip8.run();
}

