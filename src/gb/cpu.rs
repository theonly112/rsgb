use gb::mmu::Mmu;
use gb::mmu::MmuRead;
use gb::registers::Registers;
use gb::registers::Reg8;
use gb::registers::Reg16;
use gb::registers::Flags;
use gb::interrupts::Interrupts;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Cpu {
    regs: Rc<RefCell<Registers>>,
    mmu: Rc<RefCell<Mmu>>,
    int: Rc<RefCell<Interrupts>>,
    pub ticks: i32,
}

impl Cpu {
    pub fn new(registers: Rc<RefCell<Registers>>,
               mmu: Rc<RefCell<Mmu>>,
               int: Rc<RefCell<Interrupts>>)
               -> Cpu {
        Cpu {
            regs: registers,
            mmu: mmu,
            int: int,
            ticks: 0,
        }
    }

    pub fn execute(&mut self, instruction: u8) -> i32 {
        match instruction {
            0x00 => self.nop(),
            0x01 => self.ld_r16_nn(Reg16::BC),
            0x05 => self.dec_r8(Reg8::B),
            0x06 => self.ld_r8_n(Reg8::B),
            0x07 => self.rlca(),
            0x08 => self.ld_nn_ptr_sp(),
            0x0b => self.dec_r16(Reg16::BC),
            0x0c => self.inc_r8(Reg8::C),
            0x0d => self.dec_r8(Reg8::C),
            0x0e => self.ld_r8_n(Reg8::C),
            0x11 => self.ld_r16_nn(Reg16::DE),
            0x12 => self.ld_r16_ptr_r8(Reg16::DE, Reg8::A),
            0x13 => self.inc_r16(Reg16::DE),
            0x14 => self.inc_r8(Reg8::D),
            0x15 => self.dec_r8(Reg8::D),
            0x16 => self.ld_r8_n(Reg8::D),
            0x18 => self.jr_n(),
            0x19 => self.add_hl_r16(Reg16::DE),
            0x1a => self.ld_r8_r16ptr(Reg8::A, Reg16::DE),
            0x1c => self.inc_r8(Reg8::E),
            0x1f => self.rra(),
            0x20 => self.jr_nz_n(),
            0x21 => self.ld_r16_nn(Reg16::HL),
            0x22 => self.ldi_hlptr_a(),
            0x23 => self.inc_r16(Reg16::HL),
            0x25 => self.dec_r8(Reg8::H),
            0x28 => self.jr_z_n(),
            0x29 => self.add_hl_r16(Reg16::HL),
            0x2a => self.ldi_a_hlptr(),
            0x2c => self.inc_r8(Reg8::L),
            0x2f => self.cpl(),
            0x31 => self.ld_r16_nn(Reg16::SP),
            0x32 => self.ldd_hl_ptr_a(),
            0x34 => self.inc_hlptr(),
            0x35 => self.dec_hlptr(),
            0x36 => self.ld_hlptr_n(),
            0x3c => self.inc_r8(Reg8::A),
            0x3d => self.dec_r8(Reg8::A),
            0x3E => self.ld_r8_n(Reg8::A),
            0x47 => self.ld_r8_r8(Reg8::B, Reg8::A),
            0x4f => self.ld_r8_r8(Reg8::C, Reg8::A),
            0x56 => self.ld_r8_r16ptr(Reg8::D, Reg16::HL),
            0x5e => self.ld_r8_r16ptr(Reg8::E, Reg16::HL),
            0x5d => self.ld_r8_r8(Reg8::E, Reg8::L),
            0x5f => self.ld_r8_r8(Reg8::E, Reg8::A),
            0x77 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::A),
            0x78 => self.ld_r8_r8(Reg8::A, Reg8::B),
            0x79 => self.ld_r8_r8(Reg8::A, Reg8::C),
            0x7b => self.ld_r8_r8(Reg8::A, Reg8::E),
            0x7c => self.ld_r8_r8(Reg8::A, Reg8::H),
            0x7e => self.ld_r8_r16ptr(Reg8::A, Reg16::HL),
            0x87 => self.add_a_r8(Reg8::A),
            0xa1 => self.and_r8(Reg8::B),
            0xa7 => self.and_r8(Reg8::A),
            0xa9 => self.xor_r8(Reg8::C),
            0xaf => self.xor_r8(Reg8::A),
            0xb0 => self.or_r8(Reg8::B),
            0xb1 => self.or_r8(Reg8::C),
            0xbf => self.cp_r8(Reg8::A),
            0xc0 => self.ret_nz(),
            0xc1 => self.pop_r16(Reg16::BC),
            0xc3 => self.jp_nn(),
            0xc5 => self.push_r16(Reg16::BC),
            0xc8 => self.ret_z(),
            0xc9 => self.ret(),
            0xca => self.jp_z_nn(),
            0xcb => self.cb(),
            0xcd => self.call_nn(),
            0xd1 => self.pop_r16(Reg16::DE),
            0xd5 => self.push_r16(Reg16::DE),
            0xd8 => self.ret_c(),
            0xd9 => self.reti(),
            0xdd => panic!("Unknown Instruction"),
            0xe5 => self.push_r16(Reg16::HL),
            0xe0 => self.ld_ff_n_ap(),
            0xe1 => self.pop_r16(Reg16::HL),
            0xe2 => self.ld_ff_c_a(),
            0xe6 => self.and_n(),
            0xe9 => self.jp_hl(),
            0xea => self.ld_nn_ptr_a(),
            0xef => self.rst(0x0028),
            0xf0 => self.ld_ff_a_ptr_n(),
            0xf1 => self.pop_r16(Reg16::AF),
            0xf3 => self.di(),
            0xf5 => self.push_r16(Reg16::AF),
            0xfa => self.ld_a_nnptr(),
            0xfb => self.int.borrow_mut().enable_interrupts(),
            0xfe => self.cp_n(),
            _ => {
                panic!("Instruction not implemented : {:02X} at pc : {:04X}",
                       instruction,
                       self.regs.borrow().pc - 1)
            }
        }
        if instruction != 0xcb {
            self.ticks += TICKS[instruction as usize];
        }
        self.ticks
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
        self.regs.borrow_mut().write_r8(r, n);
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

