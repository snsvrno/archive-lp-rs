use std::path::{ Path, PathBuf};
use failure::Error;
use std::fs::create_dir_all;

pub enum ArchiveFormat {
    Zip
}

impl std::fmt::Display for ArchiveFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArchiveFormat::Zip => write!(f, "zip"),
        }
    }
}

pub fn from<P1:AsRef<Path>, P2:AsRef<Path>>(src : P1, des : P2, format : ArchiveFormat) -> Result<PathBuf,Error>
    where std::path::PathBuf: std::convert::From<P1>, P1 : Copy,
          std::path::PathBuf: std::convert::From<P2>, P2 : Copy, 
{
    let src_path : PathBuf = PathBuf::from(src);
    let mut des_path : PathBuf = PathBuf::from(des);
            
    // checks if we already inputed the file name.
    // if we don't then we use the folder name + standard extension
    if let None = des_path.file_name() {
        if let Some(some_name) = src_path.file_name() {
            des_path.push(format!("{}.{}",some_name.to_string_lossy(),format));    
        }
    } 

    // needs to create the folders, in case there are folders too
    if let Some(parent) = des_path.parent() {
        create_dir_all(parent)?;
    }

    match format {
        ArchiveFormat::Zip => crate::formats::zip::create(&src_path,&des_path)?,    
    }

    Ok(des_path)
}