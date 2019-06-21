use failure::Error;
use tarcrate;

use std::path::PathBuf;
use std::fs;
use std::io::Read;

#[cfg(feature = "indicate")]
extern crate indicatif;

pub fn extract(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,Error> {
    //! extracts the archive to the destination folder.
    
    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    extract_buffer(&buffer,des,false)
}

pub fn extract_root(file : &PathBuf, des : &PathBuf) -> Result<PathBuf,Error> {
    //! extracts the archive's root to the destination folder.
    //! 
    //! continues to go down into folders until it finds one with more than one item.
    //! it will then set the root of the folder as the point to start extracting. so
    //! and archive that looks like `a/b/c/d.txt` will extract to the destination 
    //! like `des/d.txt` and the folders a/b/c will be ignored.

    let mut buffer : Vec<u8> = Vec::new();
    let mut archive = fs::File::open(&file)?;
    archive.read_to_end(&mut buffer)?;

    extract_buffer(&buffer,des,true)
}

pub fn contains(archive : &PathBuf, file : &str) -> Result<bool,Error> {
    //! checks if a file exists in the archive
    
    let mut buffer : Vec<u8> = Vec::new();
    let mut archive_file = fs::File::open(&archive)?;
    archive_file.read_to_end(&mut buffer)?;

    buffer_contains(&buffer,file)
}

pub fn buffer_contains(buffer : &Vec<u8>, file_name : &str) -> Result<bool,Error> {
    //! checks if a file exists in a buffer

    let mut archive = tarcrate::Archive::new(&buffer[..]);
    for file in archive.entries()? {
        let mut file = file?;

        // checks if its a folder or a file
        // checks to see if the path ends in '/', then its a folder
        let filepath = file.header().path()?.to_str().unwrap().to_string();
        if filepath.chars().last().unwrap() == "/".chars().last().unwrap() {
            continue;
        }

        // gets the actual filename
        let filename = filepath.split("/").collect::<Vec<_>>();
        if filename[filename.len()-1] == file_name { 
            return Ok(true); 
        }
    }

    Ok(false)
}

pub fn extract_buffer(buffer : &Vec<u8>, des : &PathBuf, root : bool) -> Result<PathBuf,Error> {

    let mut archive = tarcrate::Archive::new(&buffer[..]);
    let mut root_length : Option<usize> = None;

    #[cfg(feature = "indicate")]
    let bar = { 
        let bar = indicatif::ProgressBar::new_spinner();
        bar.set_message(&format!("Extracting archive.."));
        bar
    };

    if root {
        // gets the root length, so it can remove all the base folders
        // that aren't important to the data / archive.

        #[cfg(feature = "indicate")]
        let mut count = {
            bar.set_message(&format!("Determining archive root, 0 files"));
            0
        };

        for file in archive.entries()? {

            let mut file = file?;

            // checks if its a folder or a file, we ignore folders
            // checks to see if the path ends in '/', then its a folder
            if file.header().path()?.to_str().unwrap().chars().last().unwrap() == "/".chars().last().unwrap() {
                continue;
            }

            let mut length = 0;
            if let Some(file_name) = file.header().path()?.to_str() {


                // for the indicator
                #[cfg(feature = "indicate")]
                {
                    count += 1;
                    bar.set_message(&format!("Determining archive root, {} files",count));
                }

                let splits = file_name.split("/").collect::<Vec<_>>();
                for i in 0 .. splits.len() - 1 {
                    length += splits[i].len() + 1;
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
        
        // had to do this because I couldn't figure out how to set the reader
        // count back to 0. calling .entries() moves the reader and doesn't
        // reset it back to 0.
        archive = tarcrate::Archive::new(&buffer[..]);
    }

    #[cfg(feature = "indicate")]
    let mut index = {
        let number_of_files = archive.entries()?.collect::<Vec<_>>().len(); 
        archive = tarcrate::Archive::new(&buffer[..]);
        (0 , number_of_files)
    };

    for file in archive.entries()? {
        let mut file = file?;
        
        let mut unpack_path : Option<PathBuf>;
        
        // checks if its a folder or a file
        // checks to see if the path ends in '/', then its a folder
        if file.header().path()?.to_str().unwrap().chars().last().unwrap() == "/".chars().last().unwrap() {
            continue;
        }

        match file.header().path()?.to_str() {
            None => { return Err(format_err!("Can't get file from archive")); }
            Some(file_name) => {
                 match root_length {
                    
                    None => {
                        // if there is no root_length then we don't need to remove 
                        // things from the path, but we still need to set the unpack_path
                        // otherwise we won't be unpacking anything
                        
                        let mut new_file_path = des.clone();
                        new_file_path.push(file_name);
                        unpack_path = Some(new_file_path);
                    },
                    
                    Some(root_length) => { 
                        // if we have a root_length then we need to remove the 
                        // leading folders from the file name, so we can extract 
                        // it correctly.

                        #[cfg(feature = "indicate")]
                        {
                            index.0 += 1;
                            bar.set_message(&format!("{} : {} / {}",
                                file_name[root_length..].to_string(),
                                index.0,
                                index.1
                            ));
                        }

                        let mut new_file_path = des.clone();
                        new_file_path.push(file_name[root_length..].to_string());
                        // needs to create the folders, in case there are folders too
                        if let Some(parent) = new_file_path.parent() {
                            fs::create_dir_all(parent)?;
                        }

                        debug!("New path is {:?}",new_file_path);
                        unpack_path = Some(new_file_path);
                    }
                }
            }
        }

        if let Some(path) = unpack_path { 
            debug!("Unpacking {:?}",path);

            // needs to check if the is a folder to put this file into.
            match path.parent() {
                None => { /* if no parent then not a problem */ },
                Some(parent) => { 
                    if parent.exists() == false {
                        fs::create_dir_all(parent)?;
                    }
                },
            }

            file.unpack(path)?; 
        }
    }

    #[cfg(feature = "indicate")]
    bar.finish_with_message(&format!("Extract complete",));
    Ok(des.clone())
}