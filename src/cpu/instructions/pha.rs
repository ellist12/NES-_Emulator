use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct PHA;

impl PHA {
    pub fn push(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        // PHA (Push accumulator on stack)
        // Menyimpan sementara nilai dari register A ke stack, sehingga accumulator bisa digunakan untuk
        // keperluan lain tanpa kehilangan nilai awalnya
        // Ukuran opcode : 1 byte
        // Jumlah cycle : 3 cycle
        // Contoh kode assembly : PHA
        // Artinya : Push nilai register A ke stack
        println!("PHA");
        bus.write(0x0100 + cpu.sp as u16, cpu.a);
        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.cycle += 3;
        3
    }
}
