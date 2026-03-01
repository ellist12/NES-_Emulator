use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct BPL;

impl BPL {
    // BPL (Branch if positive)
    // Melompat ke baris kode lain jika hasil operasi sebelumnya positif, jumlah lompatan tergantung dengan 1 byte berikutnya
    // Untuk BNE, angka di 1 byte berikutnya harus kita konversi dulu menjadi signed integer i8 sebelum kita operasikan
    // Ukuran Opcode : 2 byte
    // Jumlah cycle :
    //      1. 2 jika kondisi tidak terpenuhi
    //      2. 3 jika kondisi terpenuhi dan tidak melewati *page boundary*
    //      3. 4 jika kondisi terpenuhi dan melewati *page_boundary*
    // Contoh kode assembly : BPL $05
    // Artinya : Jika bit flag negatif di register status == 0, lompat 5 byte kedepan
    pub fn branch(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let mut cycle = 2;
        let bytes_to_jump = bus.read(cpu.pc);
        println!("BPL ${:x}", bytes_to_jump);
        cpu.pc = cpu.pc.wrapping_add(1);
        if cpu.status & 0b10000000 == 0 {
            cycle += 1;
            let offset = bytes_to_jump as i8;
            let old_pc = cpu.pc;
            let new_pc = cpu.pc.wrapping_add_signed(offset as i16);
            if (old_pc & 0xFF00) != (new_pc & 0xFF00) {
                cycle += 1;
            }
            cpu.pc = new_pc;
        }
        cpu.cycle += cycle;
        cycle
    }
}
