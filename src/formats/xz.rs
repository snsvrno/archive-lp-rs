
use failure::Error;

use std::fs;
use std::path::PathBuf;
use std::io::Read;

use xz2;

pub fn decode(file : &PathBuf) -> Result<Vec<u8>,Error> {

    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    let mut decoder = xz2::read::XzDecoder::new(&buffer[..]);
    let mut decoded_buffer : Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decoded_buffer)?;

    Ok(decoded_buffer)

}