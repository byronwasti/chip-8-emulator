use rand;
use std::{thread, time};
use std::time::Duration;
use std::sync::mpsc;

use opcode::{OpCode, Instruction};
use peripherals::{Chip8Disp, Chip8Input, PixelData, Chip8Key};

struct TimeQuantum;

pub struct Chip8<T: Chip8Disp, U: Chip8Input>  {
    memory: [u8; 4096],
    registers: [u8; 16],
    pc: u16,
    index: u16,
    stack: [u16; 16],
    stack_ptr: u8,

    // Timers
    delay_timer: u8,
    sound_timer: u8,
    rx_async_time: mpsc::Receiver<TimeQuantum>,

    // Peripherals
    screen: Option<T>,
    keyboard: Option<U>,
}

fn populate_builtin_sprites(memory: &mut [u8; 4096]) {
    memory[..(5*16)].copy_from_slice(&[
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);
}

impl<T, U> Chip8<T, U>  
    where T: Chip8Disp, U: Chip8Input {
    pub fn new() -> Chip8<T, U> {
        let mut memory = [0; 4096];
        populate_builtin_sprites(&mut memory);

        let (thread_tx, main_rx) = mpsc::channel();

        thread::spawn(move || {
            let rate = Duration::from_millis(16); // 60Hz
            loop {
                thread::sleep(rate);
                thread_tx.send(TimeQuantum).unwrap();
            }
        });

        Chip8 {
            memory: memory,
            registers: [0; 16],
            pc: 0x200,
            index: 0,
            stack: [0; 16],
            stack_ptr: 0,

            delay_timer: 0,
            sound_timer: 0,
            rx_async_time: main_rx,

            screen: None,
            keyboard: None,
        }
    }

    pub fn connect_display(&mut self, display: T) {
        self.screen = Some(display);
    }

    pub fn connect_keyboard(&mut self, keyboard: U) {
        self.keyboard = Some(keyboard);
    }

    pub fn upload_rom(&mut self, program: &[u8]) -> Result<(), String> {
        if program.len() > (4096 - 0x200) {
            return Err("Invalid program length!".to_string());
        }

        self.memory[0x200..(0x200 + program.len())].copy_from_slice(program);

        Ok(())
    }
    
    pub fn cycle_once(&mut self) -> bool {
        // Poll keyboard to allow it to update inputs
        if let Some(ref mut keyboard) = self.keyboard {
            let quit = keyboard.poll();
            if quit {
                return quit;
            }
        }

        // Convert raw assembly at pc into parsed Opcode
        let bytes: [u8; 2] = [ self.memory[self.pc as usize],
                               self.memory[(self.pc + 1) as usize] ];
        let opcode = OpCode::new(&bytes);


        // Ignore error instructions for now
        if let Ok(instruction) = opcode.to_instruction() {
            //println!("pc: {}, op: {:?}", self.pc, opcode.to_instruction());
            self.handle_instruction(instruction);
        }

        // Increment our program counter
        self.pc += 2;

        // Decrement timers
        while let Ok(_) = self.rx_async_time.try_recv() {
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
        }

        // Draw using the screen
        if let Some(ref mut screen) = self.screen {
            screen.draw();
        }

        false
    }

    pub fn run(&mut self) {
        let rate = Duration::from_millis(10); // 1/s
        loop {
            let now = time::Instant::now();

            let quit = self.cycle_once();
            if quit {
                break;
            }

            let time_now = now.elapsed();
            if time_now < rate {
                thread::sleep(rate - time_now);
            }
        }
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SYS(_) => return,
            Instruction::Clear => {
                if let Some(ref mut screen) = self.screen {
                    screen.clear();
                }
            }
            Instruction::Return => {
                self.stack_ptr -= 1;
                self.pc = self.stack[self.stack_ptr as usize];
            }
            Instruction::Jump(addr) => self.pc = addr - 2,
            Instruction::Call(addr) => {
                self.stack[self.stack_ptr as usize] = self.pc;
                self.pc = addr - 2;
                self.stack_ptr += 1;
            }
            Instruction::SkipEqI(reg, byte) => {
                if self.registers[reg as usize] == byte {
                    self.pc += 2
                }
            }
            Instruction::SkipNeqI(reg, byte) => {
                if self.registers[reg as usize] != byte { 
                    self.pc += 2
                }
            }
            Instruction::SkipEq(regx, regy) => {
                if self.registers[regx as usize] == self.registers[regy as usize] {
                    self.pc += 2
                }
            }
            Instruction::LoadI(reg, byte) => self.registers[reg as usize] = byte,
            Instruction::AddI(reg, byte) => {
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(byte);
            }
            Instruction::LoadR(regx, regy) => {
                self.registers[regx as usize] = self.registers[regy as usize];
            }
            Instruction::Or(regx, regy) => {
                self.registers[regx as usize] |= self.registers[regy as usize];
            }
            Instruction::And(regx, regy) => {
                self.registers[regx as usize] &= self.registers[regy as usize];
            }
            Instruction::Xor(regx, regy) => {
                self.registers[regx as usize] ^= self.registers[regy as usize];
            }
            Instruction::Add(regx, regy) => {
                let x_val = self.registers[regx as usize];
                let y_val = self.registers[regy as usize];

                let mut temp = (x_val as u16) + (y_val as u16);
                if temp > 0xFF {
                    self.registers[15] = 1;
                    temp = temp % 0xFF;
                }

                self.registers[regx as usize] = temp as u8;
            }
            Instruction::Sub(regx, regy) => {
                {// Set VF first 
                    let regx = self.registers[regx as usize];
                    let regy = self.registers[regy as usize];

                    if regx > regy {
                        self.registers[0xF] = 1;
                    } else {
                        self.registers[0xF] = 0;
                    }
                }
    
                // Apply subtraction
                self.registers[regx as usize] = self.registers[regx as usize]
                    .wrapping_sub(self.registers[regy as usize]);
            }
            Instruction::ShiftR(reg) => {
                self.registers[0xF] = self.registers[reg as usize] & 0b1;
                self.registers[reg as usize] >>= 1;
            }
            Instruction::SubN(regx, regy) => {
                {// Set VF first
                    let regx = self.registers[regx as usize];
                    let regy = self.registers[regy as usize];

                    if regy > regx {
                        self.registers[0xF] = 1;
                    } else {
                        self.registers[0xF] = 0;
                    }
                }

                self.registers[regx as usize] = self.registers[regy as usize]
                    .wrapping_sub(self.registers[regx as usize]);
            }
            Instruction::ShiftL(reg) => {
                self.registers[0xF] = self.registers[reg as usize] & 0b1000_0000;
                self.registers[reg as usize] <<= 1;
            }
            Instruction::SkipNeq(regx, regy) => {
                if self.registers[regx as usize] != self.registers[regy as usize] {
                    self.pc += 2
                }
            }
            Instruction::LoadIdx(addr) => self.index = addr,
            Instruction::JumpAddV0(addr) => self.pc = addr - 2 + self.registers[0x0] as u16,
            Instruction::Rand(reg, byte) => {
                self.registers[reg as usize] = rand::random::<u8>() & byte;
            }
            Instruction::Draw(regx, regy, nib) => {
                let mut pixel_data = Vec::new();
                let start = self.index as usize;
                let end = (self.index + (nib as u16)) as usize;

                // Iterate over our sprite data
                for (idx, line) in self.memory[start..end].iter().enumerate() {
                    // Get array of bits
                    let mut bits = [0u8; 8];
                    for i in 0..8 {
                        bits[i] = (line >> i) & 1;
                    }

                    for (bit_pos, bit) in bits.iter().enumerate() {
                        // Get positions
                        let x_pos = (self.registers[regx as usize]).wrapping_add((bit_pos as u8));
                        let y_pos = (self.registers[regy as usize]).wrapping_add((idx as u8));

                        // Get value
                        let val = *bit == 1;

                        let pixel = PixelData{ x: x_pos as usize, y: y_pos as usize, val: val };
                        
                        pixel_data.push(pixel);
                    }
                }

                if let Some(ref mut screen) = self.screen {
                    let collision = screen.set_pixel_data(&pixel_data[..]);
                    if collision {
                        self.registers[0xF] = 1;
                    } else {
                        self.registers[0xF] = 0;
                    }
                }
            }
            Instruction::SkipEqKey(reg) => {
                if let Ok(key) = Chip8Key::new(reg) {
                    if let Some(ref mut keyboard) = self.keyboard {
                        if let Some(key_pressed) = keyboard.key_pressed() {
                            if key == key_pressed {
                                self.pc += 2;
                            }
                        }
                    }
                }
            }
            Instruction::SkipNeqKey(reg) => {
                if let Ok(key) = Chip8Key::new(reg) {
                    if let Some(ref mut keyboard) = self.keyboard {
                        if let Some(key_pressed) = keyboard.key_pressed() {
                            if key != key_pressed {
                                self.pc += 2;
                            }
                        }
                    }
                }
            }
            Instruction::LoadFromDT(reg) => self.registers[reg as usize] = self.delay_timer,
            Instruction::LoadKey(reg) => {

                if let Some(ref mut keyboard) = self.keyboard {
                    if let Some(key_pressed) = keyboard.key_pressed() {
                        self.registers[reg as usize] = key_pressed as u8;
                    } else {
                        // Decrement PC so that we stay on this instruction
                        // until a key has been pressed.
                        self.pc -= 2;
                    }
                }
            }
            Instruction::SetDT(reg) => self.delay_timer = self.registers[reg as usize],
            Instruction::SetST(reg) => self.sound_timer = self.registers[reg as usize],
            Instruction::AddIdx(reg) => {
                self.index = self.index.wrapping_add(self.registers[reg as usize] as u16)
            }
            Instruction::LoadSprite(reg) => {
                self.index = (self.registers[reg as usize] * 5) as u16;
            }
            Instruction::LoadBCD(reg) => {
                let mut reg = self.registers[reg as usize];
                
                for idx in (0..3).rev() {
                    let val = reg % 10;
                    self.memory[(self.index + idx) as usize] = val;
                    reg = (reg - val)/10;
                }
            }
            Instruction::StoreRegs(reg) => {
                for idx in 0..reg {
                    self.memory[(self.index + idx as u16) as usize] = self.registers[idx as usize];
                }
            }
            Instruction::ReadRegs(reg) => {
                for idx in 0..reg {
                    self.registers[idx as usize] = self.memory[(self.index + idx as u16) as usize];
                }
            }
        }
    }
}

