use gb::mmu::Mmu;
use gb::mmu::MmuRead;
use gb::registers::Registers;
use gb::registers::Reg8;
use gb::registers::Flags;

use std::cell::RefCell;
use std::rc::Rc;
use std::ops::DerefMut;

pub struct Cpu {
    regs: Rc<RefCell<Registers>>,
    mmu: Rc<RefCell<Mmu>>,
}

impl Cpu {
    pub fn new(registers: Rc<RefCell<Registers>>, mmu: Rc<RefCell<Mmu>>) -> Cpu {
        Cpu {
            regs: registers,
            mmu: mmu,
        }
    }

    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            0x00 => self.nop(),
            0xc3 => self.jp_nn(),
            0xaf => self.xor_r8(Reg8::A),
            _ => {
                panic!("Instruction not implemented : {:X} at pc : {:X}",
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

    // 0x01
    fn nop(&self) {}

    // 0xC3
    fn jp_nn(&self) {
        let pc = self.regs.borrow().pc;
        let addr = self.mmu.borrow().read_u16(pc);
        self.regs.borrow_mut().pc = addr;
    }

    // 0xAF
    fn xor_r8(&mut self, reg: Reg8) {
        let arg = self.read_arg8();

        let mut regs = self.regs.borrow_mut();
        let mut reg_val = regs.read_r8(&reg);
        reg_val ^= arg;

        regs.write_r8(&reg, reg_val);

        if reg_val > 0 {
            regs.clear(Flags::Zero);
        } else {
            regs.set(Flags::Zero);
        }

        regs.clear(Flags::Carry);
        regs.clear(Flags::Negative);
        regs.clear(Flags::HalfCarry);
    }
}
