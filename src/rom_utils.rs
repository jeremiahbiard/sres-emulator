extern crate zip;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::result::Result;

pub fn load_rom(path: &Path) -> Result<Vec<u8>, Box<Error>> {

    // TODO: Add better error handling here.
    let mut file = fs::File::open(&path)?; 
    let extension = path.extension().unwrap();
    let mut program = Vec::new(); 
    
    if extension.to_str() == Some(&"zip") {
        let mut archive = zip::ZipArchive::new(&mut file)?;
        let mut rom = archive.by_index(0)?;
        rom.read_to_end(&mut program)?;
        return Ok(program);
    }
    file.read_to_end(&mut program)?;
    Ok(program)
}
