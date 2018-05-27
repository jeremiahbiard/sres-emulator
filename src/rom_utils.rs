use std::fs;
use std::io::Read;
use std::path::Path;

pub fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut fh = fs::File::open(path).expect("Error opening rom.");
    let mut program = Vec::new();
    fh.read_to_end(&mut program).expect("Error reading rom");
    program
}
