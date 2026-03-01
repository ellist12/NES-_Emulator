use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct JSR;

impl JSR {
    // JSR (Jump to subroutine)
    // CPU Pergi ke alamat lain untuk menjalankan kode, tapi ia "mencatat" alamat asalnya agar nanti bisa pulang menggunakan
    // instruksi RTS (Return from Subsoutine)
    // Ukuran opcode : 3 byte
    // Jumlah cycle : 6 cycle
    // Contoh kode assembly : JSR $1000 [20 00 10]
    // Artinya : Pergi ke alamat $1000, tapi catat alamat asal di stack agar bisa balik
    pub fn jump(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let lo = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let hi = bus.read(cpu.pc) as u16;
        let addr = (hi << 8)  | lo;
        cpu.pc = cpu.pc.wrapping_add(1);

        println!("JSR ${:x}", addr);

        let pc_lo = ((cpu.pc - 1) & 0x00FF) as u8;
        let pc_hi = ((cpu.pc - 1) >> 8) as u8;
        bus.write(0x0100 + cpu.sp as u16, pc_hi);
        cpu.sp = cpu.sp.wrapping_sub(1);
        bus.write(0x0100 + cpu.sp as u16, pc_lo);
        cpu.sp = cpu.sp.wrapping_sub(1);

        cpu.pc = addr;
        cpu.cycle += 6;

        6
    }
}
