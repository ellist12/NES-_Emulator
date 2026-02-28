use crate::{bus::Bus, cpu::cpu::Cpu};


pub struct LDA;

impl LDA {
    pub fn immideate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        // LDA Immediate: Ambil byte berikutnya, taruh di register A
        // Ukuran Opcode : 2 byte
        // Jumlah cycle : 2
        // Contoh kode assembly : LDA #$30 [A9 30]
        // Artinya : Ambil angka di byte berikutnya (30) lalu masukkan ke register A
        let param = bus.read(cpu.pc);
        // println!("param a9: {}", param);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = param;
        println!("LDA #${:x}", param);
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 2;
        2
    }

    // LDA Zeropage: Ambil data di alamat ram bagian ZEROPAGE yang di specify di 1 byte berikutnya
    //               bagian ZEROPAGE di ram punya rentang dari $0000 - $00FF
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 3
    // Contoh kode assembly : LDA $10 [A5 10]
    // Artinya : ambil angka di ram dengan address $10 ($0010), dan masukkan ke register A
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let addr = bus.read(cpu.pc);
        let param = bus.read(addr as u16);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = param;
        println!("LDA ${:?}", param);
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 3;
        3
    }

    // LDA Absolute: Ambil data di alamat spesifik yang ditunjuk oleh 2 byte berikutnya
    // Ukuran Opcode : 3 byte
    // Jumlah cycle : 4
    // Contoh kode assembly : LDA $1000 [AD 00 10]
    // Artinya : Ambil angka di dua byte berikutnya ($1000), lalu masukkan ke register A
    pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let lo = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let hi = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let addr = (hi << 8) | lo;
        let data = bus.read(addr);
        println!("LDA ${:x}", addr);
        cpu.a = data;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 4;
        4
    }
}
