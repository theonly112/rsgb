use std::path::Path;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Cartrige {
    pub Type : CartridgeType

}

impl Cartrige {
    pub fn from_path(p : &Path) -> Cartrige {
        Cartrige { 
            Type : CartridgeType::from_u8(0)
        }
    }
}

#[derive(Debug)]
pub enum CartridgeType {
    Plain,
    MBC1,
    Unknown
}

impl CartridgeType {
    pub fn from_u8(value : u8) -> CartridgeType {
        match value {
            0 => CartridgeType::Plain,
            1 => CartridgeType::MBC1,
            _ => CartridgeType::Unknown 
        }
    }
}