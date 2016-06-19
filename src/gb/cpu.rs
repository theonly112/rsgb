use gb::mmu::Mmu;
use gb::mmu::MmuRead;
use gb::registers::Registers;
use gb::registers::Reg8;
use gb::registers::Reg16;
use gb::registers::Flags;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Cpu {
    regs: Rc<RefCell<Registers>>,
    mmu: Rc<RefCell<Mmu>>,

    ticks: i32,
}

impl Cpu {
    pub fn new(registers: Rc<RefCell<Registers>>, mmu: Rc<RefCell<Mmu>>) -> Cpu {
        Cpu {
            regs: registers,
            mmu: mmu,
            ticks: 0,
        }
    }

    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            0x00 => self.nop(),
            0x05 => self.dec_r8(Reg8::B),
            0x06 => self.ld_r8_n(Reg8::B),
            0x07 => self.rlca(),
            0x08 => self.ld_nn_ptr_sp(),
            0x0d => self.dec_r8(Reg8::C),
            0x0e => self.ld_r8_n(Reg8::C),
            0x14 => self.inc_r8(Reg8::D),
            0x15 => self.dec_r8(Reg8::D),
            0x16 => self.ld_r8_n(Reg8::D),
            0x19 => self.add_hl_r16(Reg16::DE),
            0x1f => self.rra(),
            0x20 => self.jr_nz_n(),
            0x21 => self.ld_r16_nn(Reg16::HL),
            0x25 => self.dec_r8(Reg8::H),
            0x29 => self.add_hl_r16(Reg16::HL),
            0x32 => self.ldd_hl_ptr_a(),
            0x3E => self.ld_r8_n(Reg8::A),
            0x77 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::A),
            0x7b => self.ld_r8_r8(Reg8::A, Reg8::E),
            0xc3 => self.jp_nn(),
            0xaf => self.xor_r8(Reg8::A),
            0xb0 => self.or_r8(Reg8::B),
            0xbf => self.cp_r8(Reg8::A),
            0xe0 => self.ld_ff_n_ap(),
            0xf0 => self.ld_ff_a_ptr_n(),
            0xf3 => self.di(),
            _ => {
                panic!("Instruction not implemented : {:02X} at pc : {:04X}",
                       instruction,
                       self.regs.borrow().pc)
            }
        }
    }

    fn read_arg8(&mut self) -> u8 {
        let pc = self.regs.borrow().pc;
        let val = self.mmu.borrow().read_u8(pc);
        self.regs.borrow_mut().pc = pc + 1;
        return val;
    }

    fn read_arg16(&mut self) -> u16 {
        let pc = self.regs.borrow().pc;
        let val = self.mmu.borrow().read_u16(pc);
        self.regs.borrow_mut().pc = pc + 2;
        return val;
    }

    // 0x01
    fn nop(&self) {}

    fn ld_r8_n(&mut self, r: Reg8) {
        let n = self.read_arg8();
        self.regs.borrow_mut().write_r8(&r, n);
    }

    fn ld_r16_nn(&mut self, r: Reg16) {
        let nn = self.read_arg16();
        self.regs.borrow_mut().write_r16(r, nn);
    }

    // 0xC3
    fn jp_nn(&self) {
        let pc = self.regs.borrow().pc;
        let addr = self.mmu.borrow().read_u16(pc);
        self.regs.borrow_mut().pc = addr;
    }

    fn xor_r8(&mut self, reg: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut a_value = regs.read_r8(&Reg8::A);
        let reg_val = regs.read_r8(&reg);
        a_value ^= reg_val;

        regs.write_r8(&reg, a_value);

        if a_value > 0 {
            regs.clear(Flags::Zero);
        } else {
            regs.set(Flags::Zero);
        }

        regs.clear(Flags::Carry);
        regs.clear(Flags::Negative);
        regs.clear(Flags::HalfCarry);
    }

    fn ldd_hl_ptr_a(&mut self) {
        let mut mmu = self.mmu.borrow_mut();
        let regs = self.regs.borrow_mut();
        mmu.write_u8(regs.read_r16(&Reg16::HL), regs.read_r8(&Reg8::A));
    }

    fn dec_r8(&mut self, r: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut reg_value = regs.read_r8(&r);

        if (reg_value & 0x0F) > 0 {
            regs.clear(Flags::HalfCarry);
        } else {
            regs.set(Flags::HalfCarry);
        }

        reg_value = reg_value.wrapping_sub(1);

        if reg_value > 0 {
            regs.clear(Flags::Zero);
        } else {
            regs.set(Flags::Zero);
        }

        regs.set(Flags::Negative);
        regs.write_r8(&r, reg_value);
    }

    fn jr_nz_n(&mut self) {
        let relative = self.read_arg8() as i8;
        let mut regs = self.regs.borrow_mut();
        if regs.check(Flags::Zero) {
            self.ticks += 8;
        } else {
            regs.pc = ((regs.pc as i16) + relative as i16) as u16;
            self.ticks += 12;
        }
    }

    fn rra(&mut self) {
        let mut regs = self.regs.borrow_mut();
        let carry = match regs.check(Flags::Carry) {
            true => 1 << 7,
            false => 0,
        };

        let mut a_value = regs.read_r8(&Reg8::A);
        if (a_value & 0x01) > 0 {
            regs.set(Flags::Carry);
        } else {
            regs.clear(Flags::Carry);
        }
        a_value >>= 1;
        a_value = (a_value & carry);


        regs.write_r8(&Reg8::A, a_value);

        regs.clear(Flags::Negative);
        regs.clear(Flags::Zero);
        regs.clear(Flags::HalfCarry);
    }

    fn or_r8(&mut self, r: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut a_value = regs.read_r8(&Reg8::A);
        let r8_value = regs.read_r8(&r);

        a_value |= r8_value;

        regs.write_r8(&Reg8::A, a_value);

        if a_value > 0 {
            regs.clear(Flags::Zero);
        } else {
            regs.set(Flags::Zero);
        }


        regs.clear(Flags::Carry);
        regs.clear(Flags::Negative);
        regs.clear(Flags::HalfCarry);
    }

    fn inc_r8(&mut self, r: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut value = regs.read_r8(&r);

        if (value & 0x0f) == 0x0f {
            regs.set(Flags::HalfCarry);
        } else {
            regs.clear(Flags::HalfCarry);
        }

        value += 1;

        if value > 0 {
            regs.clear(Flags::Zero);
        } else {
            regs.set(Flags::Zero);
        }

        regs.clear(Flags::Negative);

        regs.write_r8(&r, value);
    }

    fn ld_r8_r8(&mut self, r8_lhs: Reg8, r8_rhs: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let rhs_value = regs.read_r8(&r8_rhs);
        regs.write_r8(&r8_lhs, rhs_value);
    }

    fn cp_r8(&mut self, r: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let a_value = regs.read_r8(&Reg8::A);
        let r8_value = regs.read_r8(&r);

        if a_value == r8_value {
            regs.set(Flags::Zero);
        } else {
            regs.clear(Flags::Zero);
        }

        if r8_value > a_value {
            regs.set(Flags::Carry);
        } else {
            regs.clear(Flags::Carry);
        }

        if (r8_value & 0x0f) > (a_value & 0x0f) {
            regs.set(Flags::HalfCarry);
        } else {
            regs.clear(Flags::HalfCarry);
        }

        regs.clear(Flags::Negative);
    }

    fn add_hl_r16(&mut self, r: Reg16) {
        let mut regs = self.regs.borrow_mut();
        let mut hl_value = regs.read_r16(&Reg16::HL);
        let r16_value = regs.read_r16(&r);

        let result: u32 = hl_value as u32 + r16_value as u32;

        if (result & 0xffff0000) > 0 {
            regs.set(Flags::Carry);
        } else {
            regs.clear(Flags::Carry);
        }

        hl_value = (result & 0xffff) as u16;
        regs.write_r16(Reg16::HL, hl_value);

        if (hl_value & 0x0f) + (r16_value & 0x0f) > 0x0f {
            regs.set(Flags::HalfCarry);
        } else {
            regs.clear(Flags::HalfCarry);
        }

        regs.clear(Flags::Negative);
    }

    fn ld_r16_ptr_r8(&mut self, r16: Reg16, r8: Reg8) {
        let regs = self.regs.borrow_mut();
        let r16_value = regs.read_r16(&r16);
        let r8_value = regs.read_r8(&r8);
        let mut mmu = self.mmu.borrow_mut();
        mmu.write_u8(r16_value, r8_value);
    }

    fn rlca(&mut self) {
        let mut regs = self.regs.borrow_mut();
        let mut a_value = regs.read_r8(&Reg8::A);
        let carry = (a_value & 0x80) >> 7;

        if carry > 0 {
            regs.set(Flags::Carry);
        } else {
            regs.clear(Flags::Carry);
        }

        a_value <<= 1;
        a_value += carry;

        regs.write_r8(&Reg8::A, a_value);

        regs.clear(Flags::Negative);
        regs.clear(Flags::Zero);
        regs.clear(Flags::HalfCarry);
    }

    fn ld_nn_ptr_sp(&mut self) {
        let addr = self.read_arg16();
        let regs = self.regs.borrow();
        let mut mmu = self.mmu.borrow_mut();
        mmu.write_u16(addr, regs.sp);
    }

    fn di(&mut self) {
        println!("TODO: disable interrupts");
    }

    fn ld_ff_n_ap(&mut self) {
        let addr = self.read_arg8();
        let mut mmu = self.mmu.borrow_mut();
        let a_value = self.regs.borrow().read_r8(&Reg8::A);
        mmu.write_u8(0xFF00 + addr as u16, a_value);
    }

    fn ld_ff_a_ptr_n(&mut self) {
        let addr = self.read_arg8();
        let mem_value = self.mmu.borrow().read_u8(0xFF00 + addr as u16);
        self.regs.borrow_mut().write_r8(&Reg8::A, mem_value);
    }
}
