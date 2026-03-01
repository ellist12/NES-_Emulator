use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct STY;

impl STY {
    // STY Zeropage: setor data ke bagian ram ZEROPAGE di alamat yang di specify di 1 byte berikutnya
    //               bagian ZEROPAGE di ram punya rentang dari $0000 - $00FF
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 3
    // Contoh kode assembly : STY $10
    // Artinya: Tulis value yang ada di register Y, ke address $10 di ram bagian ZEROPAGE ($0010)
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let addr = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        bus.write(addr as u16, cpu.y);
        println!("STY ${:x}", addr);
        cpu.cycle += 3;
        3
    }
}
