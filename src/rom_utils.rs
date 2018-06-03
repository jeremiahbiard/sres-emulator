extern crate zip;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::result::Result;

#[derive(Debug)]
enum RomType {
    LoRom,
    HiRom,
}

#[derive(Debug)]
struct RomHeader {
    rom_name: String,
    rom_type: RomType,
}

impl RomHeader {
    fn new(rom_name: String, rom_type: RomType) -> RomHeader {
        RomHeader {
            rom_name,
            rom_type,
        }
    }
}

pub fn load_rom(path: &Path) -> Result<Vec<u8>, Box<Error>> {

    // TODO: Add better error handling here.
    let mut file = fs::File::open(&path)?; 
    let extension = path.extension().unwrap();
    let mut program = Vec::new(); 
    
    if extension.to_str() == Some(&"zip") {
        let mut archive = zip::ZipArchive::new(&mut file)?;
        let mut rom = archive.by_index(0)?;
        rom.read_to_end(&mut program)?;
    } else {
        file.read_to_end(&mut program)?;
    };

    let h: RomHeader = extract_header_info(&program);
    println!("{:?}", h); 
    Ok(program)
}

// TODO: actually get the info from the rom
fn extract_header_info(_program: &Vec<u8>) -> RomHeader {
   RomHeader::new("SUPER MARIOWORLD".to_string(), RomType::LoRom)
}
