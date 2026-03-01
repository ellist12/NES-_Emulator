use std::fmt;

use crate::{bus::Bus, cpu::instructions::{and::AND, beq::BEQ, bne::BNE, bpl::BPL, cld::CLD, dec::DEC, dey::DEY, jmp::JMP, jsr::JSR, lda::LDA, ldx::LDX, ldy::LDY, pha::PHA, sei::SEI, sta::STA, sty::STY, txs::TXS}, mochanes::Region};

pub struct Cpu {
    // Register Utama
    pub(crate) a: u8, // Accumulator
    pub(crate) x: u8, // Index X
    pub(crate) y: u8, // Index Y

    // Register Spesial
    pub(crate) sp: u8,    // Special Register
    pub(crate) pc: u16,   // Program Counter
    pub(crate) status: u8, // Status register

    pub(crate) cycle: u16, // Untuk menghitung cycle CPU
    max_cycle: u16 // Maximum cpu cycle yang dijalankan dalam 1 frame
}

// // Bikin format debug cpu custom, biar bisa nampilin binary nya
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
         .field("a", &format_args!("{:08b} [{}] [${:x}]", self.a, self.a, self.a))
         .field("x", &format_args!("{:08b} [{}] [${:x}]", self.x, self.x, self.x))
         .field("y", &format_args!("{:08b} [{}] [${:x}]", self.y, self.y, self.y))
         .field("sp", &format_args!("{:08b} [{}] [${:x}]", self.sp, self.sp, self.sp))
         .field("pc", &format_args!("{:016b} [{}] [${:x}]", self.pc, self.pc, self.pc))
         .field("status", &format_args!("{:08b} [{}] [${:x}]", self.status, self.status, self.status))
         .finish()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { 
            a: 0, 
            x: 0, 
            y: 0, 
            sp: 0xFD,
            pc: 0, 
            status: 0x24,
            cycle: 0,
            max_cycle: 0
        }
    }

    // Fungsi reset ini dijalankan pertama kali saat CPU menyala
    pub fn reset(&mut self, bus: &mut Bus) {
        //1. Ambil lowbyte dari address 0xFFFC
        let low_byte = bus.read(0xFFFC) as u16;
        //2. Ambil highbyte dari address 0xFFFD
        let high_byte = bus.read(0xFFFD) as u16;
        //3. Gabung keduanya, highbytenya digeser ke kiri 8 kali, lalu digabung dengan lowbyte buat dapetin PC (program counter)
        //   pertama
        self.pc = (high_byte << 8) | low_byte;
        // bus.write(0x8000, 0);

        println!("{:b}", self.pc);
    }

    pub fn set_max_cycle(&mut self, region: &Region) {
        self.max_cycle = if *region == Region::NTSC {
            29780
        } else {
            35464
        }
    }

    // Fungsi ini memiliki 3 bagian: FETCH, DECODE, EXECUTE, ini adalah fungsi inti dari emulatornya, di run setelah semua 
    // komponen emulator udah siap
    pub fn step(&mut self, bus: &mut Bus) -> u16 {
        //1. FETCH: Ambil intruksi dari alamat memori yang disimpan di program counter, lalu majukan program counter
        let opcode = bus.read(self.pc);
        println!("Opcode : {:x}, {}", opcode, self.pc);
        self.pc += 1;

        if self.cycle > self.max_cycle {
            self.cycle = 0;
        }

        //2. DECODE & EXECUTE: Cek opcodenya apa
        match opcode {
            0x10 => {
                BPL::branch(self, bus)
            }
            0x20 => {
                JSR::jump(self, bus)
            }
            0x48 => {
                PHA::push(self, bus)
            }
            0x4c => {
                JMP::jump(self, bus)
            }
            0x78 => {
                SEI::set(self)
            }
            0x85 => {
                STA::zeropage(self, bus)
            }
            0xD8 => {
                CLD::clear(self)
            }
            0x29 => {
                AND::immediate(self, bus)
            }
            0x88 => {
                DEY::decrease(self)
            }
            0x91 => {
                STA::indirect_y(self, bus)
            }
            0x9A => {
                TXS::transfer(self)
            }
            0x84 => {
                STY::zeropage(self, bus)
            }
            0x8D => {
                STA::absolute(self, bus)
            }
            0xA0 => {
                LDY::immedeate(self, bus)
            }
            0xA2 => {
                LDX::immediate(self, bus)
            }
            0xA4 => {
                LDY::zeropage(self, bus)
            }
            0xA5 => {
                LDA::zeropage(self, bus)
            }
            0xA9 => {
                LDA::immedeate(self, bus)
            }
            0xAC => {
                LDY::absolute(self, bus)
            }
            0xAD => {
                LDA::absolute(self, bus)
            }
            0xC6 => {
                DEC::zeropage(self, bus)
            }
            0xD0 => {
                BNE::branch(self, bus)
            }
            0xF0 => {
                BEQ::branch(self, bus)
            }
            _ => {
                panic!("Opcode {:02x} belum diimplementasi",opcode);
            }
        }
    }

    // Fungsi untuk cek, apakah angka yang ingin dimasukkan ke register itu negatif atau zero,
    // kalau negatif, nyalakan negative flag, kalau zero, nyalakan zero flag, kalau sebaliknya,
    // matikan keduanya
    pub(crate) fn update_zero_and_negative_flags(&mut self, result: u8) {
        // cek angka 0 apa nggak
        if result == 0 {
            self.status = self.status | 0b00000010;
        } else {
            self.status = self.status & 0b11111101;
        }

        // cek angka negatif atau nggak
        // untuk cek negatif, kita tinggal mengecek bit ke 7 (pertama dari pandangan kita) itu nilainya 1 apa nggak
        // kalau nyala kita set negatif flag jadi 1, kalau 0 kita set negatif flag jadi 0. Kenapa cek nya pakai bit
        // nggak cek langsung < 0? Karena CPU dan binary number tidak mengenal konsep angka negatif. CPU menyimpan angka
        // dalam bentuk unsigned integer u8 (0 - 255) nah kita bisa nyalakan bit flag negatif di register status untuk bilang
        // oh hasil perhitungan sebelumnya itu bisa jadi angka negatif lho. Dengan menyalakan bit flag, kita bisa mengubah representasi
        // angka u8 tadi jadi signed integer i8 (-128 - 127). Lalu its up to the programmer to decide angka itu
        // dianggap sebagai i8 atau u8;
        if result & 0b10000000 != 0  {
            self.status = self.status | 0b10000000;
        } else {
            self.status = self.status & 0b01111111;
        }
    }
}
