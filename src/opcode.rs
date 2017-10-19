type Address = u16;
type Register = u8;
type Immediate = u8;
type Nibble = u8;

pub enum Instruction {
    SYS(Address),
    Clear,
    Return,
    Jump(Address),
    Call(Address),
    SkipEqI(Register, Immediate),
    SkipNeqI(Register, Immediate),
    SkipEq(Register, Register),
    LoadI(Register, Immediate),
    AddI(Register, Immediate),
    LoadR(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    Add(Register, Register),
    Sub(Register, Register),
    ShiftR(Register),
    SubN(Register, Register),
    ShiftL(Register),
    SkipNeq(Register, Register),
    LoadIdx(Address),
    JumpAddV0(Address),
    Rand(Register, Immediate),
    Draw(Register, Register, Nibble),
    SkipEqKey(Register),
    SkipNeqKey(Register),
    LoadFromDT(Register),
    LoadKey(Register),
    SetDT(Register),
    SetST(Register),
    AddIdx(Register),
    LoadSprite(Register),
    LoadBCD(Register),
    StoreRegs(Register),
    ReadRegs(Register),
}

pub struct OpCode {
    opcode: u16,
}

impl OpCode {
    pub fn new(bytes: &[u8; 2]) -> OpCode {
        let opcode: u16 = ((bytes[0] as u16) << 8) + bytes[1] as u16;
        OpCode { opcode: opcode }
    }

    fn addr(&self) -> Address {
        self.opcode & 0x0FFF
    }

    fn x_register(&self) -> Register {
        ((self.opcode & 0x0F00) >> 8) as Register
    }

    fn y_register(&self) -> Register {
        ((self.opcode & 0x00F0) >> 4) as Register
    }

    fn immediate(&self) -> Immediate {
        (self.opcode & 0x00FF) as Immediate
    }
    
    fn nibble(&self) -> Nibble {
        (self.opcode & 0x000F) as Nibble
    }

    pub fn to_instruction(&self) -> Result<Instruction, String> {
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode {
                0x00E0 => Ok(Instruction::Clear),
                0x00EE => Ok(Instruction::Return),
                _ => Ok(Instruction::SYS(self.addr())),
            }
            0x1000 => Ok(Instruction::Jump(self.addr())),
            0x2000 => Ok(Instruction::Call(self.addr())),
            0x3000 => Ok(Instruction::SkipEqI(self.x_register(), self.immediate())),
            0x4000 => Ok(Instruction::SkipNeqI(self.x_register(), self.immediate())),
            0x5000 => Ok(Instruction::SkipEq(self.x_register(), self.y_register())),
            0x6000 => Ok(Instruction::LoadI(self.x_register(), self.immediate())),
            0x7000 => Ok(Instruction::AddI(self.x_register(), self.immediate())),
            0x8000 => match self.opcode & 0x000F {
                0x0 => Ok(Instruction::LoadR(self.x_register(), self.y_register())),
                0x1 => Ok(Instruction::Or(self.x_register(), self.y_register())),
                0x2 => Ok(Instruction::And(self.x_register(), self.y_register())),
                0x3 => Ok(Instruction::Xor(self.x_register(), self.y_register())),
                0x4 => Ok(Instruction::Add(self.x_register(), self.y_register())),
                0x5 => Ok(Instruction::Sub(self.x_register(), self.y_register())),
                0x6 => Ok(Instruction::ShiftR(self.x_register())),
                0x7 => Ok(Instruction::SubN(self.x_register(), self.y_register())),
                0xE => Ok(Instruction::ShiftL(self.x_register())),
                _ => Err("Invalid instruction!".to_string()),
            },
            0x9000 => match self.opcode & 0x000F {
                0x0 => Ok(Instruction::SkipNeq(self.x_register(), self.y_register())),
                _ => Err("Invalid instruction!".to_string()),
            }
            0xA000 => Ok(Instruction::LoadIdx(self.addr())),
            0xB000 => Ok(Instruction::JumpAddV0(self.addr())),
            0xC000 => Ok(Instruction::Rand(self.x_register(), self.immediate())),
            0xD000 => Ok(Instruction::Draw(self.x_register(), self.y_register(), self.nibble())),
            0xE000 => match self.opcode & 0x00FF {
                0x9E => Ok(Instruction::SkipEqKey(self.x_register())),
                0xA1 => Ok(Instruction::SkipNeqKey(self.x_register())),
                _ => Err("Invalid instruction!".to_string()),
            }
            0xF000 => match self.opcode & 0x00FF {
                0x07 => Ok(Instruction::LoadFromDT(self.x_register())),
                0x0A => Ok(Instruction::LoadKey(self.x_register())),
                0x15 => Ok(Instruction::SetDT(self.x_register())),
                0x18 => Ok(Instruction::SetST(self.x_register())),
                0x1E => Ok(Instruction::AddIdx(self.x_register())),
                0x29 => Ok(Instruction::LoadSprite(self.x_register())),
                0x33 => Ok(Instruction::LoadBCD(self.x_register())),
                0x55 => Ok(Instruction::StoreRegs(self.x_register())),
                0x65 => Ok(Instruction::ReadRegs(self.x_register())),
                _ => Err("Invalid instruction!".to_string()),
            }
            n => {
                unreachable!()
            }
        }
    }
}

