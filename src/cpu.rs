#[derive(Default, Debug)]
pub struct Cpu {
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
    pub fn new() -> Cpu {
        Cpu::default()
    }

    fn step(&mut self) {
        self.pc += 1;
        println!("pc: {}", self.pc);
    }
}
