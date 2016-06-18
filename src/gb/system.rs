// use gb::cartrige::Cartrige;
use gb::catridge::Cartrige;
use gb::cpu::Registers;
use gb::cpu::Cpu;
use gb::mmu::Mmu;
use gb::mmu::MmuRead;

pub struct System {
    cartirge: Cartrige,
    registers: Registers,
    cpu: Cpu,
    mmu: Mmu,
}

impl System {
    pub fn new(cart: Cartrige) -> System {
        System {
            cartirge: cart.clone(),
            cpu: Cpu::new(),
            registers: Registers::new(),
            mmu: Mmu::new(cart),
        }
    }

    pub fn run(&self) {
        let instruction = self.mmu.read_u8(self.registers.pc);
        //self.cpu.execute(instruction);
    }
}