    fn and_r8(&mut self, reg: Reg8) {
        let a_value = {
            let mut regs = self.regs.borrow_mut();
            let reg_val = regs.read_r8(reg);

            let mut a_value = regs.read_r8(Reg8::A);
            a_value &= reg_val;


            regs.clear(Flags::Carry);
            regs.clear(Flags::Negative);
            regs.set(Flags::HalfCarry);
            a_value
        };
        self.zero_flag_u8(a_value);
    }

    fn and_n(&mut self) {
        let n = self.read_arg8();
        let mut a_value = self.regs.borrow().read_r8(Reg8::A);
        a_value &= n;

        self.zero_flag_u8(a_value);
        self.regs.borrow_mut().clear(Flags::Carry);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().set(Flags::HalfCarry);
        self.regs.borrow_mut().write_r8(Reg8::A, a_value);
    }

    fn xor_r8(&mut self, reg: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut a_value = regs.read_r8(Reg8::A);
        let reg_val = regs.read_r8(reg);
        a_value ^= reg_val;

        regs.write_r8(Reg8::A, a_value);

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
        let mut regs = self.regs.borrow_mut();
        let mut hl = regs.read_r16(Reg16::HL);
        mmu.write_u8(hl, regs.read_r8(Reg8::A));
        hl -= 1;
        regs.write_r16(Reg16::HL, hl);
    }

    fn dec_r8(&mut self, r: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut reg_value = regs.read_r8(r);

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
        regs.write_r8(r, reg_value);
    }

    fn rra(&mut self) {
        let mut regs = self.regs.borrow_mut();
        let carry = match regs.check(Flags::Carry) {
            true => 1 << 7,
            false => 0,
        };

        let mut a_value = regs.read_r8(Reg8::A);
        if (a_value & 0x01) > 0 {
            regs.set(Flags::Carry);
        } else {
            regs.clear(Flags::Carry);
        }
        a_value >>= 1;
        a_value = a_value & carry;


        regs.write_r8(Reg8::A, a_value);

        regs.clear(Flags::Negative);
        regs.clear(Flags::Zero);
        regs.clear(Flags::HalfCarry);
    }

    fn or_r8(&mut self, r: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut a_value = regs.read_r8(Reg8::A);
        let r8_value = regs.read_r8(r);

        a_value |= r8_value;

        regs.write_r8(Reg8::A, a_value);

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
        let mut value = regs.read_r8(r);

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

        regs.write_r8(r, value);
    }

    fn ld_r8_r8(&mut self, r8_lhs: Reg8, r8_rhs: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let rhs_value = regs.read_r8(r8_rhs);
        regs.write_r8(r8_lhs, rhs_value);
    }

