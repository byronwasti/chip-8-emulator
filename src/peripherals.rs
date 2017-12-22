
pub trait Chip8Disp {
    fn set_pixel_data(&mut self);
    fn draw(&mut self);
}

pub trait Chip8Input {
    fn is_key_pressed(&self, &Chip8Key) -> bool;
}

pub enum Chip8Key {
    k0,
    k1,
    k2,
    k3,
    k4,
    k5,
    k6,
    k7,
    k8,
    k9,
    kA,
    kB,
    kC,
    kD,
    kE,
    kF,
}

