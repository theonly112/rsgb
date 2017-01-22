use gb::gpu::Color;

use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::pixels::PixelFormatEnum;
#[allow(unused_imports)]
use sdl2::event::Event;
#[allow(unused_imports)]
use sdl2::keyboard::Keycode;
use sdl2::*;

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
    pub fn new() -> SdlDisplay<'window> {

        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem.window("rsgb", 160, 144)
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

        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..144 {
                    for x in 0..160 {
                        let color = framebuffer[y * 160 + x];
                        let offset = y * pitch + x * 3;
                        buffer[offset + 0] = color.r;
                        buffer[offset + 1] = color.g;
                        buffer[offset + 2] = color.b;
                    }
                }
            })
            .unwrap();


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


// let mut event_pump = self.context.event_pump().unwrap();
//
// 'running: loop {
// for event in event_pump.poll_iter() {
// match event {
// Event::Quit { .. } |
// Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
// _ => {}
// }
// }
// The rest of the game loop goes here...
// }