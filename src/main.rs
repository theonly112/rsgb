mod gb;

use gb::catridge::Cartrige;

use std::path::Path;

fn main() {
    println!("Hello, world!");
    let path = Path::new("");
    let c = Cartrige::from_path(path);
    println!("{:?}", c);
}
