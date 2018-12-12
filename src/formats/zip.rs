/// working with zip files, using 'zip-rs'

use failure::Error;

use std::fs;
use std::path::PathBuf;
use std::io::{Cursor,Read,Write};

use zipcrate;

pub fn unzip(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,Error> {
    //! unzips the archive to the destination folder.
    
    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    unzip_buffer(&buffer,des,false)
}

pub fn unzip_root(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,Error> {
    //! unzips the archive's root to the destination folder.
    
    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    unzip_buffer(&buffer,des,true)
}

pub fn contains(archive : &PathBuf, file_name : &str) -> Result<bool,Error> {
    //! checks if a file is in the archive.
    //! 
    //! 
    let mut buffer : Vec<u8> = Vec::new();
    let mut archive_file = fs::File::open(&archive)?;
    archive_file.read_to_end(&mut buffer)?;

    let mut zip = zipcrate::ZipArchive::new(Cursor::new(buffer))?;

    for i in 0 .. zip.len() {
        let mut file = zip.by_index(i)?;

        // checks if its a folder or a file
        // checks to see if the path ends in '/', then its a folder
        if file.name().chars().last().unwrap() == "/".to_string().chars().last().unwrap() {
            continue;
        }

        // gets the actual filename
        let filename = file.name().split("/").collect::<Vec<_>>();
        if filename[filename.len()-1] == file_name { 
            return Ok(true); 
        }
    }

    Ok(false)
}

fn unzip_buffer(buffer : &Vec<u8>, des : &PathBuf,root : bool) -> Result<PathBuf,Error> {
    let mut archive = zipcrate::ZipArchive::new(Cursor::new(buffer))?;
    let mut root_length = 0;

    // attempts to determine if the zip is actually inside redundant
    // folders, so we want to have the root of all the actual files
    // not just a folder with all the files inside of it.
    if root {
        for i in 0 .. archive.len() {

            let mut file = archive.by_index(i)?;

            // checks if its a folder or a file
            // checks to see if the path ends in '/', then its a folder
            if file.name().chars().last().unwrap() == "/".to_string().chars().last().unwrap() {
                continue;
            } 

            // a mess of code that tries to get the length of the folders 
            // in front of the file name, the intent is to find the file 
            // with the smallest amount of folders, and then that is the 
            // true root of the archive.
            {
                let mut splits = file.name().split("/");
                let count = splits.clone().count() - 1;
                let mut length = 0;
                for _ in 0 .. count {
                    if let Some(part) = splits.next() {
                        length += part.len() + 1;
                    }
                }

                if root_length < length {
                    root_length = length;
                }
            }
        }
    }

    for i in 0 .. archive.len() {
        if let Ok(mut file_in_zip) = archive.by_index(i) {

            // checks if its a folder or a file
            // checks to see if the path ends in '/', then its a folder
            if file_in_zip.name().chars().last().unwrap() == "/".to_string().chars().last().unwrap() {
                continue;
            } 

            let mut new_file_path = des.clone();
            new_file_path.push(file_in_zip.name()[root_length..].to_string());

            let mut file_buf : Vec<u8> = Vec::new();
            file_in_zip.read_to_end(&mut file_buf)?;

            // needs to create the folders, in case there are folders too
            if let Some(parent) = new_file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // creates the file now.
            let mut new_file = fs::File::create(&new_file_path)?;
            new_file.write_all(&file_buf)?;
        }
    }

    Ok(des.clone())
}