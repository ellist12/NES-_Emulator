use crate::{bus::Bus, cpu::Cpu};

mod cpu;
mod bus;
mod cartridge;

fn main() {
    let mut bus = Bus::new();
    bus.load_rom("Excitebike (Japan, USA).nes");
    let mut cpu = Cpu::new();
    cpu.reset(&mut bus);
    println!("{:?}", cpu);

    for i in 0..100 {
        cpu.step(&mut bus);
    }
}
