extern crate sres_emulator;

use std::env;
use std::process;
use std::path::Path;
use sres_emulator::rom_utils;
use sres_emulator::cpu;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please specify a rom file.");
        process::exit(1);
    }
    let filename = Path::new(&args[1]);

    let program: Vec<u8> = rom_utils::load_rom(filename)
                            .unwrap_or_else(|err| {
                                eprintln!("couldn't load rom file: {}", err);
                                process::exit(1);
                            });
    let header = &program[0x7fc0..0x7fd4];
    for byte in header {
        print!("{}", *byte as char);
    }
    println!();

    /*
    let cpu = cpu::Cpu::new();

    println!("{:#?}", cpu);
    */

}
