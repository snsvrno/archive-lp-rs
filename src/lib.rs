#[macro_use] extern crate failure; use failure::Error;
#[macro_use] extern crate log;

extern crate zip as zipcrate;

use std::path::{PathBuf, Path};
use std::fs;

mod formats;

pub fn extract_to_root<P:AsRef<Path>>(src : P, des : P) -> Result<PathBuf,Error>
    where std::path::PathBuf: std::convert::From<P>, P : std::fmt::Display + Copy, 
{
    let src_path : PathBuf = PathBuf::from(src);
    let des_path : PathBuf = PathBuf::from(des);

    if false == src_path.exists() { 
        return Err(format_err!("Source file, '{}' does not exist.",src)); 
    }

    if false == des_path.exists() { 
        warn!("Destination '{}' does not exist.",src);
        fs::create_dir_all(&des_path)?;
    }

    match src_path.extension() {
        None => Err(format_err!("File '{}' has no extension, not a file?",src)),
        Some(ext) => { 
            match ext.to_str().unwrap() {
                "zip" => formats::zip::unzip(&src_path,&des_path),
                // "gz" => { return gz::unzip(&src,&des); }
                ext => Err(format_err!("Unknown extension type {}",ext)),
            }
        } 
    }
}