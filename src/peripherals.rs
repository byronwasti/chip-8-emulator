pub struct PixelData { pub x: usize, pub y: usize, pub val: bool }

pub trait Chip8Disp {
    fn set_pixel_data(&mut self, data: &[PixelData]) -> bool;
    fn draw(&mut self);
    fn clear(&mut self);
}

pub trait Chip8Input {
    fn key_pressed(&self) -> Option<Chip8Key>;
    fn poll(&mut self) -> bool;
}

#[derive(Copy, Clone, PartialEq)]
pub enum Chip8Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl Chip8Key {
    pub fn new(value: u8) -> Result<Chip8Key, String>  {
        use self::Chip8Key::*;
        match value {
            0 => Ok(Key0),
            1 => Ok(Key1),
            2 => Ok(Key2),
            3 => Ok(Key3),
            4 => Ok(Key4),
            5 => Ok(Key5),
            6 => Ok(Key6),
            7 => Ok(Key7),
            8 => Ok(Key8),
            9 => Ok(Key9),
            10 => Ok(KeyA),
            11 => Ok(KeyB),
            12 => Ok(KeyC),
            13 => Ok(KeyD),
            14 => Ok(KeyE),
            15 => Ok(KeyF),
            _ => Err("Invalid Key token".to_string()),
        }
    }
}

