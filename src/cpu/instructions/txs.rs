use crate:: cpu::cpu::Cpu;

pub struct TXS;

impl TXS {
    // TXS: Transfer X to Stack Pointer
    // Pindah data dari register X ke stack pointer
    // Ukuran opcode : 1 byte
    // Jumlah cycle : 2
    // Contoh kode assembly : TXS [9a]
    // Artinya : transfer data dari regiser x ke stack pointer
    pub fn transfer(cpu: &mut Cpu) -> u16 {
        println!("TXS");
        cpu.sp = cpu.x;
        cpu.cycle += 2;
        2
    }
}
