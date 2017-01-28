use gb::catridge::Cartrige;
use gb::gpu::Gpu;
use gb::input::Input;
use gb::component::SystemComponent;

use std::rc::Rc;
use std::cell::RefCell;

extern crate rand;
use self::rand::*;

pub struct Mmu {
    cart: Rc<Cartrige>,
    gpu: Rc<RefCell<Gpu>>,
    input: Rc<RefCell<Input>>,
    wram: [u8; 0x2000],
    hram: [u8; 0x0080],
    io: [u8; 0x0100],
    oam: [u8; 0x100],
    vram: [u8; 0x2000],
    sram: [u8; 0x2000],
    interupt_enable: u8,
    interupt_flag: u8,
}

impl Mmu {
    pub fn new(cart: Rc<Cartrige>, gpu: Rc<RefCell<Gpu>>, input: Rc<RefCell<Input>>) -> Mmu {
        Mmu {
            cart: cart,
            wram: [0; 0x2000],
            hram: [0; 0x0080],
            io: [0; 0x0100],
            oam: [0; 0x0100],
            vram: [0; 0x2000],
            sram: [0; 0x2000],
            gpu: gpu,
            input: input,
            interupt_enable: 0,
            interupt_flag: 0,
        }
    }


    fn read_input(&self) -> u8 {
        if self.io[0x00] & 0x20 == 0 {
            // let value = 0xC0 | 15 | 0x10;
            let value = 0xC0 | self.input.borrow().get_keys1() | 0x10;
            return value;
        }
        if self.io[0x00] & 0x10 == 0 {
            // let value = 0xC0 | 13 | 0x20;
            let value = 0xC0 | self.input.borrow().get_keys2() | 0x20;
            return value;
        }
        if self.io[0x00] & 0x30 == 0 {
            return 0xff;
        }
        return 0;
    }

    fn update_background_palette(&mut self, value: u8) {
        self.gpu.borrow_mut().update_background_palette(value);
        self.io[0x47] = value;
    }
    fn update_sprite_palette(&mut self, index: usize, value: u8) {
        self.gpu.borrow_mut().update_sprite_palette(index, value);
        if index == 0 {
            self.io[0x48] = value;
        } else {
            self.io[0x49] = value;
        }
    }
    fn vram_write(&mut self, addr: u16, val: u8) {
        self.vram[(addr - 0x8000) as usize] = val;
        if addr <= 0x97ff {
            self.update_tile(addr);
        }
    }

    fn update_tile(&mut self, addr: u16) {
        let address = addr & 0x1ffe;
        let tile = (address >> 4) & 511;
        let y = (address >> 1) & 7;

        const VRAM_OFFSET: u16 = 0x8000;
        let memory_address = address + VRAM_OFFSET;

        for x in 0..8 {
            let bit_index = 1 << (7 - x);
            let a = if self.read_u8(memory_address) & bit_index > 0 {
                1
            } else {
                0
            };
            let b = if self.read_u8(memory_address + 1) & bit_index > 0 {
                2
            } else {
                0
            };
            self.gpu.borrow_mut().tiles[y as usize][x as usize][tile as usize] = a + b;
        }
    }

    fn copy(&mut self, value: u8) {
        const LENGTH: u16 = 160;
        const DESTINATION: u16 = 0xFE00;
        let value = value as u16;
        let source = (value << 8) as u16;
        for i in 0..LENGTH {
            let value = self.read_u8(source + i);
            self.write_u8(DESTINATION + i, value);
        }
    }
}

pub trait MmuRead {
    fn read_u8(&self, addr: u16) -> u8;
    fn read_u16(&self, addr: u16) -> u16;
    fn write_u8(&mut self, addr: u16, val: u8);
    fn write_u16(&mut self, addr: u16, val: u16);
}

impl MmuRead for Mmu {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF => self.cart.rom[addr as usize],
            0xA000...0xBFFF => self.sram[(addr - 0xA000) as usize],
            0x8000...0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xC000...0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000...0xFDFF => self.wram[(addr - 0xE000) as usize],
            0xFE00...0xFEFF => self.oam[(addr - 0xFE00) as usize],
            0xFF04 => rand::thread_rng().gen::<u8>(), // TODO rand
            0xFF40 => self.gpu.borrow().control,
            0xFF42 => self.gpu.borrow().get_scroll_y(),
            0xFF43 => self.gpu.borrow().get_scroll_x(),
            0xFF44 => self.gpu.borrow().get_scanline(),
            0xFF00 => self.read_input(),
            0xFF0F => self.interupt_flag,
            0xFFFF => self.interupt_enable,
            0xFF80...0xFFFE => self.hram[(addr - 0xff80) as usize],
            0xFF00...0xFF7F => self.io[(addr - 0xff00) as usize],
            _ => panic!("invalid addr"),
        }
    }

    fn read_u16(&self, addr: u16) -> u16 {
        self.read_u8(addr) as u16 + ((self.read_u8(addr + 1) as u16) << 8)
    }

    fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x7FFF => return,//panic!("mbc not implemented"),
            0x8000...0x9FFF => self.vram_write(addr, val),
            0xA000...0xBFFF => self.sram[(addr - 0xA000) as usize] = val,
            0xC000...0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xE000...0xFDFF => self.wram[(addr - 0xE000) as usize] = val,
            0xFE00...0xFEFF => self.oam[(addr - 0xfe00) as usize] = val,
            0xFF80...0xFFFE => self.hram[(addr - 0xff80) as usize] = val,
            0xFF40 => self.gpu.borrow_mut().set_control(val),
            0xFF42 => self.gpu.borrow_mut().set_scroll_x(val),
            0xFF43 => self.gpu.borrow_mut().set_scroll_y(val),
            0xFF46 => self.copy(val),
            0xFF47 => self.update_background_palette(val),
            0xFF48 => self.update_sprite_palette(0, val),
            0xFF49 => self.update_sprite_palette(1, val),
            0xFF0F => self.interupt_flag = val,
            0xFF00...0xFF7F => {
                self.io[(addr - 0xff00) as usize] = val;
                if (addr == 0xff02) {
                    print!("{}", self.io[1]);
                }
            }
            0xFFFF => self.interupt_enable = val,
            _ => panic!("Not implemented"),
        }
    }

    fn write_u16(&mut self, addr: u16, val: u16) {
        self.write_u8(addr, (val & 0x00ff) as u8);
        self.write_u8(addr + 1, ((val & 0xff00) >> 8) as u8);
    }
}

