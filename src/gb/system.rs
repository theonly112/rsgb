use gb::catridge::Cartrige;
use gb::registers::Registers;
use gb::cpu::Cpu;
use gb::mmu::Mmu;
use gb::mmu::MmuRead;
use gb::gpu::Gpu;
use gb::interrupts::Interrupts;
use gb::component::SystemComponent;
use gb::display::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct System {
    registers: Rc<RefCell<Registers>>,
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,
    gpu: Rc<RefCell<Gpu>>,
    int: Rc<RefCell<Interrupts>>,
}

impl System {
    pub fn new(cart: Cartrige) -> System {
        let gpu = Rc::new(RefCell::new(Gpu::new()));
        let regs = Rc::new(RefCell::new(Registers::new()));
        let cart = Rc::new(cart);

        let mmu = Mmu::new(cart.clone(), gpu.clone());
        let mmu = Rc::new(RefCell::new(mmu));
        gpu.borrow_mut().mmu = Some(mmu.clone());

        let int = Interrupts::new(mmu.clone(), regs.clone(), gpu.clone());
        let int = Rc::new(RefCell::new(int));

        let cpu = Cpu::new(regs.clone(), mmu.clone(), int.clone());

        System {
            cpu: cpu,
            registers: regs,
            mmu: mmu,
            gpu: gpu,
            int: int,
        }
    }

    #[allow(while_true)]
    pub fn run(&mut self, mut dis: &mut SdlDisplay) {
        self.mmu.borrow_mut().reset();
        self.gpu.borrow_mut().reset();
        // use std::io::prelude::*;
        // use std::fs::File;
        // let mut buffer = File::create("exec.log").unwrap();
        while true {
            // write!(&mut buffer,
            //        "{:?} TK: {}\n",
            //        self.registers.borrow(),
            //        self.cpu.ticks)
            //     .unwrap();
            let pc = self.registers.borrow().pc;
            let instruction = self.mmu.borrow().read_u8(self.registers.borrow().pc);
            self.registers.borrow_mut().pc = pc + 1;
            let ticks = self.cpu.execute(instruction);
            self.gpu.borrow_mut().step(ticks);
            let int_ticks = self.int.borrow_mut().step(&mut dis);
            self.cpu.ticks += int_ticks;
        }
    }
}