use failure::Error;

use std::fs;
use std::path::PathBuf;
use std::io::{Cursor,Read,Write};

use zipcrate;

pub fn unzip(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,Error> {

    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    unzip_buffer(&buffer,des)
}

pub fn unzip_buffer(buffer : &Vec<u8>, des : &PathBuf) -> Result<PathBuf,Error> {
    let mut archive = zipcrate::ZipArchive::new(Cursor::new(buffer))?;

    for i in 0 .. archive.len() {
        if let Ok(mut file_in_zip) = archive.by_index(i) {
            let mut new_file_path = des.clone();
            new_file_path.push(file_in_zip.name().to_string());

            let mut file_buf : Vec<u8> = Vec::new();
            let size = file_in_zip.read_to_end(&mut file_buf)?;

            let mut new_file = fs::File::create(&new_file_path)?;
            new_file.write_all(&file_buf)?;
        }
    }

    Ok(des.clone())
}