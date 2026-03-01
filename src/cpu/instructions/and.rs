use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct AND;

impl AND {
    // AND Immidiate
    // Lakukan operasi logic AND antara nilai di register A dan
    // angka di byte berikutnya, hasilnya dimasukan ke register A
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 2
    // Contoh kode assembly : AND #$80
    // Artinya : lakukan operasi biner AND, antara value di register A dan value di byte berikutnya,
    //           lalu masukkan hasilnya ke register a
    pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        println!("AND #${:x}", param);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = cpu.a & param;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 2;
        2
    }
}
