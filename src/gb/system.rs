// use gb::cartrige::Cartrige;
use super::catridge::Cartrige;

pub struct System {
    cartirge: Cartrige,
}

impl System {
    pub fn new(cart: Cartrige) -> System {
        System { cartirge: cart }
    }

    pub fn run(&self) {}
}