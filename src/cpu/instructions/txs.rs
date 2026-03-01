use crate:: cpu::cpu::Cpu;

pub struct TXS;

impl TXS {
    // TXS: Transfer X to Stack Pointer
    // Pindah data dari register X ke stack pointer
    // Jumlah cycle : 2
    pub fn transfer(cpu: &mut Cpu) -> u16 {
        println!("TXS");
        cpu.sp = cpu.x;
        cpu.cycle += 2;
        2
    }
}
