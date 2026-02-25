use std::io;

use crate::{bus::Bus, cpu::Cpu};

mod cpu;
mod bus;
mod ppu;
mod cartridge;

fn main() {
    let mut bus = Bus::new();
    bus.load_rom("Donkey Kong (JU) [T-Span].nes");
    let mut cpu = Cpu::new();
    cpu.reset(&mut bus);

    let mut input = String::new();

    for i in 0..100 {
        io::stdin().read_line(&mut input).unwrap();
        cpu.step(&mut bus);
        println!("{:?}", cpu);
    }
}
