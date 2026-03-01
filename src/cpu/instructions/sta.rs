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

    // STA Absolute: Tulis nilai dari register A, ke alamat memori yang ditentukan
    // Ukuran Opcode: 3 byte,
    // Jumlah cycle : 4
    // Contoh kode assembly : STA $2000 [8D 00 20]
    // Artinya: tulis yang ada di register A ke address 2000
    pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let lo = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let hi = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);

        let addr = (hi << 8) | lo;
        println!("STA ${:x}", addr);
        bus.write(addr, cpu.a);
        cpu.cycle += 4;
        4
    }
}
