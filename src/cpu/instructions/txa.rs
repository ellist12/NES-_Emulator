use crate::cpu::cpu::Cpu;

pub struct TXA;

impl TXA {
    // TXA (Transfer X to Accumulator)
    // Pindahkan dan salin nilai di register x ke register a
    // Ukuran Opcode : 1 byte
    // Jumlah cycle : 2 cycle
    pub fn transfer(cpu: &mut Cpu) -> u16 {
        cpu.a = cpu.x;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 2;
        2
    }
}
