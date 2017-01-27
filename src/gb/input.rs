use sdl2::*;
use sdl2::keyboard::Scancode;
extern crate sdl2;
pub struct Input {
    event_pump: EventPump,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Input {
    pub fn new(context: Sdl) -> Input {
        let event_pump = context.event_pump().unwrap();
        Input {
            event_pump: event_pump,
            a: false,
            b: false,
            start: false,
            select: false,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
    pub fn step(&mut self) {
        self.event_pump.pump_events();
        let state = sdl2::keyboard::KeyboardState::new(&self.event_pump);

        self.left = state.is_scancode_pressed(Scancode::Left);
        self.right = state.is_scancode_pressed(Scancode::Right);
        self.up = state.is_scancode_pressed(Scancode::Up);
        self.down = state.is_scancode_pressed(Scancode::Down);
        self.a = state.is_scancode_pressed(Scancode::A);
        self.b = state.is_scancode_pressed(Scancode::B);
        self.start = state.is_scancode_pressed(Scancode::Space);
        self.select = state.is_scancode_pressed(Scancode::Backslash);
    }

    pub fn get_keys1(&self) -> u8 {
        let mut keys1 = 0u8;

        keys1 |= if self.a { 0 } else { 1 << 3 };
        keys1 |= if self.b { 0 } else { 1 << 2 };
        keys1 |= if self.select { 0 } else { 1 << 1 };
        keys1 |= if self.start { 0 } else { 1 << 0 };

        return keys1;
    }
    pub fn get_keys2(&self) -> u8 {
        let mut keys1 = 0u8;

        keys1 |= if self.right { 0 } else { 1 << 3 };
        keys1 |= if self.left { 0 } else { 1 << 2 };
        keys1 |= if self.up { 0 } else { 1 << 1 };
        keys1 |= if self.down { 0 } else { 1 << 0 };

        return keys1;
    }
}