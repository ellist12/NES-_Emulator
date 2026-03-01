use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct CMP;

impl CMP {
    // CMP (Compare Accumulator) immediate
    // Mengcompare / membandingkan nilai yang ada di register A, nah disini comparenya bukan pakai ==, tapi
    // caranya adalah mengurangi nilai di register A, dan nilai parameter di byte berikutnya, lalu kita update
    // flag carry, zero, dan negative tergantung hasil pengurangannya
    // Ukuran Opcode : 2 byte
    // Jumlah cycle : 2 cycle
    // Contoh kode assembly : CMP #$05
    // Artinya, kita kurangi value di register A dengan $05, kalau hasilnya negatif, berarti kita set flag negatif ke 1 dan flag carry ke 0
    // kalau hasilnya 0, kita set flag zero ke 1, kalau positif, kita set flag negatif ke 0 dan flag carry ke 1
    pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let result = cpu.a as i16 - param as i16; // konversi ke i16 agar hasil bisa minus
        let result_u8 = result as u8; // buat versi u8 nya untuk cek flag zero dan negative, karena sejatinya
                                      // angka kita hanya 8 bit

        // Set flag zero
        if result_u8 == 0 {
            cpu.status = cpu.status | 0b00000010 // set cpu status zero flag to 1
        } else {
            cpu.status = cpu.status & 0b11111101 // set cpu status zero flag to 1
        }

        // set flag carry
        if result >= 0 {
            cpu.status = cpu.status | 0b00000001; // set cpu status carry flag to 1
        } else {
            cpu.status = cpu.status & 0b11111110; // set cpu status carry flag to 0
        }

        // set negative flag
        if (result_u8 & 0b10000000) != 0 {
            cpu.status = cpu.status | 0b10000000; // set cpu status negative flag to 1
        } else {
            cpu.status = cpu.status & 0b01111111; // set cpu status negative flag to 0
        }

        cpu.cycle += 2;
        2
    }
}
