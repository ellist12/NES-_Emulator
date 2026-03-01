use crate::cpu::cpu::Cpu;

pub struct DEY;

impl DEY {
    // DEY : Decrease Y Register, kurangi nilai di register y sebesar 1
    // Ukuran Opcode : 1 byte
    // Jumlah cycle : 2
    // Contoh kode assembly : DEY
    // Artinya : Kurangi nilai di register Y sebesar 1
    pub fn decrease(cpu: &mut Cpu) -> u16 {
        cpu.y = cpu.y.wrapping_sub(1);
        println!("DEY");
        cpu.update_zero_and_negative_flags(cpu.y);
        cpu.cycle += 2;
        2
    }
}
