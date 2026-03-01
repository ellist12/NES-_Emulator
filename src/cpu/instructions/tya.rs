use crate::cpu::cpu::Cpu;

pub struct TYA;

impl TYA {
    // TYA (Transfer Register Y to Accumulator)
    // Pindahkan dan salin data dari register Y ke register A
    // Ukuran Opcode : 1 byte
    // Jumlah Cycle : 2 cycle
    // Contoh kode assembly : TYA [98]
    // Artinya : salin data dari register Y ke register A
    pub fn transfer(cpu: &mut Cpu) -> u16 {
        cpu.a = cpu.y;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 2;
        2
    }
}
