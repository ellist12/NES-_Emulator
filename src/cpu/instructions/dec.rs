use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct DEC;

impl DEC {
    // DEC (Decrement Memory) Zero Page
    // Ambil nilai di sebuah alamat memory ZEROPAGE yang di specify di byte berikutnya setelah opcode, menguranginya dengan 1,
    // menyimpan kembali hasilnya ke alamat tersebut
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 5 cycle
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        println!("DEC ${:x}", param);
        cpu.pc = cpu.pc.wrapping_add(1);
        let old_data = bus.read(param as u16);
        let new_data = old_data.wrapping_sub(1);
        bus.write(param as u16, new_data);
        cpu.update_zero_and_negative_flags(new_data);
        cpu.cycle += 5;
        5
    }
}
