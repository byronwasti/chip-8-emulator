extern crate tui;
extern crate termion;
extern crate chip8_emulator;

use chip8_emulator::core as chip8;

mod display;

fn main() {
    display::playground();
}