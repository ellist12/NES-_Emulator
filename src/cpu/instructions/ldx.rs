use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct LDX;

impl LDX {
    // LDX Immideate: Ambil byte berikutnya, taruh di register X
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 2
    // Contoh kode assembly : LDX #$10 [A2 10]
    // Artinya : ambil angka di byte berikutnya (10), dan masukkan ke register X
    pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.x = param;
        println!("LDX #${:x}", param);
        cpu.update_zero_and_negative_flags(cpu.x);
        cpu.cycle += 2;
        2
    }
}
