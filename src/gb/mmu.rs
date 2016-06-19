use gb::catridge::Cartrige;

use std::rc::Rc;

pub struct Mmu {
    cart: Rc<Cartrige>,
    wram: [u8; 0x2000],
    hram: [u8; 0x0080],
    io: [u8; 0x0100],
}

impl Mmu {
    pub fn new(cart: Rc<Cartrige>) -> Mmu {
        Mmu {
            cart: cart,
            wram: [0; 0x2000],
            hram: [0; 0x0080],
            io: [0; 0x0100],
        }
    }
}

impl MmuRead for Mmu {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF => self.cart.rom[addr as usize],
            0xA000...0xBFFF => panic!("mbc not implemented"),
            0x8000...0x9FFF => panic!("vram not implemented"),
            0xC000...0xDFFF => panic!("wram"),
            0xE000...0xFDFF => panic!("wram"),
            0xFE00...0xFEFF => panic!("oam"),
            0xFF04 => panic!("pls do rand"),
            0xFF40 => panic!(""),
            0xFF42 => panic!(""),
            0xFF43 => panic!(""),
            0xFF44 => panic!("scanline"),
            0xFF00 => panic!("io"),
            0xFF0F => panic!("interrupts flags"),
            0xFFFF => panic!("interrupts enable"),
            0xFF80...0xFFFE => panic!("hram"),
            0xFF00...0xFF7F => panic!("io"),
            _ => panic!("invalid addr"),
        }
    }

    fn read_u16(&self, addr: u16) -> u16 {
        self.read_u8(addr) as u16 + ((self.read_u8(addr + 1) as u16) << 8)
    }

    fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x7FFF => panic!("mbc not implemented"),
            0x8000...0x9FFF => panic!("vram not implemented"),
            0xA000...0xBFFF => panic!("mbc not implemented"),
            0xC000...0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xE000...0xFDFF => panic!("wram not implemented"),
            0xFE00...0xFEFF => panic!("oam not implemented"),
            0xFF80...0xFFFE => self.hram[(addr - 0xff80) as usize] = val,
            0xFF40 => println!("system->gpu->control = value;"),
            0xFF42 => println!("system->gpu->scrollY= value;"),
            0xFF43 => println!("system->gpu->scrollX = value;"),
            0xFF46 => panic!("Copy(0xFE00, value << 8, 160);"),
            0xFF47 => panic!("system->gpu->backgroundPalette[i]"),
            0xFF48 => panic!("system->gpu->spritePalette[0][i] "),
            0xFF49 => panic!("system->gpu->spritePalette[1][i] "),
            0xFF0F => println!("system->interrupts->flags = value;"),
            0xFF00...0xFF7F => self.io[(addr - 0xff00) as usize] = val,
            0xFFFF => println!("system->interrupts->enable = value;"),
            _ => panic!("Not implemented"),
        }
    }

    fn write_u16(&mut self, addr: u16, val: u16) {
        self.write_u8(addr, (val & 0x00ff) as u8);
        self.write_u8(addr + 1, ((val & 0xff00) >> 8) as u8);
    }
}

pub trait MmuRead {
    fn read_u8(&self, addr: u16) -> u8;
    fn read_u16(&self, addr: u16) -> u16;
    fn write_u8(&mut self, addr: u16, val: u8);
    fn write_u16(&mut self, addr: u16, val: u16);
}