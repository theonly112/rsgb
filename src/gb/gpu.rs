use gb::component::SystemComponent;
use gb::mmu::*;
use gb::interrupts::*;

use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
pub struct Status {
    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    lyc: u8,
    dma: u8,
    wy: u8,
    wx: u8,
    vbk: u8,
    bgpi: u8,
    obpi: u8,
}
#[allow(dead_code)]
impl Status {
    fn new() -> Status {
        Status {
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            wy: 0,
            wx: 0,
            vbk: 0,
            bgpi: 0,
            obpi: 0,
        }
    }

    fn display_enabled(&self) -> bool {
        self.lcdc & (0x01 << 7) > 1
    }
    fn window_tilemap(&self) -> u16 {
        if self.lcdc & (0x01 << 6) == 1 {
            0x8000
        } else {
            0x9000
        }
    }
    fn window_enabled(&self) -> bool {
        self.lcdc & (0x01 << 5) > 1
    }

    fn bg_tile_data(&self) -> u16 {
        if self.lcdc & (0x01 << 4) > 1 {
            0x8000
        } else {
            0x8800
        }
    }
    fn bg_tilemap(&self) -> u16 {
        if self.lcdc & (0x01 << 3) > 1 {
            0x9C00
        } else {
            0x9800
        }
    }

    fn ob_size(&self) -> bool {
        self.lcdc & (0x01 << 2) > 0
    }
    fn ob_enabled(&self) -> bool {
        self.lcdc & (0x01 << 1) > 0
    }
    fn bg_enabled(&self) -> bool {
        self.lcdc & (0x01 << 0) == 1
    }
}

