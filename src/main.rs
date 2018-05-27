extern crate sres_emulator;

use std::env;
use sres_emulator::rom_utils::*;

fn main() {
    let rom_file = env::args().nth(1).expect("Specify a rom to open");
    let program = Vec::from(load_rom(rom_file));
    let header = &program[0x7fc0..0x7fd4];
    for byte in header {
        print!("{}", *byte as char);
    }
    println!();

}
