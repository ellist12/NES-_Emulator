use std::io;

use crate::{bus::Bus, cpu::Cpu};

mod cpu;
mod bus;
mod cartridge;

fn main() {
    let mut bus = Bus::new();
    bus.load_rom("Excitebike (Japan, USA).nes");
    let mut cpu = Cpu::new();
    cpu.reset(&mut bus);

    let mut input = String::new();

    for i in 0..100 {
        io::stdin().read_line(&mut input).unwrap();
        cpu.step(&mut bus);
        println!("{:?}", cpu);
    }
}
