use crate::{cartridge::Cartridge, ppu::Ppu};

pub struct Bus {
    //ram
    ram: [u8; 2048],
    //ppu (Picture Processing Unit)
    ppu: Ppu,
    //apu (Audio Processing Unit)
    //rom
    cartridge: Option<Cartridge>
    //controller
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 2048],
            ppu: Ppu::new(),
            cartridge: None
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => { // Internal RAM and mirror
                self.ram[addr as usize & 0x07FF]
            },
            0x2000..=0x2007 => { // PPU Register
                // self.ppu.handle_opcode(addr);
                0
            },
            0x2008..=0x3FFF => { // PPU Register mirror
                //TODO
                0
            },
            0x4000..=0x4017 => { // APU and IO
                //TODO
                0
            },
            0x4018..=0x401F => { // Disabled, do nothing
                //TODO
                0
            },
            0x4020..=0x5FFF => { // Expansion ROM
                //TODO
                0
            },
            0x6000..=0x7FFF => { // SROM (Save ROM)
                //TODO
                0
            },
            0x8000..=0xFFFF => { // Cartidge space
                if let None = self.cartridge {
                    panic!("Tolong masukkan cartridgenya");
                }

                let cartridge = self.cartridge.as_mut().unwrap();
                cartridge.read_prg(addr)
            }
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => { // Internal RAM and mirror
                // self.ram[addr as usize & 0x07FF]
            },
            0x2000..=0x2007 => { // PPU Register
                //TODO

            },
            0x2008..=0x3FFF => { // PPU Register mirror
                //TODO

            },
            0x4000..=0x4017 => { // APU and IO
                //TODO

            },
            0x4018..=0x401F => { // Disabled, do nothing
                //TODO

            },
            0x4020..=0x5FFF => { // Expansion ROM
                //TODO

            },
            0x6000..=0x7FFF => { // SROM (Save ROM)
                //TODO

            },
            0x8000..=0xFFFF => { // Cartidge space

            }
        }
    }

    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }
}
