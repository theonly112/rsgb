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
    halted: bool,
    stopped: bool,
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
            halted: false,
            stopped: false,
            ticks: 0,
        }
    }

    pub fn execute(&mut self, instruction: u8) -> i32 {
        match instruction {
            0x00 => self.nop(),
            0x01 => self.ld_r16_nn(Reg16::BC),
            0x02 => self.ld_r16_ptr_r8(Reg16::BC, Reg8::A),
            0x03 => self.inc_r16(Reg16::BC),
            0x04 => self.inc_r8(Reg8::B),
            0x05 => self.dec_r8(Reg8::B),
            0x06 => self.ld_r8_n(Reg8::B),
            0x07 => self.rlca(),
            0x08 => self.ld_nn_ptr_sp(),
            0x09 => self.add_hl_r16(Reg16::BC),
            0x0a => self.ld_r8_r16ptr(Reg8::A, Reg16::BC),
            0x0b => self.dec_r16(Reg16::BC),
            0x0c => self.inc_r8(Reg8::C),
            0x0d => self.dec_r8(Reg8::C),
            0x0e => self.ld_r8_n(Reg8::C),
            0x0f => self.rrca(),
            0x10 => self.stop(),
            0x11 => self.ld_r16_nn(Reg16::DE),
            0x12 => self.ld_r16_ptr_r8(Reg16::DE, Reg8::A),
            0x13 => self.inc_r16(Reg16::DE),
            0x14 => self.inc_r8(Reg8::D),
            0x15 => self.dec_r8(Reg8::D),
            0x16 => self.ld_r8_n(Reg8::D),
            0x17 => self.rla(),
            0x18 => self.jr_n(),
            0x19 => self.add_hl_r16(Reg16::DE),
            0x1a => self.ld_r8_r16ptr(Reg8::A, Reg16::DE),
            0x1b => self.dec_r16(Reg16::DE),
            0x1c => self.inc_r8(Reg8::E),
            0x1d => self.dec_r8(Reg8::E),
            0x1e => self.ld_r8_n(Reg8::E),
            0x1f => self.rra(),
            0x20 => self.jr_nz_n(),
            0x21 => self.ld_r16_nn(Reg16::HL),
            0x22 => self.ldi_hlptr_a(),
            0x23 => self.inc_r16(Reg16::HL),
            0x24 => self.inc_r8(Reg8::H),
            0x25 => self.dec_r8(Reg8::H),
            0x26 => self.ld_r8_n(Reg8::H),
            0x27 => self.daa(),
            0x28 => self.jr_z_n(),
            0x29 => self.add_hl_r16(Reg16::HL),
            0x2a => self.ldi_a_hlptr(),
            0x2b => self.dec_r16(Reg16::HL),
            0x2c => self.inc_r8(Reg8::L),
            0x2d => self.dec_r8(Reg8::L),
            0x2e => self.ld_r8_n(Reg8::L),
            0x2f => self.cpl(),
            0x30 => self.jr_nc_n(),
            0x31 => self.ld_r16_nn(Reg16::SP),
            0x32 => self.ldd_hl_ptr_a(),
            0x33 => self.inc_r16(Reg16::SP),
            0x34 => self.inc_hlptr(),
            0x35 => self.dec_hlptr(),
            0x36 => self.ld_hlptr_n(),
            0x37 => self.scf(),
            0x38 => self.jr_c_n(),
            0x39 => self.add_hl_r16(Reg16::SP),
            0x3a => self.ldd_a_hlptr(),
            0x3b => self.dec_r16(Reg16::SP),
            0x3c => self.inc_r8(Reg8::A),
            0x3d => self.dec_r8(Reg8::A),
            0x3e => self.ld_r8_n(Reg8::A),
            0x3f => self.ccf(),
            0x40 => self.ld_r8_r8(Reg8::B, Reg8::B),
            0x41 => self.ld_r8_r8(Reg8::B, Reg8::C),
            0x42 => self.ld_r8_r8(Reg8::B, Reg8::D),
            0x43 => self.ld_r8_r8(Reg8::B, Reg8::E),
            0x44 => self.ld_r8_r8(Reg8::B, Reg8::H),
            0x45 => self.ld_r8_r8(Reg8::B, Reg8::L),
            0x46 => self.ld_r8_r16ptr(Reg8::B, Reg16::HL),
            0x47 => self.ld_r8_r8(Reg8::B, Reg8::A),
            0x48 => self.ld_r8_r8(Reg8::C, Reg8::B),
            0x49 => self.ld_r8_r8(Reg8::C, Reg8::C),
            0x4a => self.ld_r8_r8(Reg8::C, Reg8::D),
            0x4b => self.ld_r8_r8(Reg8::C, Reg8::E),
            0x4c => self.ld_r8_r8(Reg8::C, Reg8::H),
            0x4d => self.ld_r8_r8(Reg8::C, Reg8::L),
            0x4e => self.ld_r8_r16ptr(Reg8::C, Reg16::HL),
            0x4f => self.ld_r8_r8(Reg8::C, Reg8::A),
            0x50 => self.ld_r8_r8(Reg8::D, Reg8::B),
            0x51 => self.ld_r8_r8(Reg8::D, Reg8::C),
            0x52 => self.ld_r8_r8(Reg8::D, Reg8::D),
            0x53 => self.ld_r8_r8(Reg8::D, Reg8::E),
            0x54 => self.ld_r8_r8(Reg8::D, Reg8::H),
            0x55 => self.ld_r8_r8(Reg8::D, Reg8::L),
            0x56 => self.ld_r8_r16ptr(Reg8::D, Reg16::HL),
            0x57 => self.ld_r8_r8(Reg8::D, Reg8::A),
            0x58 => self.ld_r8_r8(Reg8::E, Reg8::B),
            0x59 => self.ld_r8_r8(Reg8::E, Reg8::C),
            0x5a => self.ld_r8_r8(Reg8::E, Reg8::D),
            0x5b => self.ld_r8_r8(Reg8::E, Reg8::E),
            0x5c => self.ld_r8_r8(Reg8::E, Reg8::H),
            0x5d => self.ld_r8_r8(Reg8::E, Reg8::L),
            0x5e => self.ld_r8_r16ptr(Reg8::E, Reg16::HL),
            0x5f => self.ld_r8_r8(Reg8::E, Reg8::A),
            0x60 => self.ld_r8_r8(Reg8::H, Reg8::B),
            0x61 => self.ld_r8_r8(Reg8::H, Reg8::C),
            0x62 => self.ld_r8_r8(Reg8::H, Reg8::D),
            0x63 => self.ld_r8_r8(Reg8::H, Reg8::E),
            0x64 => self.ld_r8_r8(Reg8::H, Reg8::H),
            0x65 => self.ld_r8_r8(Reg8::H, Reg8::L),
            0x66 => self.ld_r8_r16ptr(Reg8::H, Reg16::HL),
            0x67 => self.ld_r8_r8(Reg8::H, Reg8::A),
            0x68 => self.ld_r8_r8(Reg8::L, Reg8::B),
            0x69 => self.ld_r8_r8(Reg8::L, Reg8::C),
            0x6a => self.ld_r8_r8(Reg8::L, Reg8::D),
            0x6b => self.ld_r8_r8(Reg8::L, Reg8::E),
            0x6c => self.ld_r8_r8(Reg8::L, Reg8::H),
            0x6d => self.ld_r8_r8(Reg8::L, Reg8::L),
            0x6e => self.ld_r8_r16ptr(Reg8::L, Reg16::HL),
            0x6f => self.ld_r8_r8(Reg8::L, Reg8::A),
            0x70 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::B),
            0x71 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::C),
            0x72 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::D),
            0x73 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::E),
            0x74 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::H),
            0x75 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::L),
            0x76 => self.halt(),
            0x77 => self.ld_r16_ptr_r8(Reg16::HL, Reg8::A),
            0x78 => self.ld_r8_r8(Reg8::A, Reg8::B),
            0x79 => self.ld_r8_r8(Reg8::A, Reg8::C),
            0x7a => self.ld_r8_r8(Reg8::A, Reg8::D),
            0x7b => self.ld_r8_r8(Reg8::A, Reg8::E),
            0x7c => self.ld_r8_r8(Reg8::A, Reg8::H),
            0x7d => self.ld_r8_r8(Reg8::A, Reg8::L),
            0x7e => self.ld_r8_r16ptr(Reg8::A, Reg16::HL),
            0x7f => self.ld_r8_r8(Reg8::A, Reg8::A),
            0x80 => self.add_a_r8(Reg8::B),
            0x81 => self.add_a_r8(Reg8::C),
            0x82 => self.add_a_r8(Reg8::D),
            0x83 => self.add_a_r8(Reg8::E),
            0x84 => self.add_a_r8(Reg8::H),
            0x85 => self.add_a_r8(Reg8::L),
            0x86 => self.add_a_hlptr(),
            0x87 => self.add_a_r8(Reg8::A),
            0x88 => self.adc_r8(Reg8::B),
            0x89 => self.adc_r8(Reg8::C),
            0x8a => self.adc_r8(Reg8::D),
            0x8b => self.adc_r8(Reg8::E),
            0x8c => self.adc_r8(Reg8::H),
            0x8d => self.adc_r8(Reg8::L),
            0x8e => self.adc_hlptr(),
            0x8f => self.adc_r8(Reg8::A),
            0x90 => self.sub_r8(Reg8::B),
            0x91 => self.sub_r8(Reg8::C),
            0x92 => self.sub_r8(Reg8::D),
            0x93 => self.sub_r8(Reg8::E),
            0x94 => self.sub_r8(Reg8::H),
            0x95 => self.sub_r8(Reg8::L),
            0x96 => self.sub_hlptr(),
            0x97 => self.sub_r8(Reg8::A),
            0x98 => self.sbc_r8(Reg8::B),
            0x99 => self.sbc_r8(Reg8::C),
            0x9a => self.sbc_r8(Reg8::D),
            0x9b => self.sbc_r8(Reg8::E),
            0x9c => self.sbc_r8(Reg8::H),
            0x9d => self.sbc_r8(Reg8::L),
            0x9e => self.sbc_hlptr(),
            0x9f => self.sbc_r8(Reg8::A),
            0xa0 => self.and_r8(Reg8::B),
            0xa1 => self.and_r8(Reg8::C),
            0xa2 => self.and_r8(Reg8::D),
            0xa3 => self.and_r8(Reg8::E),
            0xa4 => self.and_r8(Reg8::H),
            0xa5 => self.and_r8(Reg8::L),
            0xa6 => self.and_hlptr(),
            0xa7 => self.and_r8(Reg8::A),
            0xa8 => self.xor_r8(Reg8::B),
            0xa9 => self.xor_r8(Reg8::C),
            0xaa => self.xor_r8(Reg8::D),
            0xab => self.xor_r8(Reg8::E),
            0xac => self.xor_r8(Reg8::H),
            0xad => self.xor_r8(Reg8::L),
            0xae => self.xor_hlptr(),
            0xaf => self.xor_r8(Reg8::A),
            0xb0 => self.or_r8(Reg8::B),
            0xb1 => self.or_r8(Reg8::C),
            0xb2 => self.or_r8(Reg8::D),
            0xb3 => self.or_r8(Reg8::E),
            0xb4 => self.or_r8(Reg8::H),
            0xb5 => self.or_r8(Reg8::L),
            0xb6 => self.or_hlptr(),
            0xb7 => self.or_r8(Reg8::A),
            0xb8 => self.cp_r8(Reg8::B),
            0xb9 => self.cp_r8(Reg8::C),
            0xba => self.cp_r8(Reg8::D),
            0xbb => self.cp_r8(Reg8::E),
            0xbc => self.cp_r8(Reg8::H),
            0xbd => self.cp_r8(Reg8::L),
            0xbe => self.cp_hlptr(),
            0xbf => self.cp_r8(Reg8::A),
            0xc0 => self.ret_nz(),
            0xc1 => self.pop_r16(Reg16::BC),
            0xc2 => self.jp_nz_nn(),
            0xc3 => self.jp_nn(),
            0xc4 => self.call_nz_nn(),
            0xc5 => self.push_r16(Reg16::BC),
            0xc6 => self.add_a_n(),
            0xc7 => self.rst(0x0000),
            0xc8 => self.ret_z(),
            0xc9 => self.ret(),
            0xca => self.jp_z_nn(),
            0xcb => self.cb(),
            0xcc => self.call_z_nn(),
            0xcd => self.call_nn(),
            0xce => self.adc_n(),
            0xcf => self.rst(0x0008),
            0xd0 => self.ret_nc(),
            0xd1 => self.pop_r16(Reg16::DE),
            0xd2 => self.jp_nc_nn(),
            0xd3 => panic!("Unknown Instruction"),
            0xd4 => self.call_nc_nn(),
            0xd5 => self.push_r16(Reg16::DE),
            0xd6 => self.sub_n(),
            0xd7 => self.rst(0x0010),
            0xd8 => self.ret_c(),
            0xd9 => self.reti(),
            0xda => self.jp_c_nn(),
            0xdb => panic!("Unknown Instruction"),
            0xdc => self.call_c_nn(),
            0xdd => panic!("Unknown Instruction"),
            0xde => self.sbc_n(),
            0xdf => self.rst(0x0018),
            0xe0 => self.ld_ff_n_ap(),
            0xe1 => self.pop_r16(Reg16::HL),
            0xe2 => self.ld_ff_c_a(),
            0xe3 => panic!("Unknown Instruction"),
            0xe4 => panic!("Unknown Instruction"),
            0xe5 => self.push_r16(Reg16::HL),
            0xe6 => self.and_n(),
            0xe7 => self.rst(0x0020),
            0xe8 => self.add_sp_n(),
            0xe9 => self.jp_hl(),
            0xea => self.ld_nn_ptr_a(),
            0xeb => panic!("Unknown Instruction"),
            0xec => panic!("Unknown Instruction"),
            0xed => panic!("Unknown Instruction"),
            0xee => self.xor_n(),
            0xef => self.rst(0x0028),
            0xf0 => self.ld_ff_a_ptr_n(),
            0xf1 => self.pop_r16(Reg16::AF),
            0xf2 => self.ld_a_ff_c(),
            0xf3 => self.di(),
            0xf4 => panic!("Unknown Instruction"),
            0xf5 => self.push_r16(Reg16::AF),
            0xf6 => self.or_n(),
            0xf7 => self.rst(0x0030),
            0xf8 => self.ld_hl_sp_n(),
            0xf9 => self.ld_sp_hl(),
            0xfa => self.ld_a_nnptr(),
            0xfb => self.int.borrow_mut().enable_interrupts(),
            0xfc => panic!("Unknown Instruction"),
            0xfd => panic!("Unknown Instruction"),
            0xfe => self.cp_n(),
            0xff => self.rst(0x0038),
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

            regs.a = a_value;
            a_value
        };
        self.zero_flag_u8(a_value);
    }

    fn and_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);

        let mut a_value = self.regs.borrow().read_r8(Reg8::A);
        a_value &= value;

        self.zero_flag_u8(a_value);

        self.regs.borrow_mut().clear(Flags::Carry);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().set(Flags::HalfCarry);

        self.regs.borrow_mut().a = a_value;
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

    fn xor(&mut self, value: u8) {
        let mut regs = self.regs.borrow_mut();
        let mut a_value = regs.read_r8(Reg8::A);
        a_value ^= value;

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

    fn xor_n(&mut self) {
        let n = self.read_arg8();
        self.xor(n);
    }

    fn xor_r8(&mut self, reg: Reg8) {
        let reg_val = self.regs.borrow_mut().read_r8(reg);
        self.xor(reg_val);
    }

    fn xor_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.xor(value);
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

    fn rrca(&mut self) {
        let mut a = self.regs.borrow().a;
        let carry = a & 0xf1;
        self.carry_flag(carry > 0);
        a >>= 1;
        if carry > 0 {
            a |= 0x80;
        }
        self.regs.borrow_mut().a = a;
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::Zero);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
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

    fn rla(&mut self) {
        let carry = if self.regs.borrow().check(Flags::Carry) {
            1
        } else {
            0
        };

        let mut a = self.regs.borrow().a;
        self.carry_flag(a & 0x80 > 0);
        a <<= 1;
        a += carry;

        self.regs.borrow_mut().a = a;

        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::Zero);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
    }

    fn or_r8(&mut self, r: Reg8) {
        let r8_value = self.regs.borrow_mut().read_r8(r);
        self.or(r8_value);
    }

    fn or_n(&mut self) {
        //
        let n = self.read_arg8();
        self.or(n);
    }

    fn or_hlptr(&mut self) {
        let hl_value = self.regs.borrow_mut().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl_value);
        self.or(value);
    }

    fn or(&mut self, value: u8) {
        let mut a_value = self.regs.borrow_mut().read_r8(Reg8::A);
        a_value |= value;

        self.zero_flag_u8(a_value);

        let mut regs = self.regs.borrow_mut();
        regs.write_r8(Reg8::A, a_value);
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

        value = value.wrapping_add(1);

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
        let r8_value = self.regs.borrow_mut().read_r8(r);
        self.cp(r8_value);
    }

    fn cp(&mut self, value: u8) {
        let a = self.regs.borrow().a;
        self.zero_flag_bool(a == value);
        self.carry_flag(value > a);
        self.half_carry_flag((value & 0x0f) > (a & 0x0f));
        // TODO: make sure this is set negative and not clear
        self.regs.borrow_mut().set(Flags::Negative);
    }
    fn cp_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.cp(value);
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

    fn add_sp_n(&mut self) {
        let n = self.read_arg8() as i8;
        let result = (self.regs.borrow().sp as i32 + n as i32) as u32;
        self.carry_flag((result & 0xffff0000) > 0);
        self.regs.borrow_mut().sp = (result & 0xffff) as u16;
        let sp = self.regs.borrow_mut().sp;

        self.half_carry_flag((sp & 0x0f) as u8 + (n & 0x0f) as u8 > 0x0f);
        self.regs.borrow_mut().clear(Flags::Zero);
        self.regs.borrow_mut().clear(Flags::Negative);
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

    fn ld_a_ff_c(&mut self) {
        let c = self.regs.borrow().read_r8(Reg8::C);
        let value = self.mmu.borrow().read_u8(0xff00 + c as u16);
        self.regs.borrow_mut().a = value;
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

    fn call_nc_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Carry) {
            self.ticks += 12;
        } else {
            let pc = self.regs.borrow_mut().pc;
            self.push_u16(pc);
            self.regs.borrow_mut().pc = nn;
            self.ticks += 24;
        }
    }

    fn call_c_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Carry) {
            let pc = self.regs.borrow_mut().pc;
            self.push_u16(pc);
            self.regs.borrow_mut().pc = nn;
            self.ticks += 24;
        } else {
            self.ticks += 12;
        }
    }

    fn call_nz_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Zero) {
            self.ticks += 12;
        } else {
            let pc = self.regs.borrow_mut().pc;
            self.push_u16(pc);
            self.regs.borrow_mut().pc = nn;
            self.ticks += 24;
        }
    }

    fn call_z_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Zero) {
            let pc = self.regs.borrow_mut().pc;
            self.push_u16(pc);
            self.regs.borrow_mut().pc = nn;
            self.ticks += 24;
        } else {
            self.ticks += 12;
        }
    }

    fn ld_hl_sp_n(&mut self) {
        let n = self.read_arg8() as i8;
        let result = (self.regs.borrow().sp as i32 + n as i32) as u32;

        let sp = self.regs.borrow().sp;
        self.carry_flag((result & 0xffff0000) > 0);
        self.half_carry_flag((sp & 0x0f) as u16 + (n & 0x0f) as u16 > 0x0f);
        self.regs.borrow_mut().clear(Flags::Zero);
        self.regs.borrow_mut().clear(Flags::Negative);

        self.regs.borrow_mut().write_r16(Reg16::HL, (result & 0xffff) as u16);
    }

    fn ld_sp_hl(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        self.regs.borrow_mut().write_r16(Reg16::SP, hl);
    }

    fn ld_a_nnptr(&mut self) {
        let nn = self.read_arg16();
        let val = self.mmu.borrow().read_u8(nn);
        self.regs.borrow_mut().write_r8(Reg8::A, val);
    }

    fn inc_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.inc(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn dec_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.dec(value);
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

    fn ret_nc(&mut self) {
        if self.regs.borrow().check(Flags::Carry) {
            self.ticks += 8;
        } else {
            self.ticks += 20;
            let pc = self.pop_u16();
            self.regs.borrow_mut().pc = pc;
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

    fn add_a_n(&mut self) {
        let n = self.read_arg8();
        let a = self.regs.borrow().a;
        let result = self.add(a, n);
        self.regs.borrow_mut().a = result;
    }

    fn add_a_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let a = self.regs.borrow().a;
        let value = self.mmu.borrow().read_u8(hl);
        let result = self.add(a, value);
        self.regs.borrow_mut().a = result;
    }

    fn ld_r8_r16ptr(&mut self, lhs: Reg8, rhs: Reg16) {
        let mut regs = self.regs.borrow_mut();
        let addr = regs.read_r16(rhs);
        let val = self.mmu.borrow_mut().read_u8(addr);
        regs.write_r8(lhs, val);
    }

    fn inc_r16(&self, reg: Reg16) {
        let mut regs = self.regs.borrow_mut();
        let value = regs.read_r16(reg).wrapping_add(1);
        regs.write_r16(reg, value);
    }

    fn ldd_a_hlptr(&self) {
        let mut hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.regs.borrow_mut().a = value;
        hl -= 1;
        self.regs.borrow_mut().write_r16(Reg16::HL, hl);
    }

    fn adc_n(&mut self) {
        let n = self.read_arg8();
        self.adc(n);
    }

    fn adc_r8(&mut self, reg: Reg8) {
        let reg_value = self.regs.borrow().read_r8(reg);
        self.adc(reg_value);
    }

    fn adc_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.adc(value);
    }

    fn sub_r8(&mut self, reg: Reg8) {
        let value = self.regs.borrow().read_r8(reg);
        self.sub(value);
    }

    fn sub_n(&mut self) {
        let n = self.read_arg8();
        self.sub(n);
    }

    fn sub_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.sub(value);
    }

    fn sbc(&mut self, value: u8) {
        let value = value.wrapping_add(if self.regs.borrow().check(Flags::Carry) {
            1
        } else {
            0
        });
        self.regs.borrow_mut().set(Flags::Negative);
        let mut a = self.regs.borrow().a;
        self.carry_flag(value > a);
        self.zero_flag_bool(value == a);
        self.half_carry_flag((value & 0x0f) > (a & 0x0f));
        a = a.wrapping_sub(value);

        self.regs.borrow_mut().a = a;
    }
    fn sbc_r8(&mut self, reg: Reg8) {
        let value = self.regs.borrow().read_r8(reg);
        self.sbc(value);
    }

    fn sbc_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.sbc(value);
    }

    fn sbc_n(&mut self) {
        let n = self.read_arg8();
        self.sbc(n);
    }


    fn daa(&mut self) {
        let mut s: u16 = self.regs.borrow().a as u16;
        if self.regs.borrow().check(Flags::Negative) {
            if self.regs.borrow().check(Flags::HalfCarry) {
                s = (s - 0x06) & 0xff;
            }
            if self.regs.borrow().check(Flags::Carry) {
                s = s - 0x60;
            }
        } else {
            if self.regs.borrow().check(Flags::HalfCarry) || (s & 0x0f) > 9 {
                s += 0x06;
            }
            if self.regs.borrow().check(Flags::Carry) || s > 0x9f {
                s += 0x60;
            }
        }

        self.regs.borrow_mut().a = s as u8;
        self.regs.borrow_mut().clear(Flags::HalfCarry);
        self.zero_flag_u8(s as u8);

        if s >= 0x100 {
            self.regs.borrow_mut().set(Flags::Carry);
        }

    }

    fn halt(&mut self) {
        let master = self.int.borrow().master;
        if master {
            self.halted = true;
        } else {
            self.regs.borrow_mut().pc += 1;
        }
    }

    fn stop(&mut self) {
        // TODO: implement stop
        self.stopped = true;
    }

    fn scf(&self) {
        self.regs.borrow_mut().set(Flags::Carry);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
    }

    fn ccf(&self) {
        let carry = self.regs.borrow().check(Flags::Carry);
        self.carry_flag(carry == false);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
    }



    // Helper functions for common instructions
    fn sub(&mut self, value: u8) {
        self.regs.borrow_mut().set(Flags::Negative);
        let a = self.regs.borrow().a;
        self.carry_flag(value > a);
        self.half_carry_flag((value & 0x0f) > (a & 0x0f));

        a.wrapping_sub(value);
        self.regs.borrow_mut().a = a;
        self.zero_flag_u8(a);
    }
    fn adc(&mut self, value: u8) {
        let value = if self.regs.borrow().check(Flags::Carry) {
            value + 1
        } else {
            value
        };

        let result = self.regs.borrow().a as u16 + value as u16;
        self.carry_flag((result & 0xff00) != 0);
        let a = self.regs.borrow().a;
        self.zero_flag_bool(value == a);
        self.half_carry_flag((value & 0x0f) + (a & 0x0f) > 0x0f);
        self.regs.borrow_mut().set(Flags::Negative);
        self.regs.borrow_mut().a = (result & 0xff) as u8;
    }

    fn inc(&mut self, value: u8) -> u8 {
        self.half_carry_flag((value & 0x0f) == 0x0f);
        // TODO should this be a wrapping add?
        let value = value.wrapping_add(1);
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

    fn jp_nz_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Zero) {
            self.ticks += 12;
        } else {
            self.ticks += 16;
            self.regs.borrow_mut().pc = nn;
        }
    }

    fn jp_nc_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Carry) {
            self.ticks += 12;
        } else {
            self.ticks += 16;
            self.regs.borrow_mut().pc = nn;
        }
    }

    fn jp_c_nn(&mut self) {
        let nn = self.read_arg16();
        if self.regs.borrow().check(Flags::Carry) {
            self.ticks += 16;
            self.regs.borrow_mut().pc = nn;
        } else {
            self.ticks += 12;
        }
    }

    fn jr_c_n(&mut self) {
        let relative = self.read_arg8() as i8;
        let mut regs = self.regs.borrow_mut();
        if regs.check(Flags::Carry) {
            self.ticks += 12;
            regs.pc = ((regs.pc as i16) + relative as i16) as u16;
        } else {
            self.ticks += 8;
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

    fn jr_nc_n(&mut self) {
        let relative = self.read_arg8() as i8;
        let mut regs = self.regs.borrow_mut();
        if regs.check(Flags::Carry) {
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
            0x00 => self.rlc_r8(Reg8::B),
            0x01 => self.rlc_r8(Reg8::C),
            0x02 => self.rlc_r8(Reg8::D),
            0x03 => self.rlc_r8(Reg8::E),
            0x04 => self.rlc_r8(Reg8::H),
            0x05 => self.rlc_r8(Reg8::L),
            0x06 => self.rlc_hlptr(),
            0x07 => self.rlc_r8(Reg8::A),
            0x08 => self.rrc_r8(Reg8::B),
            0x09 => self.rrc_r8(Reg8::C),
            0x0a => self.rrc_r8(Reg8::D),
            0x0b => self.rrc_r8(Reg8::E),
            0x0c => self.rrc_r8(Reg8::H),
            0x0d => self.rrc_r8(Reg8::L),
            0x0e => self.rrc_hlptr(),
            0x0f => self.rrc_r8(Reg8::A),
            0x10 => self.rl_r8(Reg8::B),
            0x11 => self.rl_r8(Reg8::C),
            0x12 => self.rl_r8(Reg8::D),
            0x13 => self.rl_r8(Reg8::E),
            0x14 => self.rl_r8(Reg8::H),
            0x15 => self.rl_r8(Reg8::L),
            0x16 => self.rl_hlptr(),
            0x17 => self.rl_r8(Reg8::A),
            0x18 => self.rr_r8(Reg8::B),
            0x19 => self.rr_r8(Reg8::C),
            0x1a => self.rr_r8(Reg8::D),
            0x1b => self.rr_r8(Reg8::E),
            0x1c => self.rr_r8(Reg8::H),
            0x1d => self.rr_r8(Reg8::L),
            0x1e => self.rr_hlptr(),
            0x1f => self.rr_r8(Reg8::A),
            0x20 => self.sla_r8(Reg8::B),
            0x21 => self.sla_r8(Reg8::C),
            0x22 => self.sla_r8(Reg8::D),
            0x23 => self.sla_r8(Reg8::E),
            0x24 => self.sla_r8(Reg8::H),
            0x25 => self.sla_r8(Reg8::L),
            0x26 => self.sla_hlptr(),
            0x27 => self.sla_r8(Reg8::A),
            0x28 => self.sra_r8(Reg8::B),
            0x29 => self.sra_r8(Reg8::C),
            0x2a => self.sra_r8(Reg8::D),
            0x2b => self.sra_r8(Reg8::E),
            0x2c => self.sra_r8(Reg8::H),
            0x2d => self.sra_r8(Reg8::L),
            0x2e => self.sra_hlptr(),
            0x2f => self.sra_r8(Reg8::A),
            0x30 => self.swap_r8(Reg8::B),
            0x31 => self.swap_r8(Reg8::C),
            0x32 => self.swap_r8(Reg8::D),
            0x33 => self.swap_r8(Reg8::E),
            0x34 => self.swap_r8(Reg8::H),
            0x35 => self.swap_r8(Reg8::L),
            0x36 => self.swap_hlptr(),            
            0x37 => self.swap_r8(Reg8::A),
            0x38 => self.srl_r8(Reg8::B),
            0x39 => self.srl_r8(Reg8::C),
            0x3a => self.srl_r8(Reg8::D),
            0x3b => self.srl_r8(Reg8::E),
            0x3c => self.srl_r8(Reg8::H),
            0x3d => self.srl_r8(Reg8::L),
            0x3e => self.srl_hlptr(),
            0x3f => self.srl_r8(Reg8::A),
            0x40 => self.bit_n_r8(0, Reg8::B),
            0x41 => self.bit_n_r8(0, Reg8::C),
            0x42 => self.bit_n_r8(0, Reg8::D),
            0x43 => self.bit_n_r8(0, Reg8::E),
            0x44 => self.bit_n_r8(0, Reg8::H),
            0x45 => self.bit_n_r8(0, Reg8::L),
            0x46 => self.bit_n_hlptr(0),
            0x47 => self.bit_n_r8(0, Reg8::A),
            0x48 => self.bit_n_r8(1, Reg8::B),
            0x49 => self.bit_n_r8(1, Reg8::C),
            0x4a => self.bit_n_r8(1, Reg8::D),
            0x4b => self.bit_n_r8(1, Reg8::E),
            0x4c => self.bit_n_r8(1, Reg8::H),
            0x4d => self.bit_n_r8(1, Reg8::L),
            0x4e => self.bit_n_hlptr(1),
            0x4f => self.bit_n_r8(1, Reg8::A),
            0x50 => self.bit_n_r8(2, Reg8::B),
            0x51 => self.bit_n_r8(2, Reg8::C),
            0x52 => self.bit_n_r8(2, Reg8::D),
            0x53 => self.bit_n_r8(2, Reg8::E),
            0x54 => self.bit_n_r8(2, Reg8::H),
            0x55 => self.bit_n_r8(2, Reg8::L),
            0x56 => self.bit_n_hlptr(2),
            0x57 => self.bit_n_r8(2, Reg8::A),
            0x58 => self.bit_n_r8(3, Reg8::B),
            0x59 => self.bit_n_r8(3, Reg8::C),
            0x5a => self.bit_n_r8(3, Reg8::D),
            0x5b => self.bit_n_r8(3, Reg8::E),
            0x5c => self.bit_n_r8(3, Reg8::H),
            0x5d => self.bit_n_r8(3, Reg8::L),
            0x5e => self.bit_n_hlptr(3),
            0x5f => self.bit_n_r8(3, Reg8::A),
            0x60 => self.bit_n_r8(4, Reg8::B),
            0x61 => self.bit_n_r8(4, Reg8::C),
            0x62 => self.bit_n_r8(4, Reg8::D),
            0x63 => self.bit_n_r8(4, Reg8::E),
            0x64 => self.bit_n_r8(4, Reg8::H),
            0x65 => self.bit_n_r8(4, Reg8::L),
            0x66 => self.bit_n_hlptr(4),
            0x67 => self.bit_n_r8(4, Reg8::A),
            0x68 => self.bit_n_r8(5, Reg8::B),
            0x69 => self.bit_n_r8(5, Reg8::C),
            0x6a => self.bit_n_r8(5, Reg8::D),
            0x6b => self.bit_n_r8(5, Reg8::E),
            0x6c => self.bit_n_r8(5, Reg8::H),
            0x6d => self.bit_n_r8(5, Reg8::L),
            0x6e => self.bit_n_hlptr(5),
            0x6f => self.bit_n_r8(5, Reg8::A),
            0x70 => self.bit_n_r8(6, Reg8::B),
            0x71 => self.bit_n_r8(6, Reg8::C),
            0x72 => self.bit_n_r8(6, Reg8::D),
            0x73 => self.bit_n_r8(6, Reg8::E),
            0x74 => self.bit_n_r8(6, Reg8::H),
            0x75 => self.bit_n_r8(6, Reg8::L),
            0x76 => self.bit_n_hlptr(6),
            0x77 => self.bit_n_r8(6, Reg8::A),
            0x78 => self.bit_n_r8(7, Reg8::B),
            0x79 => self.bit_n_r8(7, Reg8::C),
            0x7a => self.bit_n_r8(7, Reg8::D),
            0x7b => self.bit_n_r8(7, Reg8::E),
            0x7c => self.bit_n_r8(7, Reg8::H),
            0x7d => self.bit_n_r8(7, Reg8::L),
            0x7e => self.bit_n_hlptr(7),
            0x7f => self.bit_n_r8(7, Reg8::A),
            0x80 => self.res_bit_r8(0, Reg8::B),
            0x81 => self.res_bit_r8(0, Reg8::C),
            0x82 => self.res_bit_r8(0, Reg8::D),
            0x83 => self.res_bit_r8(0, Reg8::E),
            0x84 => self.res_bit_r8(0, Reg8::H),
            0x85 => self.res_bit_r8(0, Reg8::L),
            0x86 => self.res_bit_hlptr(0),
            0x87 => self.res_bit_r8(0, Reg8::A),
            0x88 => self.res_bit_r8(1, Reg8::B),
            0x89 => self.res_bit_r8(1, Reg8::C),
            0x8a => self.res_bit_r8(1, Reg8::D),
            0x8b => self.res_bit_r8(1, Reg8::E),
            0x8c => self.res_bit_r8(1, Reg8::H),
            0x8d => self.res_bit_r8(1, Reg8::L),
            0x8e => self.res_bit_hlptr(1),
            0x8f => self.res_bit_r8(1, Reg8::A),
            0x90 => self.res_bit_r8(2, Reg8::B),
            0x91 => self.res_bit_r8(2, Reg8::C),
            0x92 => self.res_bit_r8(2, Reg8::D),
            0x93 => self.res_bit_r8(2, Reg8::E),
            0x94 => self.res_bit_r8(2, Reg8::H),
            0x95 => self.res_bit_r8(2, Reg8::L),
            0x96 => self.res_bit_hlptr(2),
            0x97 => self.res_bit_r8(2, Reg8::A),
            0x98 => self.res_bit_r8(3, Reg8::B),
            0x99 => self.res_bit_r8(3, Reg8::B),
            0x9a => self.res_bit_r8(3, Reg8::B),
            0x9b => self.res_bit_r8(3, Reg8::B),
            0x9c => self.res_bit_r8(3, Reg8::B),
            0x9d => self.res_bit_r8(3, Reg8::B),
            0x9e => self.res_bit_hlptr(3),
            0x9f => self.res_bit_r8(3, Reg8::A),
            0xa0 => self.res_bit_r8(4, Reg8::B),
            0xa1 => self.res_bit_r8(4, Reg8::C),
            0xa2 => self.res_bit_r8(4, Reg8::D),
            0xa3 => self.res_bit_r8(4, Reg8::E),
            0xa4 => self.res_bit_r8(4, Reg8::H),
            0xa5 => self.res_bit_r8(4, Reg8::L),
            0xa6 => self.res_bit_hlptr(4),
            0xa7 => self.res_bit_r8(4, Reg8::A),
            0xa8 => self.res_bit_r8(5, Reg8::B),
            0xa9 => self.res_bit_r8(5, Reg8::C),
            0xaa => self.res_bit_r8(5, Reg8::D),
            0xab => self.res_bit_r8(5, Reg8::E),
            0xac => self.res_bit_r8(5, Reg8::H),
            0xad => self.res_bit_r8(5, Reg8::L),
            0xae => self.res_bit_hlptr(5),
            0xaf => self.res_bit_r8(5, Reg8::A),
            0xb0 => self.res_bit_r8(6, Reg8::B),
            0xb1 => self.res_bit_r8(6, Reg8::C),
            0xb2 => self.res_bit_r8(6, Reg8::D),
            0xb3 => self.res_bit_r8(6, Reg8::E),
            0xb4 => self.res_bit_r8(6, Reg8::H),
            0xb5 => self.res_bit_r8(6, Reg8::L),
            0xb6 => self.res_bit_hlptr(6),
            0xb7 => self.res_bit_r8(6, Reg8::A),
            0xb8 => self.res_bit_r8(7, Reg8::B),
            0xb9 => self.res_bit_r8(7, Reg8::C),
            0xba => self.res_bit_r8(7, Reg8::D),
            0xbb => self.res_bit_r8(7, Reg8::E),
            0xbc => self.res_bit_r8(7, Reg8::H),
            0xbd => self.res_bit_r8(7, Reg8::L),
            0xbe => self.res_bit_hlptr(7),
            0xbf => self.res_bit_r8(7, Reg8::A),
            0xc0 => self.set_bit_r8(0, Reg8::B),
            0xc1 => self.set_bit_r8(0, Reg8::C),
            0xc2 => self.set_bit_r8(0, Reg8::D),
            0xc3 => self.set_bit_r8(0, Reg8::E),
            0xc4 => self.set_bit_r8(0, Reg8::H),
            0xc5 => self.set_bit_r8(0, Reg8::L),
            0xc6 => self.set_bit_hlptr(0),
            0xc7 => self.set_bit_r8(0, Reg8::A),
            0xc8 => self.set_bit_r8(1, Reg8::B),
            0xc9 => self.set_bit_r8(1, Reg8::C),
            0xca => self.set_bit_r8(1, Reg8::D),
            0xcb => self.set_bit_r8(1, Reg8::E),
            0xcc => self.set_bit_r8(1, Reg8::H),
            0xcd => self.set_bit_r8(1, Reg8::L),
            0xce => self.set_bit_hlptr(1),
            0xcf => self.set_bit_r8(1, Reg8::A),
            0xd0 => self.set_bit_r8(2, Reg8::B),
            0xd1 => self.set_bit_r8(2, Reg8::C),
            0xd2 => self.set_bit_r8(2, Reg8::D),
            0xd3 => self.set_bit_r8(2, Reg8::E),
            0xd4 => self.set_bit_r8(2, Reg8::H),
            0xd5 => self.set_bit_r8(2, Reg8::L),
            0xd6 => self.set_bit_hlptr(2),
            0xd7 => self.set_bit_r8(2, Reg8::A),
            0xd8 => self.set_bit_r8(3, Reg8::B),
            0xd9 => self.set_bit_r8(3, Reg8::C),
            0xda => self.set_bit_r8(3, Reg8::D),
            0xdb => self.set_bit_r8(3, Reg8::E),
            0xdc => self.set_bit_r8(3, Reg8::H),
            0xdd => self.set_bit_r8(3, Reg8::L),
            0xde => self.set_bit_hlptr(3),
            0xdf => self.set_bit_r8(3, Reg8::A),
            0xe0 => self.set_bit_r8(4, Reg8::B),
            0xe1 => self.set_bit_r8(4, Reg8::C),
            0xe2 => self.set_bit_r8(4, Reg8::D),
            0xe3 => self.set_bit_r8(4, Reg8::E),
            0xe4 => self.set_bit_r8(4, Reg8::H),
            0xe5 => self.set_bit_r8(4, Reg8::L),
            0xe6 => self.set_bit_hlptr(4),
            0xe7 => self.set_bit_r8(4, Reg8::A),
            0xe8 => self.set_bit_r8(5, Reg8::B),
            0xe9 => self.set_bit_r8(5, Reg8::C),
            0xea => self.set_bit_r8(5, Reg8::D),
            0xeb => self.set_bit_r8(5, Reg8::E),
            0xec => self.set_bit_r8(5, Reg8::H),
            0xed => self.set_bit_r8(5, Reg8::L),
            0xee => self.set_bit_hlptr(5),
            0xef => self.set_bit_r8(5, Reg8::A),
            0xf0 => self.set_bit_r8(6, Reg8::B), 
            0xf1 => self.set_bit_r8(6, Reg8::C), 
            0xf2 => self.set_bit_r8(6, Reg8::D), 
            0xf3 => self.set_bit_r8(6, Reg8::E), 
            0xf4 => self.set_bit_r8(6, Reg8::H), 
            0xf5 => self.set_bit_r8(6, Reg8::L), 
            0xf6 => self.set_bit_hlptr(6), 
            0xf7 => self.set_bit_r8(6, Reg8::A), 
            0xf8 => self.set_bit_r8(7, Reg8::B),
            0xf9 => self.set_bit_r8(7, Reg8::C),
            0xfa => self.set_bit_r8(7, Reg8::D),
            0xfb => self.set_bit_r8(7, Reg8::E),
            0xfc => self.set_bit_r8(7, Reg8::H),
            0xfd => self.set_bit_r8(7, Reg8::L),
            0xfe => self.set_bit_hlptr(7),
            0xff => self.set_bit_r8(7, Reg8::A),
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

    fn sla(&self, value: u8) -> u8 {
        self.carry_flag((value & 0x80) != 0);
        let value = value << 1;
        self.zero_flag_u8(value);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
        return value;
    }

    fn bit(&self, bit: u8, value: u8) {
        self.zero_flag_u8((value & bit));
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().set(Flags::HalfCarry);
    }

    fn srl(&mut self, value: u8) -> u8 {
        self.carry_flag((value & 0x01) != 0);
        let value = value >> 1;
        self.zero_flag_u8(value);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
        return value;
    }

    fn srl_r8(&mut self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.srl(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }
    fn srl_hlptr(&mut self) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.srl(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn bit_n_r8(&self, bit: u8, reg: Reg8) {
        let val = self.regs.borrow().read_r8(reg);
        self.bit((1 << bit), val);
    }

    fn bit_n_hlptr(&self, bit: u8) {
        let hl = self.regs.borrow().read_r16(Reg16::HL);
        let value = self.mmu.borrow().read_u8(hl);
        self.bit(bit, value);
    }

    fn sra(&self, value: u8) -> u8 {
        self.carry_flag((value & 0x01) > 0);
        let value = (value & 0x80) | (value >> 1);
        self.zero_flag_u8(value);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
        return value;
    }

    fn sra_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.sra(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn sra_hlptr(&self) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.sra(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }



    fn sla_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.sla(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn sla_hlptr(&self) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.sla(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn rl(&self, value: u8) -> u8 {
        let carry = if self.regs.borrow().check(Flags::Carry) {
            1
        } else {
            0
        };

        self.carry_flag((value & 0x80) > 0);
        let mut value = value << 1;
        value = value + carry as u8;

        self.zero_flag_u8(value);

        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);

        return value;
    }
    fn rl_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.rl(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn rl_hlptr(&self) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.rl(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn rlc(&self, value: u8) -> u8 {
        let carry = (value & 0x80) >> 7;
        self.carry_flag((value & 0x80) > 0);
        let mut value = value << 1;
        value = value + carry;
        self.zero_flag_u8(value);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
        return value;
    }
    fn rlc_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.rlc(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn rlc_hlptr(&self) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.rlc(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn rrc(&self, value: u8) -> u8 {
        let carry = value & 0x01;
        let mut value = value >> 1;
        self.carry_flag(carry > 0);
        if carry > 0 {
            value |= 0x80;
        }

        self.zero_flag_u8(value);
        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);
        return value;
    }

    fn rrc_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.rrc(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn rrc_hlptr(&self) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.rrc(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn rr(&self, value: u8) -> u8 {
        let mut value = value >> 1;

        if self.regs.borrow().check(Flags::Carry) {
            value |= 0x80;
        }

        self.carry_flag((value & 0x01) > 0);
        self.zero_flag_u8(value);

        self.regs.borrow_mut().clear(Flags::Negative);
        self.regs.borrow_mut().clear(Flags::HalfCarry);

        return value;
    }

    fn rr_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.rr(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn rr_hlptr(&self) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.rr(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn swap_r8(&self, reg: Reg8) {
        let mut value = self.regs.borrow().read_r8(reg);
        value = self.swap(value);
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn swap_hlptr(&self) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value = self.swap(value);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn res_bit_hlptr(&self, bit: u8) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value &= !(1 << bit);
        self.mmu.borrow_mut().write_u8(hl, value);
    }

    fn res_bit_r8(&self, bit: u8, reg: Reg8) {
        let mut regs = self.regs.borrow_mut();
        let mut value = regs.read_r8(reg);
        value &= !(1 << bit);
        regs.write_r8(reg, value);
    }

    fn set_bit_r8(&self, bit: u8, reg: Reg8) {
        let regs = self.regs.borrow();
        let mut value = regs.read_r8(reg);
        value |= 1 << bit;
        self.regs.borrow_mut().write_r8(reg, value);
    }

    fn set_bit_hlptr(&self, bit: u8) {
        let regs = self.regs.borrow();
        let hl = regs.read_r16(Reg16::HL);
        let mut value = self.mmu.borrow().read_u8(hl);
        value |= 1 << bit;
        self.mmu.borrow_mut().write_u8(hl, value);
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