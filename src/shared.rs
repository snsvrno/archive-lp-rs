use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use ansi_term::Colour::{Red,Yellow,Blue,Green};

pub fn get_file_contents(file : &PathBuf) -> Option<Vec<u8>> {
	let mut buf : Vec<u8> = Vec::new();
  let zip_file = File::open(&file);
  match zip_file {
    Err(error) => { 
      output_error!("Cannot openning file {}: {}",Red.paint(file.display().to_string()),Yellow.paint(error.to_string()));
      return None; 
    },
    Ok(mut zip_file) => { 
      match zip_file.read_to_end(&mut buf){
        Err(error) => { 
          output_error!("Cannot  reading {}'s buffer: {} ",Red.paint(file.display().to_string()),Yellow.paint(error.to_string()));
          return None; 
        },
        Ok (result) => { 
          output_debug!("File buffer read successfully from {}: {}",Blue.paint(file.display().to_string()),Green.paint(result.to_string()));
          return Some(buf);
        }
      }
    }
  }
}