impl SystemComponent for Mmu {
    fn reset(&mut self) {
        for i in 0..255 {
            self.io[i] = IO_RESET[i];
        }
        self.write_u8(0xFF05, 0);
        self.write_u8(0xFF06, 0);
        self.write_u8(0xFF07, 0);
        self.write_u8(0xFF10, 0x80);
        self.write_u8(0xFF11, 0xBF);
        self.write_u8(0xFF12, 0xF3);
        self.write_u8(0xFF14, 0xBF);
        self.write_u8(0xFF16, 0x3F);
        self.write_u8(0xFF17, 0x00);
        self.write_u8(0xFF19, 0xBF);
        self.write_u8(0xFF1A, 0x7A);
        self.write_u8(0xFF1B, 0xFF);
        self.write_u8(0xFF1C, 0x9F);
        self.write_u8(0xFF1E, 0xBF);
        self.write_u8(0xFF20, 0xFF);
        self.write_u8(0xFF21, 0x00);
        self.write_u8(0xFF22, 0x00);
        self.write_u8(0xFF23, 0xBF);
        self.write_u8(0xFF24, 0x77);
        self.write_u8(0xFF25, 0xF3);
        self.write_u8(0xFF26, 0xF1);
        self.write_u8(0xFF40, 0x91);
        self.write_u8(0xFF42, 0x00);
        self.write_u8(0xFF43, 0x00);
        self.write_u8(0xFF45, 0x00);
        self.write_u8(0xFF47, 0xFC);
        self.write_u8(0xFF48, 0xFF);
        self.write_u8(0xFF49, 0xFF);
        self.write_u8(0xFF4A, 0x00);
        self.write_u8(0xFF4B, 0x00);
        self.write_u8(0xFFFF, 0x00);

    }
}

static IO_RESET: [u8; 0x100] =
    [0x0F, 0x00, 0x7C, 0xFF, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
     0x01, 0x80, 0xBF, 0xF3, 0xFF, 0xBF, 0xFF, 0x3F, 0x00, 0xFF, 0xBF, 0x7F, 0xFF, 0x9F, 0xFF,
     0xBF, 0xFF, 0xFF, 0x00, 0x00, 0xBF, 0x77, 0xF3, 0xF1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
     0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
     0x00, 0xFF, 0x00, 0xFF, 0x91, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00,
     0x00, 0xFF, 0x7E, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3E, 0xFF, 0xFF, 0xFF,
     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0,
     0xFF, 0xC1, 0x00, 0xFE, 0xFF, 0xFF, 0xFF, 0xF8, 0xFF, 0x00, 0x00, 0x00, 0x8F, 0x00, 0x00,
     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00,
     0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89,
     0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E,
     0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x45, 0xEC, 0x52, 0xFA,
     0x08, 0xB7, 0x07, 0x5D, 0x01, 0xFD, 0xC0, 0xFF, 0x08, 0xFC, 0x00, 0xE5, 0x0B, 0xF8, 0xC2,
     0xCE, 0xF4, 0xF9, 0x0F, 0x7F, 0x45, 0x6D, 0x3D, 0xFE, 0x46, 0x97, 0x33, 0x5E, 0x08, 0xEF,
     0xF1, 0xFF, 0x86, 0x83, 0x24, 0x74, 0x12, 0xFC, 0x00, 0x9F, 0xB4, 0xB7, 0x06, 0xD5, 0xD0,
     0x7A, 0x00, 0x9E, 0x04, 0x5F, 0x41, 0x2F, 0x1D, 0x77, 0x36, 0x75, 0x81, 0xAA, 0x70, 0x3A,
     0x98, 0xD1, 0x71, 0x02, 0x4D, 0x01, 0xC1, 0xFF, 0x0D, 0x00, 0xD3, 0x05, 0xF9, 0x00, 0x0B,
     0x00];