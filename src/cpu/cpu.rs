use std::fmt;

use crate::{bus::Bus, cpu::instructions::{bpl::BPL, jsr::JSR, lda::LDA, ldy::LDY}, mochanes::Region};

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
                // PHA (Push accumulator on stack)
                // Menyimpan sementara nilai dari register A ke stack, sehingga accumulator bisa digunakan untuk
                // keperluan lain tanpa kehilangan nilai awalnya
                // Ukuran opcode : 1 byte
                // Jumlah cycle : 3 cycle
                // Contoh kode assembly : PHA
                // Artinya : Push nilai register A ke stack
                println!("PHA");
                bus.write(0x0100 + self.sp as u16, self.a);
                self.sp = self.sp.wrapping_sub(1);
                self.cycle += 3;
                3
            }
            0x4c => {
                // JMP (Jump)
                // Melakukan jump (mengganti nilai program counter) ke value yang ditetapkan di 2 byte berikutnya.
                // Opcode ini mirip seperti JSR, tapi tidak menyimpan lokasi pc awal di stack
                // Ukuran opcode : 3 byte
                // Jumlah cycle : 3
                // Contoh kode assembly : JMP $0020
                // Artinya : Jump ke address $0020
                let lo = bus.read(self.pc) as u16;
                let hi = bus.read(self.pc.wrapping_add(1)) as u16;
                let addr = (hi << 8) | lo;
                println!("JMP ${:x}", addr);
                self.pc = addr;
                self.cycle += 3;
                3
            }
            0x78 => {
                // SEI (Set Interrupt Flag)
                // Nyalakan bit flag interrupt di status (0b00000100)
                // Jumlah cycle : 2
                println!("SEI");
                self.status = self.status | 0b00000100;
                self.cycle += 2;
                2
            }
            0x85 => {
                // STA (Store Accumulator) Zero Page
                // Store nilai dari register A, ke ram bagian ZEROPAGE yang addressnya di specify di byte berikutnya
                // Ukuran Opcode : 2 byte
                // Jumlah cycle : 3 cycle
                // Contoh kode assembly : STA $02
                // Artinya : simpan nilai dari register A, ke bagian ram ZEROPAGE dengan address $02 (simpan ke $0002)
                let param = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let addr = param as u16;
                println!("STA ${:x}", param);
                bus.write(addr, self.a);
                self.cycle += 3;
                3
            }
            0xD8 => {
                // CLD (Clear Decimal Mode)
                // Nyalakan bit flag decimal mode di status (0b00001000)
                // Jumlah cycle : 2
                println!("CLD");
                self.status = self.status | 0b00001000;
                self.cycle += 2;
                2
            }
            0x29 => {
                // AND Immidiate
                // Lakukan operasi logic AND antara nilai di register A dan
                // angka di byte berikutnya, hasilnya dimasukan ke register A
                // Ukuran Opcode : 2 byte
                // Jumlah cycle : 2
                // Contoh kode assembly : AND #$80
                // Artinya : lakukan operasi biner AND, antara value di register A dan value di byte berikutnya,
                //           lalu masukkan hasilnya ke register a
                let param = bus.read(self.pc);
                println!("AND #${:x}", param);
                self.pc = self.pc.wrapping_add(1);
                self.a = self.a & param;
                self.update_zero_and_negative_flags(self.a);
                self.cycle += 2;
                2
            }
            0x88 => {
                // DEY : Decrease Y Register, kurangi nilai di register y sebesar 1
                // Ukuran Opcode : 1 byte
                // Jumlah cycle : 2
                // Contoh kode assembly : DEY
                // Artinya : Kurangi nilai di register Y sebesar 1
                self.y = self.y.wrapping_sub(1);
                println!("DEY");
                self.update_zero_and_negative_flags(self.y);
                self.cycle += 2;
                2
            }
            0x91 => {
                // STA (Indirect), Y : Lihat angka di alamat ram ZEROPAGE yang ditunjuk oleh byte berikutnya,
                //                     baca 2 byte dari alamat itu untuk mendapatkan sebuah alamat baru,
                //                     alamat baru itu kemudian tambahkan dengan value yang ada di register Y
                //                     simpan nilai register A ke alamat baru yang sudah ditambahkan dengan value
                //                     di register Y tersebut
                // Ukuran Opcode : 2 byte
                // Jumlah cycle : 6
                // Contoh kode assembly : STA ($20), Y
                // Artinya :
                //   1. Lihat angka yang ditunjuk oleh byte berikutnya, misal $20
                //   2. Baca value dari byte di alamat $20 dan $21, lalu gabungkan jadi satu alamat di satu variabel u16 baru, misal $8000
                //   3. Ambil nilai di register Y, misal $05
                //   4. Jumlahkan keduanya, $8000 + $0005 = $8005
                //   5. Tulis value di register A ke alamat $8005
                let param = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(param) as u16;
                let hi = bus.read(param + 1) as u16;
                let addr_to_add = (hi << 8) | lo;
                let addr = addr_to_add + self.y as u16;
                println!("STA (${}), Y", param);
                bus.write(addr, self.a);
                self.cycle += 6;
                6
            }
            0x9A => {
                // TXS: Transfer X to Stack Pointer
                // Pindah data dari register X ke stack pointer
                // Jumlah cycle : 2
                println!("TXS");
                self.sp = self.x;
                self.cycle += 2;
                2
            }
            0x84 => {
                // STY Zeropage: setor data ke bagian ram ZEROPAGE di alamat yang di specify di 1 byte berikutnya
                //               bagian ZEROPAGE di ram punya rentang dari $0000 - $00FF
                // Ukuran Opcode : 2 byte
                // Jumlah cycle : 3
                // Contoh kode assembly : STY $10
                // Artinya: Tulis value yang ada di register Y, ke address $10 di ram bagian ZEROPAGE ($0010)
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                bus.write(addr as u16, self.y);
                println!("STY ${:x}", addr);
                self.cycle += 3;
                3
            }
            0x8D => {
                // STA Absolute: Tulis nilai dari register A, ke alamat memori yang ditentukan
                // Ukuran Opcode: 3 byte,
                // Jumlah cycle : 4
                // Contoh kode assembly : STA $2000 [8D 00 20]
                // Artinya: tulis yang ada di register A ke address 2000
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);

                let addr = (hi << 8) | lo;
                println!("STA ${:x}", addr);
                bus.write(addr, self.a);
                self.cycle += 4;
                4
            }
            0xA0 => {
                LDY::immideate(self, bus)
            }
            0xA2 => {
                // LDX Immideate: Ambil byte berikutnya, taruh di register X
                // Ukuran Opcode : 2 byte
                // Jumlah cycle : 2
                // Contoh kode assembly : LDX #$10 [A2 10]
                // Artinya : ambil angka di byte berikutnya (10), dan masukkan ke register X
                let param = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.x = param;
                println!("LDX #${:x}", param);
                self.update_zero_and_negative_flags(self.x);
                self.cycle += 2;
                2
            }
            0xA4 => {
                LDY::zeropage(self, bus)
            }
            0xA5 => {
                LDA::zeropage(self, bus)
            }
            0xA9 => {
                LDA::immideate(self, bus)
            }
            0xAC => {
                LDY::absolute(self, bus)
            }
            0xAD => {
                LDA::absolute(self, bus)
            }
            0xC6 => {
                // DEC (Decrement Memory) Zero Page
                // Ambil nilai di sebuah alamat memory ZEROPAGE yang di specify di byte berikutnya setelah opcode, menguranginya dengan 1,
                // menyimpan kembali hasilnya ke alamat tersebut
                // Ukuran Opcode : 2 byte
                // Jumlah cycle : 5 cycle
                let param = bus.read(self.pc);
                println!("DEC ${:x}", param);
                self.pc = self.pc.wrapping_add(1);
                let old_data = bus.read(param as u16);
                let new_data = old_data.wrapping_sub(1);
                bus.write(param as u16, new_data);
                self.update_zero_and_negative_flags(new_data);
                self.cycle += 5;
                5
            }
            0xD0 => {
                // BNE (Branch if Not Equal)
                // Melompat ke baris kode lain jika hasil operasi sebelumnya tidak 0, jumlah lompatan tergantung dengan 1 byte berikutnya
                // Untuk BNE, angka di 1 byte berikutnya harus kita konversi dulu menjadi signed integer i8 sebelum kita operasikan
                // Ukuran Opcode : 2 byte
                // Jumlah cycle :
                //      1. 2 jika kondisi tidak terpenuhi
                //      2. 3 jika kondisi terpenuhi dan tidak melewati *page boundary*
                //      3. 4 jika kondisi terpenuhi dan melewati *page_boundary*
                // Contoh kode assembly : BNE $05
                // Artinya : Jika bit flag zero di register status == 0, lompat 5 byte kedepan
                let bytes_to_jump = bus.read(self.pc);
                println!("BNE ${:x}", bytes_to_jump);
                self.pc = self.pc.wrapping_add(1);
                let mut cycle = 2;

                if (self.status & 0b00000010) == 0 {
                    cycle += 1;
                    let offset = bytes_to_jump as i8;
                    let old_pc = self.pc;
                    let new_pc = self.pc.wrapping_add_signed(offset as i16);
                    if (old_pc & 0xFF00) != (new_pc & 0xFF00) {
                        cycle += 1;
                    }
                    self.pc = new_pc;
                }
                self.cycle += cycle;
                cycle
            }
            0xF0 => {
                // BEQ (Branch if Equal/ Branch if Zero)
                // Melompat ke baris kode lain jika hasil operasi sebelumnya adalah 0, jumlah lompatan tergantung dengan 1 byte berikutnya.
                // Untuk BEQ, angka di 1 byte berikutnya harus kita konversi dulu menjadi signed integer i8 sebelum kita operasikan
                // Ukuran Opcode : 2 byte
                // Jumlah cycle :
                //      1. 2 jika kondisi tidak terpenuhi
                //      2. 3 jika kondisi terpenuhi dan tidak melewati *page boundary*
                //      3. 4 jika kondisi terpenuhi dan melewati *page_boundary*
                // Contoh kode assembly : BEQ $05
                // Artinya : Jika bit flag zero di register status == 1, lompat 5 byte kedepan
                let bytes_to_jump = bus.read(self.pc);
                println!("BEQ {:x}", bytes_to_jump);
                self.pc = self.pc.wrapping_add(1);
                let mut cycle = 2;

                if (self.status & 0b00000010) != 0 {
                    cycle += 1;
                    let offset = bytes_to_jump as i8;
                    let old_pc = self.pc;
                    let new_pc = self.pc.wrapping_add_signed(offset as i16);
                    if (old_pc & 0xFF00) != (new_pc & 0xFF00) {
                        // Terjadi page crossing, tambah 1 cycle
                        cycle += 1;
                    }
                    self.pc = new_pc;
                }
                self.cycle += cycle;
                cycle
            }
            _ => {
                panic!("Opcode {:02x} belum diimplementasi",opcode);
                0
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