    fn cp_r8(&mut self, r: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let a_value = regs.read_r8(Reg8::A);
        let r8_value = regs.read_r8(r);

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
        let mut hl_value = regs.read_r16(Reg16::HL);
        let r16_value = regs.read_r16(r);

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
        let r16_value = regs.read_r16(r16);
        let r8_value = regs.read_r8(r8);
        let mut mmu = self.mmu.borrow_mut();
        mmu.write_u8(r16_value, r8_value);
    }

    fn rlca(&mut self) {
        let mut regs = self.regs.borrow_mut();
        let mut a_value = regs.read_r8(Reg8::A);
        let carry = (a_value & 0x80) >> 7;

        if carry > 0 {
            regs.set(Flags::Carry);
        } else {
            regs.clear(Flags::Carry);
        }

        a_value <<= 1;
        a_value += carry;

        regs.write_r8(Reg8::A, a_value);

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
        self.int.borrow_mut().disable_interrupts();
    }

    fn ld_ff_n_ap(&mut self) {
        let addr = self.read_arg8();
        let mut mmu = self.mmu.borrow_mut();
        let a_value = self.regs.borrow().read_r8(Reg8::A);
        mmu.write_u8(0xFF00 + addr as u16, a_value);
    }

    fn ld_ff_a_ptr_n(&mut self) {
        let addr = self.read_arg8();
        let mem_value = self.mmu.borrow().read_u8(0xFF00 + addr as u16);
        self.regs.borrow_mut().write_r8(Reg8::A, mem_value);
    }

    fn cp_n(&mut self) {
        let n = self.read_arg8();
        let mut regs = self.regs.borrow_mut();

        let a_value = regs.read_r8(Reg8::A);

        if a_value == n {
            regs.set(Flags::Zero);
        } else {
            regs.clear(Flags::Zero);
        }

        if n > a_value {
            regs.set(Flags::Carry);
        } else {
            regs.clear(Flags::Carry);
        }

        if (n & 0x0f) > (a_value & 0x0f) {
            regs.set(Flags::HalfCarry);
        } else {
            regs.clear(Flags::HalfCarry);
        }
        regs.set(Flags::Negative);
    }

    fn call_nn(&mut self) {
        let nn = self.read_arg16();
        let pc = self.regs.borrow_mut().pc;
        self.push_u16(pc);
        self.regs.borrow_mut().pc = nn;
    }

    fn ld_a_nnptr(&mut self) {
        let nn = self.read_arg16();
        let val = self.mmu.borrow().read_u8(nn);
        self.regs.borrow_mut().write_r8(Reg8::A, val);
    }

