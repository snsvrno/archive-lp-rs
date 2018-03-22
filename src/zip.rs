use zipcrate;
use ansi_term::Colour::{Red,Yellow,Blue,Green};

use std::path::PathBuf;
use std::fs::File;
use std::io::{Cursor,Read,Write};

use shared;

pub fn unzip(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,&'static str> {
  output_debug!("Processing as a zip");

  if let Some(buf) = shared::get_file_contents(&file) {
    match unzip_buffer(&buf,&des,Some("love.exe")) {
      Some(path) => { return Ok(path); },
      None => { },
    }
  }

  Err("Failed to extract")
}

pub fn unzip_buffer(buffer : &Vec<u8>, des : &PathBuf, root_file : Option<&str>) -> Option<PathBuf> {
  let archive = zipcrate::ZipArchive::new(Cursor::new(buffer));
  match archive {
    Err(error) => { 
      output_error!("Cannot read archive file stream buffer: {}",Yellow.paint(error.to_string()));
      return None;  
    }
    Ok(mut archive) => {
      
      let mut root_length = 0;
      if let Some(root_file) = root_file {
        for i in 0..archive.len() {
          if let Ok(mut file) = archive.by_index(i) {
            if file.name().contains(&root_file) {
              root_length = file.name().len()-8;
              let new_length : String = file.name()[root_length..].to_string();
              output_debug!("Found love.exe in archive, calculating root_length ({}) which gives a new path of {} for love.exe",Blue.paint(root_length.to_string()),Blue.paint(new_length));
            }
          }
        }
      }

      for i in 0..archive.len() {
        if let Ok(mut file_in_zip) = archive.by_index(i) {
          
          let mut new_file_path = des.clone();
          new_file_path.push(file_in_zip.name()[root_length..].to_string());
          
          let mut file_buf : Vec<u8> = Vec::new();
          match file_in_zip.read_to_end(&mut file_buf){
            Err(error) => { output_error!("Processing {}: {}",Red.paint(file_in_zip.name()),Yellow.paint(error.to_string())); }
            Ok(size) => { output_debug!("Processing {} ({}) bytes",Blue.paint(file_in_zip.name()),Green.paint(size.to_string())); }
          }

          let new_file = File::create(&new_file_path);
          match new_file {
            Err(error) => { output_error!("Cannot create new file \'{}\': {}",Red.paint(new_file_path.display().to_string()),Yellow.paint(error.to_string())); },
            Ok(mut new_file) => {
              match new_file.write_all(&file_buf) {
                Err(error) => { output_error!("Cannot write to file \'{}\': {}",Red.paint(new_file_path.display().to_string()),Yellow.paint(error.to_string())); },
                Ok(_) => { }
              }
            }
          }

        }
      }
      return Some(des.clone());
    }
  } 
}