use crate::bus::Bus;

#[derive(Debug)]
pub struct Cpu {
    // Register Utama
    a: u8, // Accumulator
    x: u8, // Index X
    y: u8, // Index Y

    // Register Spesial
    sp: u8,    // Special Register
    pc: u16,   // Program Counter
    status: u8 // Status register
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { 
            a: 0, 
            x: 0, 
            y: 0, 
            sp: 0xFD,
            pc: 0, 
            status: 0x24 
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

    // Fungsi ini memiliki 3 bagian: FETCH, DECODE, EXECUTE, ini adalah fungsi inti dari emulatornya, di run setelah semua 
    // komponen emulator udah siap
    pub fn step(&mut self, bus: &mut Bus) {
        //1. FETCH: Ambil intruksi dari alamat memori yang disimpan di program counter, lalu majukan program counter
        let opcode = bus.read(self.pc);
        self.pc += 1;

        //2. DECODE & EXECUTE: Cek opcodenya apa
        match opcode {
            0x78 => {
                // SEI (Set Interrupt Flag)
                // Nyalakan bit 2 di status (0b00000100)
                self.status = self.status | 0b00000100;
            }
            0xD8 => {
                // CLD (Clear Decimal Mode)
                // Nyalakan bit 3 di status (0b00001000)
                self.status = self.status | 0b000010000;
            }
            0x8D => {
                // STA Absolute: Tulis nilai dari register A, ke alamat memori yang ditentukan
                // Instruksi 3 byte, contoh : STA $2000 (8D 00 20) di binary kodingan
                // yang artinya tulis yang ada di register A ke address 2000
                let lo = bus.read(self.pc) as u16;
                self.pc += 1;
                let hi = bus.read(self.pc) as u16;
                self.pc += 1;

                let addr = (hi << 8) | lo;
                bus.write(addr, self.a);
            }
            0xA5 => {
                // LDA Zeropage: Ambil data di alamat ram yang ada di PC
                // Instruksi 2 byte: LDA $xxxx
                let addr = bus.read(self.pc);
                let param = bus.read(addr as u16);
                self.pc += 1;
                self.a = param;
            }
            0xA9 => {
                // LDA Immediate: Ambil byte berikutnya, taruh di register A
                // Instruksi 2 byte: LDA
                let param = bus.read(self.pc);
                self.pc += 1;
                self.a = param;
            }
            0xAD => {
                // LDA Absolute: Ambil data di alamat spesifik
                let lo = bus.read(self.pc) as u16;
                self.pc += 1;
                let hi = bus.read(self.pc) as u16;
                self.pc += 1;

                self.a = ((hi << 8) | lo) as u8;
            }
            _ => {
                println!("Opcode {:02x} belum diimplementasi",opcode);
            }
        }
    }
}