    fn inc_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.inc(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn dec_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.dec(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn reti(&self) {
        self.int.borrow_mut().master = true;
        let pc = self.pop_u16();
        self.regs.borrow_mut().pc = pc;
    }

    fn ret_nz(&mut self) {
        if self.regs.borrow_mut().check(Flags::Zero) {
            self.ticks += 8;
        } else {
            self.regs.borrow_mut().pc = self.pop_u16();
            self.ticks += 20;
        }
    }

    fn ld_hlptr_n(&mut self) {
        let n = self.read_arg8();
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        self.mmu.borrow_mut().write_u8(hl, n);
    }

    fn ld_nn_ptr_a(&mut self) {
        let nn = self.read_arg16();
        let a = self.regs.borrow().a;
        self.mmu.borrow_mut().write_u8(nn, a);
    }

    fn ldi_a_hlptr(&self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let val = self.mmu.borrow().read_u8(hl);
        self.regs.borrow_mut().a = val;
        self.regs.borrow_mut().write_r16(Reg16::HL, hl + 1);
    }

    fn ldi_hlptr_a(&self) {
        let mut hl = self.regs.borrow().read_r16(Reg16::HL);
        let a = self.regs.borrow().read_r8(Reg8::A);
        self.mmu.borrow_mut().write_u8(hl, a);
        hl += 1;
        self.regs.borrow_mut().write_r16(Reg16::HL, hl);
    }

    fn ld_ff_c_a(&self) {
        let addr = 0xFF00 + self.regs.borrow().c as u16;
        let a = self.regs.borrow().a;
        self.mmu.borrow_mut().write_u8(addr, a);
    }

    fn dec_r16(&self, reg: Reg16) {
        let mut regs = self.regs.borrow_mut();
        let mut val = regs.read_r16(reg);
        val -= 1;
        regs.write_r16(reg, val);
    }

    fn ret(&self) {
        let pc = self.pop_u16();
        self.regs.borrow_mut().pc = pc;
    }

    fn ret_z(&mut self) {
        if self.regs.borrow().check(Flags::Zero) {
            let pc = self.pop_u16();
            self.regs.borrow_mut().pc = pc;
            self.ticks += 20;
        } else {
            self.ticks += 8;
        }
    }

    fn ret_c(&mut self) {
        if self.regs.borrow().check(Flags::Carry) {
            let pc = self.pop_u16();
            self.regs.borrow_mut().pc = pc;
            self.ticks += 20;
        } else {
            self.ticks += 8;
        }
    }

    fn cpl(&self) {
        let a = !self.regs.borrow().a;
        self.regs.borrow_mut().a = a;
    }

    fn rst(&mut self, addr: u16) {
        self.push_r16(Reg16::PC);
        self.regs.borrow_mut().pc = addr;
    }

    fn add(&self, lhs: u8, rhs: u8) -> u8 {
        let result = lhs as u16 + rhs as u16;

        self.carry_flag((result & 0xff00) != 0);

        let lhs = (result & 0xff) as u8;

        self.zero_flag_u8(lhs);

        self.half_carry_flag((lhs & 0x0f) + (rhs & 0x0f) > 0x0f);

        self.regs.borrow_mut().clear(Flags::Negative);

        lhs
    }

    fn add_a_r8(&mut self, reg: Reg8) {
        let rhs = self.regs.borrow().read_r8(reg);
        let lhs = self.regs.borrow().a;

        let result = self.add(lhs, rhs);

        self.regs.borrow_mut().write_r8(Reg8::A, result);
    }

    fn ld_r8_r16ptr(&mut self, lhs: Reg8, rhs: Reg16) {
        let mut regs = self.regs.borrow_mut();
        let addr = regs.read_r16(rhs);
        let val = self.mmu.borrow_mut().read_u8(addr);
        regs.write_r8(lhs, val);
    }

    fn inc_r16(&self, reg: Reg16) {
        let mut regs = self.regs.borrow_mut();
        let value = regs.read_r16(reg) + 1;
        regs.write_r16(reg, value);
    }

    // jumps

    fn jp_hl(&mut self) {
        let mut regs = self.regs.borrow_mut();
        regs.pc = regs.read_r16(Reg16::HL);
    }

    fn jp_z_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Zero) {
            self.regs.borrow_mut().pc = nn;
            self.ticks += 16;
        } else {
            self.ticks += 12;
        }
    }


