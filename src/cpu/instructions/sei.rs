use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct SEI;

impl SEI {
    // SEI (Set Interrupt Flag)
    // Nyalakan bit flag interrupt di status (0b00000100)
    // Jumlah cycle : 2
    pub fn set(cpu: &mut Cpu) -> u16 {
        println!("SEI");
        cpu.status = cpu.status | 0b00000100;
        cpu.cycle += 2;
        2
    }
}
