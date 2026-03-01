use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct JMP;

impl JMP {
    // JMP (Jump)
    // Melakukan jump (mengganti nilai program counter) ke value yang ditetapkan di 2 byte berikutnya.
    // Opcode ini mirip seperti JSR, tapi tidak menyimpan lokasi pc awal di stack
    // Ukuran opcode : 3 byte
    // Jumlah cycle : 3
    // Contoh kode assembly : JMP $0020
    // Artinya : Jump ke address $0020
    pub fn jump(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let lo = bus.read(cpu.pc) as u16;
        let hi = bus.read(cpu.pc.wrapping_add(1)) as u16;
        let addr = (hi << 8) | lo;
        println!("JMP ${:x}", addr);
        cpu.pc = addr;
        cpu.cycle += 3;
        3
    }
}
