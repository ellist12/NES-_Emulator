use std::{fs::File, io::Read};

use crate::{cartridge::{self, Cartridge}, ppu::Ppu};

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

        let mut cartridge = Cartridge::new(prg_rom, chr_rom);

        self.cartridge = Some(cartridge);
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
}
