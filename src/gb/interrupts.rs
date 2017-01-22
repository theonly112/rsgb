use gb::mmu::*;
use gb::registers::*;
use gb::gpu::Gpu;
use gb::display::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interrupts {
    pub master: bool,
    mmu: Rc<RefCell<Mmu>>,
    regs: Rc<RefCell<Registers>>,
    gpu: Rc<RefCell<Gpu>>,
}

pub const INTERRUPT_ENABLE: u16 = 0xFFFF;
pub const INTERRUPT_FLAG: u16 = 0xFF0F;

// const NONE: u8 = 0;
pub const VBLANK: u8 = (1 << 0);
const LCDSTAT: u8 = (1 << 1);
const TIMER: u8 = (1 << 2);
const SERIAL: u8 = (1 << 3);
const JOYPAD: u8 = (1 << 4);

impl Interrupts {
    pub fn new(mmu: Rc<RefCell<Mmu>>,
               regs: Rc<RefCell<Registers>>,
               gpu: Rc<RefCell<Gpu>>)
               -> Interrupts {
        Interrupts {
            master: true,
            mmu: mmu,
            regs: regs,
            gpu: gpu,
        }
    }

    pub fn disable_interrupts(&mut self) {
        self.master = false;
    }

    pub fn enable_interrupts(&mut self) {
        self.master = true;
    }


    pub fn step(&mut self, mut display: &mut SdlDisplay) -> i32 {
        let mut flags = self.mmu.borrow_mut().read_u8(INTERRUPT_FLAG);
        let enable = self.mmu.borrow_mut().read_u8(INTERRUPT_ENABLE);
        let mut ticks = 0;
        let master = self.master;

        if master && enable != 0 && flags != 0 {
            let fire = enable & flags;
            if fire & VBLANK != 0 {
                self.handle_vblank(&mut display);
                flags &= !VBLANK;
                self.mmu.borrow_mut().write_u8(INTERRUPT_FLAG, flags);
                ticks = 12;
            }
            if fire & LCDSTAT != 0 {
                self.handle_lcdstat();
                flags &= !LCDSTAT;
                self.mmu.borrow_mut().write_u8(INTERRUPT_FLAG, flags);
                ticks = 12;
            }
            if fire & TIMER != 0 {
                self.handle_timer();
                flags &= !TIMER;
                self.mmu.borrow_mut().write_u8(INTERRUPT_FLAG, flags);
                ticks = 12;
            }
            if fire & SERIAL != 0 {
                self.handle_serial();
                flags &= !SERIAL;
                self.mmu.borrow_mut().write_u8(INTERRUPT_FLAG, flags);
                ticks = 12;
            }
            if fire & JOYPAD != 0 {
                self.handle_joypad();
                flags &= !JOYPAD;
                self.mmu.borrow_mut().write_u8(INTERRUPT_FLAG, flags);
                ticks = 12;
            }
        }

        ticks
    }


    fn push_pc(&self) {
        let pc = self.regs.borrow_mut().pc;
        let mut sp = self.regs.borrow_mut().sp;
        sp -= 2;
        self.regs.borrow_mut().sp = sp;
        self.mmu.borrow_mut().write_u16(sp, pc);
    }

    pub fn handle_vblank(&mut self, mut display: &mut SdlDisplay) {
        self.push_pc();
        self.master = false;
        self.regs.borrow_mut().pc = 0x40;

        // TODO cpu ticks += 12
        display.draw(self.gpu.borrow().framebuffer);


    }
    pub fn handle_lcdstat(&mut self) {
        println!("lcdstat");
        self.push_pc();
        self.master = false;
        self.regs.borrow_mut().pc = 0x48;


    }
    pub fn handle_timer(&mut self) {
        println!("timer");
        self.push_pc();
        self.master = false;
        self.regs.borrow_mut().pc = 0x50;

    }
    pub fn handle_serial(&mut self) {
        print!("serial");
        self.push_pc();
        self.master = false;
        self.regs.borrow_mut().pc = 0x58;

    }
    pub fn handle_joypad(&mut self) {
        print!("joypad");
        self.push_pc();
        self.master = false;
        self.regs.borrow_mut().pc = 0x60;
    }
}