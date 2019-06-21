use failure::Error;
use crate::{ utils, formats };
use std::path::{PathBuf, Path};

pub fn contains_file<P:AsRef<Path>>(src : P, file : &str) -> Result< bool,Error>
    where std::path::PathBuf: std::convert::From<P>, P : std::fmt::Display + Copy,  
{
    //! Checks if a certain file is in an archive.
        
    let src_path : PathBuf = PathBuf::from(src);
    
    match src_path.extension() {
        None => Err(format_err!("File '{}' has no extension, not a file?",src)),
        Some(ext) => { 
            match ext.to_str().unwrap() {
                "zip" => formats::zip::contains(&src_path,file),
                "tar" => formats::tar::contains(&src_path,file),
                "gz" => {
                    let buffer = formats::gz::decode(&src_path)?;
                    match utils::get_second_extension(src_path.file_name().unwrap().to_str().unwrap()){
                        Some("tar") => formats::tar::buffer_contains(&buffer,file),
                        Some(other_ext) => Err(format_err!("Unknown format {}",other_ext)),
                        None => Err(format_err!("No nested archive")),
                    }
                },
                "xz" => { 
                    let buffer = formats::xz::decode(&src_path)?;
                    match utils::get_second_extension(src_path.file_name().unwrap().to_str().unwrap()){
                        Some("tar") => formats::tar::buffer_contains(&buffer,file),
                        Some(other_ext) => Err(format_err!("Unknown format {}",other_ext)),
                        None => Err(format_err!("No nested archive")),
                    }
                },
                ext => Err(format_err!("Unknown extension type {}",ext)),
            }
        } 
    }
}
