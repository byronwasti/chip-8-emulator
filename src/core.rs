use rand;
use std::{thread, time};
use std::time::Duration;

use opcode::{OpCode, Instruction, Register};
use peripherals::{Chip8Disp, Chip8Input, PixelData, Chip8Key};

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

#[derive(Debug)]
struct Registers {
    registers: [u8; 16],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; 16],
        }
    }

    pub fn get(&self, reg: Register) -> u8 {
        self.registers[reg as usize]
    }

    pub fn set(&mut self, reg: Register, value: u8) {
        self.registers[reg as usize] = value;
    }

    pub fn set_0xf(&mut self, value: u8) {
        self.registers[0xF] = value;
    }
}

pub struct Chip8<T: Chip8Disp, U: Chip8Input>  {
    memory: [u8; 4096],
    registers: Registers,
    pc: u16,
    index: u16,
    stack: [u16; 16],
    stack_ptr: u8,

    // Timers
    delay_timer: u8,
    sound_timer: u8,

    // Peripherals
    screen: Option<T>,
    keyboard: Option<U>,
}

impl<T, U> Chip8<T, U>  
    where T: Chip8Disp, U: Chip8Input {
    pub fn new() -> Chip8<T, U> {
        let mut memory = [0; 4096];
        populate_builtin_sprites(&mut memory);

        Chip8 {
            memory: memory,
            registers: Registers::new(),
            pc: 0x200,
            index: 0,
            stack: [0; 16],
            stack_ptr: 0,

            delay_timer: 0,
            sound_timer: 0,

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
            error!("Invalid program length");
            return Err("Invalid program length!".to_string());
        }

        self.memory[0x200..(0x200 + program.len())].copy_from_slice(program);

        Ok(())
    }
    
    fn cycle_once(&mut self) -> bool {
        // Poll keyboard to allow it to update inputs
        if let Some(ref mut keyboard) = self.keyboard {
            let quit = keyboard.poll();
            if quit {
                info!("Keyboard quit");
                return quit;
            }
        }

        // Convert raw assembly at pc into parsed Opcode
        let bytes: [u8; 2] = [ self.memory[self.pc as usize],
                               self.memory[(self.pc + 1) as usize] ];
        let opcode = OpCode::new(&bytes);

        // Ignore error instructions for now
        if let Ok(instruction) = opcode.to_instruction() {
            debug!("pc: {}, instruction: {:?}", self.pc, instruction);
            self.handle_instruction(instruction);
            debug!("Registers:
                   reg: {:?}, index: {},
                   stack: {:?}, stack_ptr: {},
                   delay: {}, sound: {}\n", 
                   self.registers, self.index, 
                   self.stack, self.stack_ptr, 
                   self.delay_timer, self.sound_timer);
        } else {
            warn!("Invalid instruction: {:?}", opcode);
        }

        false
    }

    pub fn run(&mut self) {
        let rate = Duration::from_millis(2); // 1/s

        let timer_rate = Duration::from_millis(17);
        let mut timers_time = time::Instant::now();

        loop {
            let now = time::Instant::now();

            let quit = self.cycle_once();
            if quit {
                break;
            }

            if timers_time.elapsed() > timer_rate {
                timers_time = time::Instant::now();
                // Decrement timers
                if self.sound_timer > 0 {
                    self.sound_timer -= 1;
                }

                if self.delay_timer > 0 {
                    self.delay_timer -= 1;
                }
            }

            let elapsed = now.elapsed();
            if elapsed < rate {
                debug!("Slept CPU");
                thread::sleep(rate - elapsed);
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

                self.pc += 2;
            }
            Instruction::Return => {
                self.stack_ptr -= 1;
                self.pc = self.stack[self.stack_ptr as usize] + 2;
                self.stack[self.stack_ptr as usize] = 0;
            }
            Instruction::Jump(addr) => self.pc = addr,
            Instruction::Call(addr) => {
                // Set stack to save current location
                self.stack[self.stack_ptr as usize] = self.pc;
                self.stack_ptr += 1;
                self.pc = addr;
            }
            Instruction::SkipEqI(reg, byte) => {
                if self.registers.get(reg) == byte {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::SkipNeqI(reg, byte) => {
                if self.registers.get(reg) != byte { 
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::SkipEq(regx, regy) => {
                if self.registers.get(regx) == self.registers.get(regy) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::LoadI(reg, byte) => {
                self.registers.set(reg, byte);
                self.pc += 2;
            }
            Instruction::AddI(reg, byte) => {
                let val = self.registers.get(reg).wrapping_add(byte);
                self.registers.set(reg, val);
                self.pc += 2;
            }
            Instruction::LoadR(regx, regy) => {
                let val = self.registers.get(regy);
                self.registers.set(regx, val);
                self.pc += 2;
            }
            Instruction::Or(regx, regy) => {
                let val = self.registers.get(regx) | self.registers.get(regy);
                self.registers.set(regx, val);
                self.pc += 2;
            }
            Instruction::And(regx, regy) => {
                let val = self.registers.get(regx) & self.registers.get(regy);
                self.registers.set(regx, val);
                self.pc += 2;
            }
            Instruction::Xor(regx, regy) => {
                let val = self.registers.get(regx) ^ self.registers.get(regy);
                self.registers.set(regx, val);
                self.pc += 2;
            }
            Instruction::Add(regx, regy) => {
                let x_val = self.registers.get(regx);
                let y_val = self.registers.get(regy);

                let mut temp = (x_val as u16) + (y_val as u16);
                if temp > 0xFF {
                    self.registers.set_0xf(1);
                    temp = temp & 0xFF;
                }

                self.registers.set(regx, temp as u8);
                self.pc += 2;
            }
            Instruction::Sub(regx, regy) => {
                let x_val = self.registers.get(regx);
                let y_val = self.registers.get(regy);

                if x_val > y_val {
                    self.registers.set_0xf(1);
                } else {
                    self.registers.set_0xf(0);
                }
    
                // Apply subtraction
                let result = x_val.wrapping_sub(y_val);
                self.registers.set(regx, result);
                self.pc += 2;
            }
            Instruction::ShiftR(reg) => {
                let high_1 = self.registers.get(reg) & 0b1;
                self.registers.set_0xf(high_1);

                let val = self.registers.get(reg) >> 1;
                self.registers.set(reg, val);
                self.pc += 2;
            }
            Instruction::SubN(regx, regy) => {
                let x_val = self.registers.get(regx);
                let y_val = self.registers.get(regy);

                if y_val > x_val {
                    self.registers.set_0xf(1);
                } else {
                    self.registers.set_0xf(0);
                }
    
                // Apply subtraction
                let result = y_val.wrapping_sub(x_val);
                self.registers.set(regx, result);
                self.pc += 2;
            }
            Instruction::ShiftL(reg) => {
                let val = self.registers.get(reg) >> 7;
                self.registers.set_0xf(val);

                let val = self.registers.get(reg) << 1;
                self.registers.set(reg, val);
                self.pc += 2;
            }
            Instruction::SkipNeq(regx, regy) => {
                if self.registers.get(regx) != self.registers.get(regy) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::LoadIdx(addr) => {
                self.index = addr;
                self.pc += 2;
            }
            Instruction::JumpAddV0(addr) => {
                self.pc = addr + (self.registers.get(0x0) as u16);
            }
            Instruction::Rand(reg, byte) => {
                let val = rand::random::<u8>() & byte;
                self.registers.set(reg, val);
                self.pc += 2;
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

                    for (bit_pos, bit) in bits.iter().rev().enumerate() {
                        // Get positions
                        let x_pos = self.registers.get(regx) + (bit_pos as u8);
                        let y_pos = self.registers.get(regy) + (idx as u8);

                        // Get value
                        let val = *bit == 1;

                        let pixel = PixelData{ x: x_pos as usize, 
                                               y: y_pos as usize, 
                                               val: val };
                        
                        pixel_data.push(pixel);
                    }
                }

                if let Some(ref mut screen) = self.screen {
                    let collision = screen.set_pixel_data(&pixel_data[..]);
                    if collision {
                        self.registers.set_0xf(1);
                    } else {
                        self.registers.set_0xf(0);
                    }

                    screen.draw();
                }

                self.pc += 2;
            }
            Instruction::SkipEqKey(reg) => {
                let mut skip = false;
                if let Ok(key) = Chip8Key::new(self.registers.get(reg)) {
                    if let Some(ref mut keyboard) = self.keyboard {
                        skip = keyboard.key_pressed(key);
                    }
                }

                if skip {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::SkipNeqKey(reg) => {
                let mut skip = false;
                if let Ok(key) = Chip8Key::new(self.registers.get(reg)) {
                    if let Some(ref mut keyboard) = self.keyboard {
                        skip = !keyboard.key_pressed(key);
                    }
                }

                if skip {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::LoadFromDT(reg) => {
                let val = self.delay_timer;
                self.registers.set(reg, val);
                self.pc += 2;
            }
            Instruction::LoadKey(reg) => {
                if let Some(ref mut keyboard) = self.keyboard {
                    if let Some(key_pressed) = keyboard.last_key_pressed() {
                        self.registers.set(reg, key_pressed as u8);
                        self.pc += 2;
                    }
                }
            }
            Instruction::SetDT(reg) => {
                self.delay_timer = self.registers.get(reg);
                self.pc += 2;
            }
            Instruction::SetST(reg) => {
                self.sound_timer = self.registers.get(reg);
                self.pc += 2;
            }
            Instruction::AddIdx(reg) => {
                self.index = self.index.wrapping_add(self.registers.get(reg) as u16);
                self.pc += 2;
            }
            Instruction::LoadSprite(reg) => {
                self.index = 5 * (self.registers.get(reg) as u16);
                self.pc += 2;
            }
            Instruction::LoadBCD(reg) => {
                let val = self.registers.get(reg);
                
                /*
                for idx in (0..3).rev() {
                    let val = reg % 10;
                    self.memory[(self.index + idx) as usize] = val;
                    reg = (reg - val)/10;
                }
                */

                self.memory[(self.index + 0) as usize] = val / 100;
                self.memory[(self.index + 1) as usize] = (val / 10) % 10;
                self.memory[(self.index + 2) as usize] = (val % 100) % 10;

                self.pc += 2;
            }
            Instruction::StoreRegs(reg) => {
                for idx in 0..(reg+1) {
                    self.memory[(self.index + idx as u16) as usize] = self.registers.get(idx);
                }

                self.pc += 2;
            }
            Instruction::ReadRegs(reg) => {
                for idx in 0..(reg+1) {
                    let val = self.memory[(self.index + idx as u16) as usize];
                    self.registers.set(idx, val);
                }

                self.pc += 2;
            }
        }
    }
}

