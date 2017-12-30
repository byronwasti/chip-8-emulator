use std::time::Duration;

use sdl2;
use sdl2::render;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use peripherals::{Chip8Disp, Chip8Input, Chip8Key, PixelData};

pub struct Display {
    data: [[bool; 64]; 32],

    canvas: render::Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chip8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();

        Display {
            data: [[false; 64]; 32],

            canvas: canvas,
        }
    }
}

impl Chip8Disp for Display {
    fn set_pixel_data(&mut self, data: &[PixelData]) -> bool {
        let mut collision = false;
        for (_, pixel) in data.iter().enumerate() {
            if pixel.x >= 64 || pixel.y >= 32 {
                warn!("Invalid pixel value: {:?}", pixel);
                continue;
            }

            if pixel.val && self.data[pixel.y][pixel.x] {
                self.data[pixel.y][pixel.x] = false;
                collision = true;
            } else if !self.data[pixel.y][pixel.x] {
                self.data[pixel.y][pixel.x] = pixel.val;
            }
        }

        collision
    }
    
    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        for (y, line) in self.data.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                if *value {
                    let x = x as i32;
                    let y = y as i32;
                    self.canvas.fill_rect(Rect::new(x*10, y*10, 10, 10)).unwrap();
                }
            }
        }
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.data = [[false; 64]; 32];
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}


pub struct Keyboard {
    last_key_pressed: Option<Chip8Key>,
    keys_pressed: [bool; 16],

    event_pump: sdl2::EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
        let event_pump = sdl_context.event_pump().unwrap();
        Keyboard { 
            last_key_pressed: None,
            keys_pressed: [false; 16],
            event_pump: event_pump,
        }
    }

    fn sdl_key_as_chip8key(sdl_key: Keycode) -> Option<Chip8Key> {
        match sdl_key {
            Keycode::Num1 => Some(Chip8Key::Key1),
            Keycode::Num2 => Some(Chip8Key::Key2),
            Keycode::Num3 => Some(Chip8Key::Key3),
            Keycode::Q => Some(Chip8Key::Key4),
            Keycode::W => Some(Chip8Key::Key5),
            Keycode::E => Some(Chip8Key::Key6),
            Keycode::A => Some(Chip8Key::Key7),
            Keycode::S => Some(Chip8Key::Key8),
            Keycode::D => Some(Chip8Key::Key9),
            Keycode::X => Some(Chip8Key::Key0),
            Keycode::Z => Some(Chip8Key::KeyA),
            Keycode::C => Some(Chip8Key::KeyB),
            Keycode::Num4 => Some(Chip8Key::KeyC),
            Keycode::R => Some(Chip8Key::KeyD),
            Keycode::F => Some(Chip8Key::KeyE),
            Keycode::V => Some(Chip8Key::KeyF),
            _ => None,
        }
    }
}

impl Chip8Input for Keyboard {
    fn last_key_pressed(&self) -> Option<Chip8Key> {
        self.last_key_pressed
    }

    fn key_pressed(&self, key: Chip8Key) -> bool {
        self.keys_pressed[key as usize]
    }

    fn poll(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    return true;
                }

                Event::KeyDown { keycode: Some(key), .. } => {
                    let chip8_key = Keyboard::sdl_key_as_chip8key(key);
                    self.last_key_pressed = chip8_key;

                    if let Some(key) = chip8_key {
                        self.keys_pressed[key as usize] = true;
                    }
                }

                Event::KeyUp { keycode: Some(key), .. } => {
                    let chip8_key = Keyboard::sdl_key_as_chip8key(key);

                    if self.last_key_pressed == chip8_key {
                        self.last_key_pressed = None;
                    }

                    if let Some(key) = chip8_key {
                        self.keys_pressed[key as usize] = false;
                    }
                }

                _ => {}
            }
        }

        false
    }
}

