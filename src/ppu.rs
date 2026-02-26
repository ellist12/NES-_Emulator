use std::fmt;

pub struct Ppu {
    ppuctrl: u8, // tempat CPU mengatur PPU
    ppumask: u8, // tempat CPU mengatur setting visual
    ppustatus: u8
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ppu")
        .field("ppuctrl", &format_args!("{:08b} [{}] [${:x}]", self.ppuctrl, self.ppuctrl, self.ppuctrl))
        .field("ppumask", &format_args!("{:08b} [{}] [${:x}]", self.ppumask, self.ppumask, self.ppumask))
        .field("ppustatus", &format_args!("{:08b} [{}] [${:x}]", self.ppustatus, self.ppustatus, self.ppustatus))
        .finish()
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0
        }
    }

    pub fn handle_write(&mut self, addr: u16, val: u8) {
        if addr == 0x2000 {
            self.ppuctrl = val;
        }
    }

    pub fn handle_read(&self, addr: u16) -> u8 {
        if addr == 0x2002 {
            self.ppustatus
        } else {
            println!("PPU address {} not implemented", addr);
            0
        }
    }
}

