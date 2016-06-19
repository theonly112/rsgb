mod gb;

use gb::catridge::Cartrige;
use gb::system::System;

use std::env;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // debugging
    args.push("./TestRoms/tetris.gb".to_string());

    if args.len() < 2 {
        println!("Please provide the path to the rom as command line argument");
        return;
    }
    let path = Path::new(&args[1]);
    let c = Cartrige::from_path(path).unwrap();
    let mut system = System::new(c);
    system.run();
}
