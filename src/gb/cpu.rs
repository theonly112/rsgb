pub struct Cpu {   
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { }
    }

    pub fn execute(instruction : u8) {
        
    }
}

pub struct Registers {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: 0x01B0,
            bc: 0x0013,
            de: 0x00D8,
            hl: 0x014D,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }
}