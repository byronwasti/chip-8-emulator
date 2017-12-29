use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

pub struct Display {
    display: [[bool; 64]; 32],
}

impl Display {
    pub fn new() -> Display {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chip8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();

        Display {
            display: [[false; 64]; 32],
        }
    }
}

impl Chip8Disp for Display {
    fn set_pixel_data(&mut self, data: &[PixelData]) -> bool {
        let mut collision = false;

        collision
    }
    
    fn draw(&mut self) {
    }

    fn clear(&mut self) {
    }
}

pub struct Keyboard {
    key_pressed: Option<Chip8Key>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_pressed: None,
        }
    }
}

impl Chip8Input for Keyboard {
    fn key_pressed(&self) -> Option<Chip8Key> {
        key_pressed
    }

    fn poll(&mut self) -> bool {
    }
}

