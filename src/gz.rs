use ansi_term::Colour::Yellow;
use std::path::PathBuf;
use std::io::prelude::*;

use shared;
use flate2;

use tar;

pub fn unzip(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,&'static str> {
	
  if let Some(compressed_buffer) = shared::get_file_contents(&file) {
    
  	let mut decoder = flate2::read::GzDecoder::new(&compressed_buffer[..]);
    let mut buffer : Vec<u8> = Vec::new();
    match decoder.read_to_end(&mut buffer) {
      Err(error) => { output_error!("Error reading decode gzip: {}",Yellow.paint(error.to_string())); },
      Ok(_) => { 

        match tar::unzip_buffer(&buffer,&des,Some("love")) {
          Some(path) => { return Ok(path); },
          None => { },
        }

      }
    }
  
  }

	Err("what")
}