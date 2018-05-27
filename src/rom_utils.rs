use std::fs;
use std::io::Read;

pub fn load_rom(rom_name: String) -> Vec<u8> {
    let mut fh = fs::File::open(rom_name).expect("Error opening rom.");
    let mut program = Vec::new();
    fh.read_to_end(&mut program).expect("Error reading rom");
    return program;
}
