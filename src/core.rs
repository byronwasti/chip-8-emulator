use rand;
use bit_range::BitRange;

use opcode::{OpCode, Instruction};
use keyboard::KeyCode;

pub struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    pc: u16,
    index: u16,
    stack: [u16; 16],
    stack_ptr: u8,
    delay_timer: u8,
    sound_timer: u8,

    // Internal flag
    no_increment: bool,

    // External interactions
    clear_flag: bool,
    gfx: [[bool; 64]; 32],
    key_code: Option<KeyCode>,
}

impl Chip8 {
    pub fn new( program: &[u8] ) -> Result<Chip8, String> {
        if program.len() > (4096 - 512) {
            return Err("Invalid program length!".to_string());
        }

        let mut memory: [u8; 4096] = [0; 4096];
        for (i, byte) in program.iter().enumerate() {
            memory[i + 0x200] = *byte;
        }
        Chip8::populate_builtin_sprites(&mut memory);


        Ok(Chip8 {
            memory: memory,
            registers: [0; 16],
            pc: 0x200,
            index: 0,
            stack: [0; 16],
            stack_ptr: 0,
            delay_timer: 0,
            sound_timer: 0,

            no_increment: false,

            clear_flag: false,
            gfx: [[false; 64]; 32], // Row-major order
            key_code: None,
        })
    }

    pub fn get_clear_flag(&self) -> bool {
        self.clear_flag
    }

    pub fn unset_clear_flag(&mut self) {
        self.clear_flag = false;
    }

    pub fn get_display(&self) -> &[[bool; 64]; 32] {
        &self.gfx
    }

    pub fn set_key_code(&mut self, key_code: Option<KeyCode>) {
        self.key_code = key_code;
    }

    pub fn cycle(&mut self) {
        let bytes: [u8; 2] = [ self.memory[self.pc as usize],
                               self.memory[(self.pc + 1) as usize] ];
        let opcode = OpCode::new(&bytes);

        // Ignore error instructions for now
        if let Ok(instruction) = opcode.to_instruction() {
            self.handle_instruction(instruction);
        }

        // Increment our program counter
        if !self.no_increment {
            self.pc += 2;
        }
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

    fn handle_instruction(&mut self, instruction: Instruction) {
        // TODO: Is there a better way to structure this?
        match instruction {
            Instruction::SYS(_) => return,
            Instruction::Clear => self.clear_flag = true,
            Instruction::Return => {
                self.stack_ptr -= 1;
                self.pc = self.stack[self.stack_ptr as usize];
            }
            Instruction::Jump(addr) => self.pc = addr,
            Instruction::Call(addr) => {
                self.stack[self.stack_ptr as usize] = self.pc;
                self.pc = addr;
                self.stack_ptr += 1;
            }
            Instruction::SkipEqI(reg, byte) => if self.registers[reg as usize] == byte {
                self.pc += 2
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
            Instruction::JumpAddV0(addr) => self.pc = addr + self.registers[0x0] as u16,
            Instruction::Rand(reg, byte) => {
                self.registers[reg as usize] = rand::random::<u8>() & byte;
            }
            Instruction::Draw(regx, regy, nib) => {
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
                        let x_pos = (regx as usize) + (bit_pos as usize);
                        let y_pos = (regy as usize) + (idx as usize);

                        // Get value
                        let val = (*bit == 1);
                        
                        // XOR manually
                        if self.gfx[y_pos][x_pos] == val && val == true {
                            self.registers[15] = 1;
                            self.gfx[y_pos][x_pos] = !val;
                        } else {
                            self.gfx[y_pos][x_pos] = val;
                        }
                    }
                }


            }
            Instruction::SkipEqKey(reg) => {
                if let Some(key_code) = self.key_code {
                    if key_code as u8 == self.registers[reg as usize] {
                        self.pc += 2;
                    }
                }
            }
            Instruction::SkipNeqKey(reg) => {
                if let Some(key_code) = self.key_code {
                    if key_code as u8 != self.registers[reg as usize] {
                        self.pc += 2;
                    }
                }
            }
            Instruction::LoadFromDT(reg) => self.registers[reg as usize] = self.delay_timer,
            Instruction::LoadKey(reg) => {
                if let Some(key_code) = self.key_code {
                    self.no_increment = false;
                    self.registers[reg as usize] = key_code as u8;
                } else {
                    self.no_increment = true;
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