    fn jr_z_n(&mut self) {
        let relative = self.read_arg8() as i8;
        let mut regs = self.regs.borrow_mut();
        if regs.check(Flags::Zero) {
            self.ticks += 12;
            regs.pc = ((regs.pc as i16) + relative as i16) as u16;
        } else {
            self.ticks += 8;
        }
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

    fn jr_n(&mut self) {
        let relative = self.read_arg8() as i8;
        let mut regs = self.regs.borrow_mut();
        regs.pc = ((regs.pc as i16) + relative as i16) as u16;
    }


    // Helper functions for common instructions

    fn inc(&mut self, value: u8) -> u8 {
        self.half_carry_flag((value & 0x0f) == 0x0f);
        let value = value + 1;
        self.zero_flag_u8(value);
        self.regs.borrow_mut().clear(Flags::Negative);
        return value;
    }

    fn dec(&mut self, value: u8) -> u8 {
        self.half_carry_flag((value & 0x0f) == 0);
        let value = value - 1;
        self.zero_flag_u8(value);
        self.regs.borrow_mut().set(Flags::Negative);
        return value;
    }

    // Helper functions for stack

    fn pop_u16(&self) -> u16 {
        let mut sp = self.regs.borrow_mut().sp;
        let value = self.mmu.borrow_mut().read_u16(sp);
        sp += 2;
        self.regs.borrow_mut().sp = sp;
        // println!("poped \t\t{:04X}", value);
        return value;
    }

    fn push_u16(&mut self, val: u16) {
        self.regs.borrow_mut().sp -= 2;
        let sp = self.regs.borrow_mut().sp;
        self.mmu.borrow_mut().write_u16(sp, val);
        // println!("pushing \t{:04X}", val);
    }

    fn push_r16(&mut self, reg: Reg16) {
        let reg_value = self.regs.borrow_mut().read_r16(reg);
        self.push_u16(reg_value);
    }

    fn pop_r16(&mut self, reg: Reg16) {
        let value = self.pop_u16();
        self.regs.borrow_mut().write_r16(reg, value);
    }

    // Helper fuctions for setting Flags
    #[allow(dead_code)]
    fn half_carry_flag(&self, val: bool) {
        if val {
            self.regs.borrow_mut().set(Flags::HalfCarry);
        } else {
            self.regs.borrow_mut().clear(Flags::HalfCarry);
        }
    }
    #[allow(dead_code)]
    fn carry_flag(&self, val: bool) {
        if val {
            self.regs.borrow_mut().set(Flags::Carry);
        } else {
            self.regs.borrow_mut().clear(Flags::Carry);
        }
    }
    #[allow(dead_code)]
    fn zero_flag_u8(&self, val: u8) {
        if val > 0 {
            self.regs.borrow_mut().clear(Flags::Zero);
        } else {
            self.regs.borrow_mut().set(Flags::Zero);
        }
    }
    #[allow(dead_code)]
    fn zero_flag_bool(&self, val: bool) {
        if val {
            self.regs.borrow_mut().set(Flags::Zero);
        } else {
            self.regs.borrow_mut().clear(Flags::Zero);
        }
    }

    // Extended instruction set

    fn cb(&mut self) {
        let instruction = self.read_arg8();

        match instruction {
            0x37 => self.swap_r8(Reg8::A),
            0x87 => self.res_bit_r8(0, Reg8::A),
            _ => {
                panic!("CB Instruction not implemented : {:02X} at pc : {:04X}",
                       instruction,
                       self.regs.borrow().pc - 1)
            }
        }

        self.ticks += CB_TICKS[instruction as usize];
    }
    // helpers
    fn swap(&self, value: u8) -> u8 {
        let value = ((value & 0x0f) << 4) | ((value & 0x0f) >> 4);
        self.zero_flag_u8(value);

        let mut regs = self.regs.borrow_mut();
        regs.clear(Flags::Negative);
        regs.clear(Flags::HalfCarry);
        regs.clear(Flags::Carry);

        value
    }



    fn swap_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.swap(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn res_bit_r8(&self, bit: u8, reg: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut value = regs.read_r8(reg);
        value &= !(1 << bit);
        regs.write_r8(reg, value);
    }
}

static TICKS: [i32; 256] =
    [2, 6, 4, 4, 2, 2, 4, 4, 10, 4, 4, 4, 2, 2, 4, 4 /* 0x0_ */, 2, 6, 4, 4, 2, 2, 4, 4, 4,
     4, 4, 4, 2, 2, 4, 4 /* 0x1_ */, 0, 6, 4, 4, 2, 2, 4, 2, 0, 4, 4, 4, 2, 2, 4,
     2 /* 0x2_ */, 4, 6, 4, 4, 6, 6, 6, 2, 0, 4, 4, 4, 2, 2, 4, 2 /* 0x3_ */, 2, 2, 2,
     2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2 /* 0x4_ */, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2,
     2, 4, 2 /* 0x5_ */, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2 /* 0x6_ */, 4,
     4, 4, 4, 4, 4, 2, 4, 2, 2, 2, 2, 2, 2, 4, 2 /* 0x7_ */, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2,
     2, 2, 2, 4, 2 /* 0x8_ */, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4,
     2 /* 0x9_ */, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2 /* 0xa_ */, 2, 2, 2,
     2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2 /* 0xb_ */, 0, 6, 0, 6, 0, 8, 4, 8, 0, 2, 0, 0, 0,
     6, 4, 8 /* 0xc_ */, 0, 6, 0, 0, 0, 8, 4, 8, 0, 8, 0, 0, 0, 0, 4, 8 /* 0xd_ */, 6,
     6, 4, 0, 0, 8, 4, 8, 8, 2, 8, 0, 0, 0, 4, 8 /* 0xe_ */, 6, 6, 4, 2, 0, 8, 4, 8, 6, 4, 8,
     2, 0, 0, 4, 8 /* 0xf_ */];

static CB_TICKS: [i32; 256] =
    [8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8 /* 0x0_ */, 8, 8, 8, 8, 8, 8, 16, 8, 8,
     8, 8, 8, 8, 8, 16, 8 /* 0x1_ */, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16,
     8 /* 0x2_ */, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8 /* 0x3_ */, 8, 8, 8,
     8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8 /* 0x4_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8,
     8, 8, 12, 8 /* 0x5_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12,
     8 /* 0x6_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8 /* 0x7_ */, 8, 8, 8,
     8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8 /* 0x8_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8,
     8, 8, 12, 8 /* 0x9_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12,
     8 /* 0xa_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8 /* 0xb_ */, 8, 8, 8,
     8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8 /* 0xc_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8,
     8, 8, 12, 8 /* 0xd_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12,
     8 /* 0xe_ */, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8 /* 0xf_ */];