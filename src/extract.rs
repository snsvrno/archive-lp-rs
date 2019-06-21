use failure::Error;
use crate::{ utils, formats };
use std::path::{PathBuf, Path};
use std::fs;

pub fn extract_to<P:AsRef<Path>>(src : P, des : P) -> Result<PathBuf,Error>
    where std::path::PathBuf: std::convert::From<P>, P : std::fmt::Display + Copy, 
{
    //! Extracts the archive to the destination folder.
    //! 
    //! Checks if the archive and folder exists and then
    //! extracts the archive into the folder. If the destination
    //! folder doesn't exist then it attempts to create it.
    
    let src_path : PathBuf = PathBuf::from(src);
    let des_path : PathBuf = PathBuf::from(des);

    if false == src_path.exists() { 
        return Err(format_err!("Source file, {} does not exist.",src)); 
    }

    if false == des_path.exists() { 
        warn!("Destination {} does not exist.",src);
        fs::create_dir_all(&des_path)?;
    }

    match src_path.extension() {
        None => Err(format_err!("File {} has no extension, not a file?",src)),
        Some(ext) => {
            match ext.to_str().unwrap() {
                "zip" => formats::zip::extract(&src_path,&des_path),
                "tar" => formats::tar::extract(&src_path,&des_path),
                "gz" => { 
                    let buffer = formats::gz::decode(&src_path)?;
                    match utils::get_second_extension(src_path.file_name().unwrap().to_str().unwrap()){
                        Some("tar") => formats::tar::extract_buffer(&buffer,&des_path,false),
                        Some(other_ext) => Err(format_err!("Unknown format {}",other_ext)),
                        None => Err(format_err!("No nested archive")),
                    }
                },
                "xz" => { 
                    let buffer = formats::xz::decode(&src_path)?;
                    match utils::get_second_extension(src_path.file_name().unwrap().to_str().unwrap()){
                        Some("tar") => formats::tar::extract_buffer(&buffer,&des_path,false),
                        Some(other_ext) => Err(format_err!("Unknown format {}",other_ext)),
                        None => Err(format_err!("No nested archive")),
                    }
                },
                ext => Err(format_err!("Unknown extension type {}",ext)),
            }
        } 
    }
}


pub fn extract_root_to<P:AsRef<Path>>(src : P, des : P) -> Result<PathBuf,Error>
    where std::path::PathBuf: std::convert::From<P>, P : std::fmt::Display + Copy, 
{
    //! Extracts the root of the archive to the destination folder.
    //! 
    //! Checks if the archive and folder exists and then
    //! extracts the archive into the folder. If the destination
    //! folder doesn't exist then it attempts to create it.
    //! 
    //! The 'root' of the folder is where all the files are located.
    //! Sometimes the contents of archives are nested in multiple 
    //! folders before the actual data. 
    //! 
    //! This function looks at each actual file in the archive and finds
    //! the file with the shortest absolute path in the archive, and assumes
    //! that is the root of the archive, and then extracts that file to the root
    //! of the destination folder.

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
                "zip" => formats::zip::extract_root(&src_path,&des_path),
                "tar" => formats::tar::extract_root(&src_path,&des_path),
                "gz" => { 
                    let buffer = formats::gz::decode(&src_path)?;
                    match utils::get_second_extension(src_path.file_name().unwrap().to_str().unwrap()){
                        Some("tar") => formats::tar::extract_buffer(&buffer,&des_path,true),
                        Some(other_ext) => Err(format_err!("Unknown format {}",other_ext)),
                        None => Err(format_err!("No nested archive")),
                    }
                },
                "xz" => { 
                    let buffer = formats::xz::decode(&src_path)?;
                    match utils::get_second_extension(src_path.file_name().unwrap().to_str().unwrap()){
                        Some("tar") => formats::tar::extract_buffer(&buffer,&des_path,true),
                        Some(other_ext) => Err(format_err!("Unknown format {}",other_ext)),
                        None => Err(format_err!("No nested archive")),
                    }
                },
                ext => Err(format_err!("Unknown extension type {}",ext)),
            }
        } 
    }
}