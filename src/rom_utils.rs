extern crate zip;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::result::Result;
use std::ffi::OsStr;

pub fn load_rom(path: &Path) -> Result<Vec<u8>, Box<Error>> {

    let extension = path.extension().unwrap();
    if extension.to_str() == Some(&"zip") {
        return load_zip(path)
    }
    let mut fh = fs::File::open(&path)?;
    let mut program = Vec::new(); 
    
    fh.read_to_end(&mut program)?;
    Ok(program)
}

fn load_zip(path: &Path) -> Result<Vec<u8>, Box<Error>> {
    println!("ZIP");
    let mut fh = fs::File::open(&path)?;
    let mut program = Vec::new();

    fh.read_to_end(&mut program)?;
    Ok(program)
}
