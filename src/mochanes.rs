use std::{fs::File, io};
use std::io::Read;

use crate::{bus::Bus, cartridge::Cartridge, cpu::Cpu, ppu::Ppu};

pub struct MochaNES {
    cpu: Cpu,
    bus: Bus,
    ppu: Ppu
}

impl MochaNES {
    pub fn new() -> Self {
        MochaNES {
            cpu: Cpu::new(),
            bus: Bus::new(),
            ppu: Ppu::new()
        }
    }

    pub fn init (&mut self) {
        self.load_rom("Donkey Kong (JU) [T-Span].nes");
        self.cpu.reset(&mut self.bus);
    }

    pub fn run(&mut self) {
        let mut input = String::new();

        for _ in 0..100 {
            io::stdin().read_line(&mut input).unwrap();
            self.cpu.step(&mut self.bus);
            println!("{:?}", self.cpu);
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).expect("File tidak ditemukan");

        //1. Siapkan buffer untuk baca header (16 byte)
        let mut header = [0u8; 16];
        file.read_exact(&mut header).expect("Gagal baca header");

        //2. Validasi bahwa file yang dibaca adalah game NES
        //   Caranya yaitu baca byte 0 - 3, apakah ada tulisan 'N', 'E', 'S', EOF apa tidak
        if header[0..4] != [b'N', b'E', b'S', 0x1A] { // 0x1A disini adalah EOF
            panic!("File ini bukan file game NES");
        }

        if (header[6] & 0b00000100) != 0 {
            println!("ada trainer");
        }

        //3. Baca ukuran cartridge program dengan membaca header byte ke 4
        //   angka di byte 4 header nanti akan kita kalikan dengan 16KB untuk mengetahui ukurannya
        let prg_banks = header[4];
        let prg_size = (prg_banks as usize) * 16 * 1024; // 16 * 1024 = 16KB

        println!("prg_size: {}, prg_banks: {}", prg_size, prg_banks);

        //4. Baca ukuran cartridge grafik dengan membaca header byte ke 5
        //   angka di byte 5 header nanti akan kita kalikan dengan 8KB untuk mengetahui ukurannya
        let chr_banks = header[5];
        let chr_size = (chr_banks as usize) * 8 * 1024; // 16 * 1024 = 16KB

        //5. Sekarang, kita bisa load data program ke memori
        let mut prg_rom = vec![0u8; prg_size];
        file.read_exact(&mut prg_rom).expect("Gagal Membaca PRG-ROM");

        // println!("FC dan FD: {:x} {:x}", prg_rom[prg_size-3], prg_rom[prg_size-1]);

        //6. Kita load juga data grafik ke memori
        let mut chr_rom = vec![0u8; chr_size];
        file.read_exact(&mut chr_rom).expect("Gagal membaca CHR-ROM");

        let cartridge = Cartridge::new(prg_rom, chr_rom);

        self.bus.set_cartridge(cartridge);
    }
}
