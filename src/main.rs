extern crate tui;
extern crate termion;
extern crate chip8_emulator;

use chip8_emulator::core::Chip8;

mod display;

fn main() {
    //display::playground();
    let mut chip8 = Chip8::new(&[0x63, 0x03, 0x64, 0x00, 0xF3, 0x29, 0xD3, 0x45]).unwrap();

    for _ in 0..4 {
        chip8.cycle();
        let display = chip8.get_display();

        for line in display.iter() {
            for val in line.iter() {
                match *val {
                    true => print!("X"),
                    false => print!("_"),
                }
            }
            println!("");
        }
    }
}
