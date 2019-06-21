/// working with zip files, using 'zip-rs'

use failure::Error;

use std::fs;
use std::path::PathBuf;
use std::io::{Cursor,Read,Write};

use log::trace;

#[cfg(feature = "indicate")]
use indicatif;

use zip as zipcrate;

pub fn extract(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,Error> {
    //! unzips the archive to the destination folder.
    
    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    extract_buffer(&buffer,des,false)
}

pub fn extract_root(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,Error> {
    //! unzips the archive's root to the destination folder.
    
    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    extract_buffer(&buffer,des,true)
}

pub fn contains(archive : &PathBuf, file_name : &str) -> Result<bool,Error> {
    //! checks if a file is in the archive.

    let mut buffer : Vec<u8> = Vec::new();
    let mut archive_file = fs::File::open(&archive)?;
    archive_file.read_to_end(&mut buffer)?;

    let mut zip = zipcrate::ZipArchive::new(Cursor::new(buffer))?;

    for i in 0 .. zip.len() {
        let file = zip.by_index(i)?;

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

pub fn extract_buffer(buffer : &Vec<u8>, des : &PathBuf, root : bool) -> Result<PathBuf,Error> {
    let mut archive = zipcrate::ZipArchive::new(Cursor::new(buffer))?;
    let mut root_length : Option<usize> = None;

    #[cfg(feature = "indicate")]
    let bar = { 
        let bar = indicatif::ProgressBar::new_spinner();
        bar.set_message(&format!("Extracting archive.."));
        bar
    };

    // attempts to determine if the zip is actually inside redundant
    // folders, so we want to have the root of all the actual files
    // not just a folder with all the files inside of it.
    if root {
        for i in 0 .. archive.len() {

            #[cfg(feature = "indicate")]
            bar.set_message(&format!("Determining archive root, {} files",i));

            let file = archive.by_index(i)?;

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

                if let Some(rlength) = root_length {
                    if rlength > length {
                        root_length = Some(length);
                    }
                } else {
                    root_length = Some(length);
                }
            }
        }
    }

    let archive_length = archive.len();
    for i in 0 .. archive_length {
        if let Ok(mut file_in_zip) = archive.by_index(i) {

            // checks if its a folder or a file
            // checks to see if the path ends in '/', then its a folder
            if file_in_zip.name().chars().last().unwrap() == "/".to_string().chars().last().unwrap() {
                continue;
            } 

            let mut new_file_path = des.clone();
            if root_length.is_none() { root_length = Some(0); }
            if let Some(root_length) = root_length {
                new_file_path.push(file_in_zip.name()[root_length..].to_string());

                // progress if enabled
                #[cfg(feature = "indicate")]
                bar.set_message(&format!("{} : {} / {}",
                    file_in_zip.name()[root_length..].to_string(),
                    i,
                    archive_length
                ));
            }

            let mut file_buf : Vec<u8> = Vec::new();
            file_in_zip.read_to_end(&mut file_buf)?;

            // needs to create the folders, in case there are folders too
            if let Some(parent) = new_file_path.parent() {
                fs::create_dir_all(parent)?;
            }


            // creates the file now.
            trace!("Going to try and make a file here: {:?}",new_file_path);
            let mut new_file = fs::File::create(&new_file_path)?;
            new_file.write_all(&file_buf)?;
        }
    }

    Ok(des.clone())
}