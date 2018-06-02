extern crate sres_emulator;

use std::env;
use sres_emulator::rom_utils;

#[derive(Default, Debug)]
struct Cpu {
    pc: u16,
    reg_x: u16,
    reg_y: u16,
    reg_a: u16,
    // Direct page register. Specifies where the first bank of 64k
    // Direct Page (zero page) will be located. The Direct Page may be
    // moved to any location within Bank 0.
    reg_dp: u16,
    reg_sp: u16,
    reg_status: ProcessorStatusReg,
}

#[derive(Default, Debug)]
struct ProcessorStatusReg {
    carry: bool,
    zero: bool,
    irq_disable: bool,
    decimal_mode: bool,
    index_reg_select: bool,
    mem_accumulator_select: bool,
    overflow: bool,
    negative: bool,
}

impl ProcessorStatusReg {
    fn new() -> ProcessorStatusReg {
        ProcessorStatusReg::default()
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu::default()
    }

    fn step(&mut self) {
        self.pc += 1;
        println!("pc: {}", self.pc);
    }
}

fn main() {
    let rom_file = env::args().nth(1).expect("Specify a rom to open");
    let program = Vec::from(rom_utils::load_rom(rom_file));
    let header = &program[0x7fc0..0x7fd4];
    for byte in header {
        print!("{}", *byte as char);
    }
    println!();

    let mut cpu = Cpu::new();

    cpu.step();
    println!("{:#?}", cpu);

}
