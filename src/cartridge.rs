pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    // current_prg_rom_bank: u8,
    pub chr_rom: Vec<u8>
}

impl Cartridge {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Self {
        Cartridge { prg_rom, chr_rom}
    }

    pub fn read_prg(&mut self, addr: u16) -> u8 {
        let index = (addr as usize) - 0x8000;
        // println!("index: {}", index);
        // println!("Cartridge len : {}", self.prg_rom.len());
        self.prg_rom[((addr as usize) - 0x8000) % self.prg_rom.len()]
    }

    // pub fn write_prg_bank(&mut self, val: u8) {
    //     self.current_prg_rom_bank = val;
    // }
}