use gb::catridge::Cartrige;

use std::rc::Rc;

pub struct Mmu {
    cart: Rc<Cartrige>,
}

impl Mmu {
    pub fn new(cart: Rc<Cartrige>) -> Mmu {
        Mmu { cart: cart }
    }
}

impl MmuRead for Mmu {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF => self.cart.rom[addr as usize],
            _ => panic!("invalid addr"),
        }
    }

    fn read_u16(&self, addr: u16) -> u16 {
        self.read_u8(addr) as u16 + ((self.read_u8(addr + 1) as u16) << 8)
    }

    fn write_u8(&self, addr: u16, val: u8) {}

    fn write_u16(&self, addr: u16, val: u16) {}
}

pub trait MmuRead {
    fn read_u8(&self, addr: u16) -> u8;
    fn read_u16(&self, addr: u16) -> u16;
    fn write_u8(&self, addr: u16, val: u8);
    fn write_u16(&self, addr: u16, val: u16);
}