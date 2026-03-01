use crate:: mochanes::MochaNES;

mod cpu;
mod bus;
mod ppu;
mod cartridge;
mod mochanes;

fn main() {

    let mut mocha_nes = MochaNES::new();
    mocha_nes.init();
    mocha_nes.run();
}
