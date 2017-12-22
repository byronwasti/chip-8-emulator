use peripherals::{Chip8Input, Chip8Key};

pub struct KeyboardStdin {
    key_pressed: Option<Chip8Key>,
}

impl KeyboardStdin {
}

impl Chip8Input for KeyboardStdin {
    fn is_key_pressed(&self, key: &Chip8Key) -> bool {
        true
    }
}


