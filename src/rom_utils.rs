extern crate zip;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::result::Result;

pub fn load_rom(path: &Path) -> Result<Vec<u8>, Box<Error>> {

    // TODO: Add better error handling here.
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

    let mut file = fs::File::open(&path)?;
    let mut archive = zip::ZipArchive::new(&mut file)?;
    let mut rom = archive.by_index(0)?;
    let mut program = Vec::new();
    rom.read_to_end(&mut program)?;

    Ok(program)
}
