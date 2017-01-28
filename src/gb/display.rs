use gb::gpu::Color;

use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::Sdl;

use std::mem::transmute;

extern crate sdl2;
extern crate time;

pub trait Display {
    fn draw(&mut self, framebuffer: [Color; 160 * 144]);
}

pub struct SdlDisplay<'window> {
    pub renderer: Renderer<'window>,
    texture: Texture,
    #[allow(dead_code)]
    context: Sdl,
    last_frame_time: f64,
}

impl<'window> SdlDisplay<'window> {
    pub fn new(context: Sdl) -> SdlDisplay<'window> {
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem.window("rsgb", 160 * 4, 144 * 4)
            .position_centered()
            .build()
            .unwrap();
        let renderer = window.renderer()
            .accelerated()
            .build()
            .unwrap();

        let texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
            .unwrap();

        SdlDisplay {
            renderer: renderer,
            context: context,
            texture: texture,
            last_frame_time: 0f64,
        }
    }
}

impl<'window> Display for SdlDisplay<'window> {
    fn draw(&mut self, framebuffer: [Color; 160 * 144]) {
        self.print_debug_info();

        let pixels: [u8; 160 * 144 * 3] = unsafe { transmute(framebuffer) };
        self.texture.update(None, &pixels, 480).unwrap();

        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None).unwrap();
        self.renderer.present();
    }
}

impl<'window> SdlDisplay<'window> {
    fn print_debug_info(&mut self) {
        if self.last_frame_time == 0f64 {
            self.last_frame_time = time::precise_time_s();
        } else {
            let current = time::precise_time_s();
            let delta = current - self.last_frame_time;
            self.last_frame_time = current;
            println!("Frametime {}s", delta);
        }

    }
}