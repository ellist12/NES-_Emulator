use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct STA;

impl STA {
    // STA (Store Accumulator) Zero Page
    // Store nilai dari register A, ke ram bagian ZEROPAGE yang addressnya di specify di byte berikutnya
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 3 cycle
    // Contoh kode assembly : STA $02 [85 02]
    // Artinya : simpan nilai dari register A, ke bagian ram ZEROPAGE dengan address $02 (simpan ke $0002)
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let addr = param as u16;
        println!("STA ${:x}", param);
        bus.write(addr, cpu.a);
        cpu.cycle += 3;
        3
    }
}
