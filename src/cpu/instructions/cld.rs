use crate::cpu::cpu::Cpu;

pub struct CLD;

impl CLD {
    // CLD (Clear Decimal Mode)
    // Nyalakan bit flag decimal mode di status (0b00001000)
    // Jumlah cycle : 2
    pub fn clear(cpu: &mut Cpu) -> u16 {
        println!("CLD");
        cpu.status = cpu.status | 0b00001000;
        cpu.cycle += 2;
        2
    }
}