pub struct Gpu {
    mode: GpuMode,
    tick: i32,
    last_ticks: i32,
    background_palette: [Color; 4],
    sprite_palette: [Color; 8],
    pub framebuffer: [Color; 160 * 144],
    pub tiles: [[[u8; 386]; 8]; 8],
    pub mmu: Option<Rc<RefCell<Mmu>>>,
    pub status: Status,
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
            mode: GpuMode::HBlank,
            tick: 0,
            last_ticks: 0,
            background_palette: [WHITE; 4],
            sprite_palette: [WHITE; 8],
            framebuffer: [WHITE; 160 * 144],
            tiles: [[[0; 386]; 8]; 8],
            mmu: None,
            status: Status::new(),
        }
    }

    pub fn step(&mut self, cpu_ticks: i32) {
        self.tick += cpu_ticks - self.last_ticks;
        self.last_ticks = cpu_ticks;

        match self.mode {
            GpuMode::HBlank => {
                if self.tick >= 204 {
                    self.status.ly += 1;
                    if self.status.ly == 143 {
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
                    self.status.ly += 1;
                    if self.status.ly > 153 {
                        self.status.ly = 0;
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
        if self.status.bg_enabled() {
            self.render_background();
        }
        if self.status.window_enabled() {
            self.render_window();
        }
        if self.status.ob_enabled() {
            self.render_sprites();
        }
    }

    pub fn clear_framebuffer(&mut self) {
        self.framebuffer = [WHITE; 160 * 144];
    }

    fn render_window(&self) {
        let ref mmu = self.mmu.as_ref().unwrap().borrow_mut();
        let tiles = self.status.window_tilemap();
        let map = self.status.bg_tilemap();

        let tileY = self.status.wy / 8;
        let tileYOffset = self.status.wy % 8;

        for x in 0..160 {
            if x < self.status.wx {
                continue;
            }
        }
    }

    fn render_background(&mut self) {
        let ref mmu = self.mmu.as_ref().unwrap().borrow_mut();

        let line_width = self.status.ly as i32 * 160;

        if self.status.bg_enabled() {
            let tiles = self.status.bg_tile_data();
            let map = self.status.bg_tilemap();
            let scx = self.status.scx;
            let scy = self.status.scy;

            let line_adjusted: u16 = self.status.ly as u16 + scy as u16;

            let y_32: u16 = ((line_adjusted / 8) * 32) as u16;
            let pixely: u16 = (line_adjusted % 8) as u16;
            let pixely_2: u16 = (pixely * 2) as u16;

            for x in 0..32 {
                let mut tile: i32;
                if tiles == 0x8800 {
                    tile = mmu.read_u8((map + y_32 + x) as u16) as i32;
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
                    let buffer_x = (map_offset + pixelx).wrapping_sub(scx as u16);
                    //if buffer_x < 0 {
                    //    continue;
                    //}
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
                    let color = (tmp_palette >> (pixel * 2)) & 0x03;

                    self.framebuffer[position as usize] = self.background_palette[color as usize];
                }
            }

        }
    }

    fn render_sprites(&mut self) {
        const GPU_CONTROL_TILEMAP: u8 = 8;
        let mut map_offset: u16 = if (self.status.lcdc & GPU_CONTROL_TILEMAP) != 0 {
            0x1c00
        } else {
            0x1800
        };

        map_offset += (((self.status.ly as u16 + self.status.scy as u16) & 255) >> 3) << 5;

        let mut line_offset = self.status.scx >> 3;
        let mut x = self.status.scx & 7;
        let y = self.status.ly.wrapping_add(self.status.scy) & 7;

        let mut pixel_offset: u16 = self.status.ly as u16 * 160;

        const VRAM_OFFSET: u16 = 0x8000;

        let mut tile: u8 = 0;

        let mut scanline_row: [u8; 160] = [0; 160];

        {
            let ref mmu = self.mmu.as_ref().unwrap().borrow_mut();
            tile = mmu.read_u8(VRAM_OFFSET + map_offset + line_offset as u16);
            for i in 0..160 {
                let color = self.tiles[x as usize][y as usize][tile as usize];
                scanline_row[i] = color;

                pixel_offset += 1;
                x += 1;

                if x == 8 {
                    x = 0;
                    line_offset = (line_offset + 1) & 31;
                    tile = mmu.read_u8(VRAM_OFFSET + map_offset as u16 + line_offset as u16);
                }
            }
        }



        for i in 0..40 {
            let sprite = Sprite::from_index(&mut self.mmu.as_ref().unwrap().borrow_mut(), i);

            pixel_offset = (self.status.ly as i16 * 160 + sprite.x) as u16;

            if sprite.y <= self.status.ly as i16 && (sprite.y + 8) > self.status.ly as i16 {
                let palette = sprite.palette();
                let tile_row: usize;
                if sprite.flip_x() {
                    tile_row = (7 - (self.status.ly as i16 - sprite.y)) as usize;
                } else {
                    tile_row = (self.status.ly as i16 - sprite.y) as usize;
                }

                for xx in 0..8 {
                    // todo priority
                    if sprite.x + xx >= 0 && sprite.x + xx < 160 {
                        let color: u8;
                        if sprite.flip_y() {
                            color = self.tiles[tile_row as usize][(7 - xx) as usize][sprite.tile_number as usize];
                        } else {
                            color = self.tiles[tile_row as usize][xx as usize][sprite.tile_number as usize];
                        }

                        if color > 0 && sprite.above_bg() {
                            let palette_offset: usize = palette * 4;
                            self.framebuffer[pixel_offset as usize].r =
                                self.sprite_palette[color as usize + palette_offset].r;
                            self.framebuffer[pixel_offset as usize].g =
                                self.sprite_palette[color as usize + palette_offset].g;
                            self.framebuffer[pixel_offset as usize].b =
                                self.sprite_palette[color as usize + palette_offset].b;
                        }
                        pixel_offset = pixel_offset + 1;
                    }
                }
            }
        }
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

struct Sprite {
    x: i16,
    y: i16,
    tile_number: u8,
    options: u8,
}

impl Sprite {
    fn from_index(mmu: &mut Mmu, index: u16) -> Sprite {
        const OAM_OFFSET: u16 = 0xFE00;
        const SPRITE_SIZE: u16 = 4;
        let sprite_y = mmu.read_u8(OAM_OFFSET + index * SPRITE_SIZE) as i16 - 16;
        let sprite_x = mmu.read_u8(OAM_OFFSET + index * SPRITE_SIZE + 1) as i16 - 8;
        let sprite_tilenumber = mmu.read_u8(OAM_OFFSET + index * SPRITE_SIZE + 2);
        let sprite_options = mmu.read_u8(OAM_OFFSET + index * SPRITE_SIZE + 3);

        Sprite {
            x: sprite_x,
            y: sprite_y,
            tile_number: sprite_tilenumber,
            options: sprite_options,
        }

    }

    fn above_bg(&self) -> bool {
        self.options & (0x01 << 7) == 0
    }

    fn flip_x(&self) -> bool {
        self.options & 0x20 == 0x20
    }

    fn flip_y(&self) -> bool {
        self.options & 0x40 == 0x40
    }

    fn palette(&self) -> usize {
        if self.options & 0x10 != 0 { 1 } else { 0 }
    }
}