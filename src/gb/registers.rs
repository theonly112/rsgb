#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}
#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}


pub enum Flags {
    Carry = 0b_0001_0000,
    HalfCarry = 0b_0010_0000,
    Negative = 0b_0100_0000,
    Zero = 0b_1000_0000,
}

pub struct Registers {
    pub a: u8,
    f: u8,
    b: u8,
    pub c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }

    pub fn read_r16(&self, r: Reg16) -> u16 {
        match r {
            Reg16::AF => ((self.a as u16) << 8) + self.f as u16,
            Reg16::BC => ((self.b as u16) << 8) + self.c as u16,
            Reg16::DE => ((self.d as u16) << 8) + self.e as u16,
            Reg16::HL => ((self.h as u16) << 8) + self.l as u16,
            Reg16::SP => self.sp,
            Reg16::PC => self.pc,
        }
    }

    pub fn read_r8(&self, r: Reg8) -> u8 {
        match r {
            Reg8::A => self.a,
            Reg8::F => self.f,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    pub fn write_r16(&mut self, r: Reg16, val: u16) {
        match r {
            Reg16::AF => {
                self.a = (val >> 8) as u8;
                self.f = (val & 0xff) as u8
            }
            Reg16::BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0xff) as u8
            }
            Reg16::DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0xff) as u8
            }
            Reg16::HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8
            }
            Reg16::SP => self.sp = val,
            Reg16::PC => self.pc = val,
        }
    }

    pub fn set(&mut self, f: Flags) {
        self.f |= f as u8;
    }

    pub fn check(&self, f: Flags) -> bool {
        return (self.f & (f as u8)) > 0;
    }

    pub fn clear(&mut self, f: Flags) {
        self.f &= !(f as u8);
    }

    pub fn write_r8(&mut self, r: Reg8, val: u8) {
        match r {
            Reg8::A => self.a = val,
            Reg8::F => self.f = val,
            Reg8::B => self.b = val,
            Reg8::C => self.c = val,
            Reg8::D => self.d = val,
            Reg8::E => self.e = val,
            Reg8::H => self.h = val,
            Reg8::L => self.l = val,
        }
    }
}


use std::fmt;
impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "PC: {:04X} SP: {:04X} AF: {:04X} BC: {:04X} DE: {:04X} HL: {:04X}",
               self.pc,
               self.sp,
               self.read_r16(Reg16::AF),
               self.read_r16(Reg16::BC),
               self.read_r16(Reg16::DE),
               self.read_r16(Reg16::HL))
    }
}