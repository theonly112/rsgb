mod gb;

use gb::catridge::Cartrige;
use gb::system::System;
use gb::display::*;
use gb::input::*;

use std::env;
use std::path::Path;

extern crate sdl2;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // debugging
    args.push("./TestRoms/tetris.gb".to_string());
    if args.len() < 2 {
        println!("Please provide the path to the rom as command line argument");
        return;
    }

    let context = sdl2::init().unwrap();
    let mut display = SdlDisplay::new(context.clone());
    let input = Input::new(context.clone());

    let path = Path::new(&args[1]);
    let c = Cartrige::from_path(path).unwrap();
    let mut system = System::new(c, input);
    system.run(&mut display);
}
