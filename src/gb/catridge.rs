use std::path::Path;
use std::result::Result;
use std::fs::File;
use std::io::Error;
use std::io::Read;


#[derive(Debug, Clone)]
pub struct Cartrige {
    pub cartirge_type: CartridgeType,
    pub rom: Vec<u8>,
}

impl Cartrige {
    pub fn from_path(path: &Path) -> Result<Cartrige, Error> {
        let mut file = try!(File::open(path));
        let mut buff: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut buff));
        let c = Cartrige {
            cartirge_type: CartridgeType::from_u8(buff[0x0147]),
            rom: buff,
        };
        Ok(c)
    }
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum CartridgeType {
    Plain,
    Mbc1,
    Unknown,
}

impl CartridgeType {
    pub fn from_u8(value: u8) -> CartridgeType {
        match value {
            0 => CartridgeType::Plain,
            1 => CartridgeType::Mbc1,
            _ => CartridgeType::Unknown, 
        }
    }
}


#[test]
fn cartrige_from_u8() {
    let ct = CartridgeType::from_u8(1);
    assert!(ct == CartridgeType::Mbc1);
}