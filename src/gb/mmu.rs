use gb::catridge::Cartrige;

pub struct Mmu {
    rom: Vec<u8>,
}

impl Mmu {
    pub fn new(cart: Cartrige) -> Mmu {
        Mmu { rom: cart.rom }
    }
}

impl MmuRead for Mmu {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF => self.rom[addr as usize],
            _ => panic!("invalid addr"),
        }
    }

    fn read_u16(&self, addr: u16) -> u16 {
        0
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