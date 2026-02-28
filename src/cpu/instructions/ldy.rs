use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct LDY;

impl LDY {
    pub fn immideate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        // LDY Immideate: Ambil byte berikutnya, taruh di register Y
        // Ukuran Opcode : 2 byte
        // Jumlah cycle : 2
        // Contoh kode assembly : LDY #$10 [A0 10]
        // Artinya : ambil angka di byte berikutnya (10), dan masukkan ke register Y
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.y = param;
        println!("LDY #${:x}", param);
        cpu.update_zero_and_negative_flags(cpu.y);
        cpu.cycle += 2;
        2
    }

    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        // LDY Zeropage: Ambil data di alamat ram bagian ZEROPAGE yang dispecify di 1 byte berikutnya
        //               bagian zeropage ram punya alamat dari $0000 - $00FF
        // Ukuran opcode: 2 byte
        // Jumlah cycle : 3
        // Contoh kode assembly : LDY $05 [A4 05]
        // Artinya : ambil angka di bagian zeropage ram dengan address $05 ($0005), masukan ke register Y
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let data = bus.read(param as u16);
        cpu.y = data;
        cpu.update_zero_and_negative_flags(cpu.y);
        cpu.cycle += 3;
        3
    }

    pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        // LDY Absolute: Ambil data di alamat yang dispecify di 2 byte berikutnya, lau masukkan ke register Y
        // Ukuran opcode : 3 byte
        // Jumlah cycle : 4
        // Contoh kode assembly : LDY $2002 [AC 02 20]
        // Artinya : ambil angka di alamat $2002, lalu masukkan ke register Y
        let lo = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let hi = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let addr = (hi << 8) | lo;
        let data = bus.read(addr);
        cpu.y = data;
        cpu.update_zero_and_negative_flags(cpu.y);
        cpu.cycle += 4;
        4
    }
}
