use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Register {
    AL,
    BL,
    CL,
    DL,
    AH,
    BH,
    CH,
    DH,
    AX,
    BX,
    CX,
    DX,
    SP,
    BP,
    SI,
    DI,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let reg = match *self {
            Register::AL => "al",
            Register::BL => "bl",
            Register::CL => "cl",
            Register::DL => "dl",
            Register::AH => "ah",
            Register::BH => "bh",
            Register::CH => "ch",
            Register::DH => "dh",
            Register::AX => "ax",
            Register::BX => "bx",
            Register::CX => "cx",
            Register::DX => "dx",
            Register::SP => "sp",
            Register::BP => "bp",
            Register::SI => "si",
            Register::DI => "di",
        };
        write!(f, "{}", reg)
    }
}


pub fn decode_register(byte: u8, w: u8) -> Register {
    let w = w == 1;
    match byte {
        0b000 => if w { Register::AX } else { Register::AL },
        0b001 => if w { Register::CX } else { Register::CL },
        0b010 => if w { Register::DX } else { Register::DL },
        0b011 => if w { Register::BX } else { Register::BL },
        0b100 => if w { Register::SP } else { Register::AH },
        0b101 => if w { Register::BP } else { Register::CH },
        0b110 => if w { Register::SI } else { Register::DH },
        0b111 => if w { Register::DI } else { Register::BH },
        _ => panic!("UNKNOWN REGISTER")
    }
}

pub fn address_calculation(rm: u8, mod_: u8) -> String {
    match mod_ {
        0b00 => {
            match rm {
                0b000 => "[bx + si]",
                0b001 => "[bx + di]",
                0b010 => "[bp + si]",
                0b011 => "[bp + di]",
                0b100 => "[si]",
                0b101 => "[di]",
                0b110 => "[bp]",
                0b111 => "[bx]",
                _ => unreachable!()
            }
        }
        0b01 => {
            match rm {
                0b000 => "[bx + si",
                0b001 => "[bx + di",
                0b010 => "[bp + si",
                0b011 => "[bp + di",
                0b100 => "[si",
                0b101 => "[di",
                0b110 => "[bp",
                0b111 => "[bx",
                _ => unreachable!()
            }
        }
        0b10 => {
            match rm {
                0b000 => "[bx + si",
                0b001 => "[bx + di",
                0b010 => "[bp + si",
                0b011 => "[bp + di",
                0b100 => "[si",
                0b101 => "[di",
                0b110 => "[bp",
                0b111 => "[bx",
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    }.to_string()
}
