#[macro_use]
extern crate output;
extern crate zip as zipcrate;
extern crate ansi_term; use ansi_term::Colour::{Red,Yellow,Blue};

mod zip;

use std::path::PathBuf;
use std::fs::{create_dir_all,remove_dir_all};

pub fn extract_to(src : &PathBuf, des : &PathBuf) -> Result<PathBuf,&'static str> {
  if !src.exists() { output_error!("File {} does not exist",Red.paint(src.display().to_string())); return Err("File does not exist"); }

  if des.exists() {
    match remove_dir_all(&des) {
      Err(error) => { output_error!("Cannot delete \'{}\': {}",Red.paint(des.display().to_string()),Yellow.paint(error.to_string())); }
      Ok(_) => { output_debug!("\'{}\' already exists, deleteing it a recreating it.",Blue.paint(des.display().to_string())); }
    }
  }

  if !des.exists() {
    match create_dir_all(&des) {
      Err(error) => { output_error!("Cannot create \'{}\': {}",Red.paint(des.display().to_string()),Yellow.paint(error.to_string())); }
      Ok(_) => { output_debug!("\'{}\' created.",Blue.paint(des.display().to_string())); }
    }
  }

  match src.extension() {
    None => { output_error!("File {} has no extension, not a file?",Red.paint(src.display().to_string())); return Err("File doesn't have extension"); }
    Some(ext) => { 
      match ext.to_str().unwrap() {
        "zip" => { return zip::unzip(&src,&des); },
        ext => { output_error!("Unknown extension type {}",Yellow.paint(ext)); }
      }
    }

  }




  Err("didn't extract")
}