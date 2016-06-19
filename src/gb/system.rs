use gb::catridge::Cartrige;
use gb::registers::Registers;
use gb::cpu::Cpu;
use gb::mmu::Mmu;
use gb::mmu::MmuRead;

use std::cell::RefCell;
use std::rc::Rc;

pub struct System {
    registers: Rc<RefCell<Registers>>,
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,
}

impl System {
    pub fn new(cart: Cartrige) -> System {

        let regs = Rc::new(RefCell::new(Registers::new()));
        let cart = Rc::new(cart);
        let mmu = Rc::new(RefCell::new(Mmu::new(cart.clone())));
        let cpu = Cpu::new(regs.clone(), mmu.clone());


        System {
            cpu: cpu,
            registers: regs,
            mmu: mmu,
        }
    }

    pub fn run(&mut self) {
        while true {
            let pc = self.registers.borrow().pc;
            let instruction = self.mmu.borrow().read_u8(self.registers.borrow().pc);
            self.registers.borrow_mut().pc = pc + 1;
            self.cpu.execute(instruction);
        }
    }
}