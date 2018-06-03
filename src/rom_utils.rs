extern crate zip;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::result::Result;

pub fn load_rom<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Box<Error>> {
    let mut fh = fs::File::open(&path)?;
    let mut program = Vec::new(); 
    
    fh.read_to_end(&mut program)?;
    Ok(program)
}
