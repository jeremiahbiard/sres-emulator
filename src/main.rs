extern crate sres_emulator;

use std::env;
use sres_emulator::rom_utils;
use std::process;
use sres_emulator::cpu;

fn main() {
    let rom_file = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("please specify a rom file");
        process::exit(1);
    });

    let program = Vec::from(rom_utils::load_rom(rom_file)
                            .unwrap_or_else(|err| {
                                eprintln!("couldn't load rom file: {}", err);
                                process::exit(1);
                            }));
    let header = &program[0x7fc0..0x7fd4];
    for byte in header {
        print!("{}", *byte as char);
    }
    println!();

    let cpu = cpu::Cpu::new();

    println!("{:#?}", cpu);

}
