use gb::component::SystemComponent;
use gb::mmu::*;
use gb::interrupts::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Gpu {
    scanline: u8,
    scroll_x: u8,
    scroll_y: u8,
    pub control: u8,
    mode: GpuMode,
    tick: i32,
    last_ticks: i32,
    background_palette: [Color; 4],
    sprite_palette: [Color; 8],
    pub framebuffer: [Color; 160 * 144],
    pub tiles: [[[u8; 386]; 8]; 8],
    pub mmu: Option<Rc<RefCell<Mmu>>>,
}

enum GpuMode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}


#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

static WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

static PALETTE: [Color; 4] = [Color {
                                  r: 255,
                                  g: 255,
                                  b: 255,
                              },
                              Color {
                                  r: 192,
                                  g: 192,
                                  b: 192,
                              },
                              Color {
                                  r: 96,
                                  g: 96,
                                  b: 96,
                              },
                              Color {
                                  r: 0x00,
                                  g: 0x00,
                                  b: 0x00,
                              }];

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            scanline: 0,
            scroll_x: 0,
            scroll_y: 0,
            control: 0,
            mode: GpuMode::HBlank,
            tick: 0,
            last_ticks: 0,
            background_palette: [WHITE; 4],
            sprite_palette: [WHITE; 8],
            framebuffer: [WHITE; 160 * 144],
            tiles: [[[0; 386]; 8]; 8],
            mmu: None,
        }
    }

    pub fn step(&mut self, cpu_ticks: i32) {
        self.tick += cpu_ticks - self.last_ticks;
        self.last_ticks = cpu_ticks;

        match self.mode {
            GpuMode::HBlank => {
                if self.tick >= 204 {
                    self.scanline += 1;
                    if self.scanline == 143 {
                        self.mode = GpuMode::VBlank;
                        let ref mut mmu = self.mmu.as_ref().unwrap().borrow_mut();
                        let enable = mmu.read_u8(INTERRUPT_ENABLE);
                        let mut flag = mmu.read_u8(INTERRUPT_FLAG);
                        if enable & VBLANK != 0 {
                            flag |= VBLANK;
                            mmu.write_u8(INTERRUPT_FLAG, flag);
                        }
                    } else {
                        self.mode = GpuMode::OAM;
                    }
                    self.tick -= 204;
                }
            }
            GpuMode::VBlank => {
                if self.tick >= 456 {
                    self.scanline += 1;
                    if self.scanline > 153 {
                        self.scanline = 0;
                        self.mode = GpuMode::OAM
                    }
                    self.tick -= 456;
                }
            }
            GpuMode::OAM => {
                if self.tick >= 80 {
                    self.mode = GpuMode::VRAM;
                    self.tick -= 80;
                }
            }
            GpuMode::VRAM => {
                if self.tick >= 172 {
                    self.mode = GpuMode::HBlank;
                    self.render_scanline();
                    self.tick -= 172;
                }
            }
        }
    }

    fn render_scanline(&mut self) {
        let scanline = self.scanline;
        self.render_background(scanline);
        self.render_window();
        self.render_sprites();
    }

    fn render_window(&self) {
        // let ref mmu = match (self.mmu) {
        //   Some(ref mmu) => mmu,
        //    None => panic!("..."),
        // };

        let ref mmu = self.mmu.as_ref().unwrap();
        // mmu.borrow_mut();
        // TODO:
    }

    fn is_bit_set(&self, value: u8, bit: u8) -> bool {
        (value & (0x01 << bit)) != 0
    }

    fn render_background(&mut self, line: u8) {
        // TODO:
        let ref mmu = self.mmu.as_ref().unwrap().borrow_mut();

        let lcdc = self.control;
        let line_width = line as i32 * 160;

        if self.is_bit_set(lcdc, 0) {
            let tiles: u16 = if self.is_bit_set(lcdc, 4) {
                0x8000
            } else {
                0x8800
            };

            let map: u16 = if self.is_bit_set(lcdc, 3) {
                0x9C00
            } else {
                0x9800
            };

            let scx = self.scroll_x;
            let scy = self.scroll_y;

            let line_adjusted: u16 = line as u16 + scy as u16;

            let y_32: u16 = ((line_adjusted / 8) * 32) as u16;
            let pixely: u16 = (line_adjusted % 8) as u16;
            let pixely_2: u16 = (pixely * 2) as u16;

            for x in 0..32 {
                let mut tile: i32 = 0;
                if tiles == 0x8800 {
                    tile = (mmu.read_u8((map + y_32 + x) as u16) as i32);
                    tile += 128;
                } else {
                    tile = mmu.read_u8((map + y_32 + x) as u16) as i32;
                }

                let map_offset = x * 8;
                let tile_16 = tile * 16;
                let final_pixely_2 = pixely_2;
                let tile_address = tiles as u16 + tile_16 as u16 + final_pixely_2 as u16;

                let byte1 = mmu.read_u8(tile_address as u16);
                let byte2 = mmu.read_u8((tile_address + 1) as u16);
                for pixelx in 0..8 {
                    // TODO this -scx seems weird
                    let buffer_x = map_offset + pixelx - scx as u16;
                    if buffer_x < 0 {
                        continue;
                    }
                    if buffer_x > 160 {
                        continue;
                    }

                    let pixelx_pos = pixelx;

                    let mut pixel = if (byte1 & (0x1 << (7 - pixelx_pos))) != 0 {
                        1
                    } else {
                        0
                    };
                    pixel |= if (byte2 & (0x1 << (7 - pixelx_pos))) != 0 {
                        2
                    } else {
                        0
                    };

                    let position = line_width + buffer_x as i32;
                    let tmp_palette = mmu.read_u8(0xff47);
                    let color = ((tmp_palette >> (pixel * 2)) & 0x03);

                    self.framebuffer[position as usize] = self.background_palette[color as usize];
                }
            }

        }
    }



    fn render_sprites(&self) {
        // TODO:
    }

    pub fn get_scanline(&self) -> u8 {
        self.scanline
    }

    pub fn set_scroll_x(&mut self, value: u8) {
        self.scroll_x = value;
    }

    pub fn set_scroll_y(&mut self, value: u8) {
        self.scroll_y = value;
    }

    pub fn set_control(&mut self, value: u8) {
        self.control = value;
    }

    #[allow(dead_code)]
    pub fn set_scanline(&mut self, line: u8) {
        self.scanline = line;
    }

    pub fn update_background_palette(&mut self, val: u8) {
        for i in 0..4 {
            let index = ((val >> (i * 2)) & 3) as usize;
            self.background_palette[i] = PALETTE[index];
        }
    }
    pub fn update_sprite_palette(&mut self, index: usize, val: u8) {
        for i in 0..4 {
            let palette = ((val >> (i * 2)) & 3) as usize;
            self.sprite_palette[index * 4 + i] = PALETTE[palette];
        }
    }
}

impl SystemComponent for Gpu {
    fn reset(&mut self) {
        for i in 0..4 {
            self.background_palette[i] = PALETTE[i];
        }
        for x in 0..2 {
            for y in 0..4 {
                self.sprite_palette[x * 4 + y] = PALETTE[y];
            }
        }
    }
}