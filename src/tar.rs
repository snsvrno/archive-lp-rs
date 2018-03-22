use ansi_term::Colour::{Red,Yellow,Blue,Green};

use std::path::PathBuf;

use tarcrate;

pub fn unzip_buffer(buffer : &Vec<u8>, des : &PathBuf, root_file : Option<&str>) -> Option<PathBuf> {


  let mut root_length = 0;
  match tarcrate::Archive::new(&buffer[..]).entries() {
    Err(error) => { output_error!("Error getting entries from tarball: {}",Yellow.paint(error.to_string())); },
    Ok(entries) => { 

      // first digging through it and finding the root file.
      if let Some(root_file) = root_file {
        for file in entries{
          match file { 
            Err(error) => { output_error!("Error reading file: {}",Yellow.paint(error.to_string())); },
            Ok(mut file) => { 
              if let Ok(file_name) = file.header().path() {
                if let Some(filename) = file_name.to_str() {

                  let temp_path = PathBuf::from(&filename);
                  if root_file == temp_path.file_name().unwrap().to_str().unwrap() {
                    if root_length == 0 || root_length > filename.len() { 
                      root_length = filename.len() - root_file.len();
                      
                      output_debug!("Found {root} in archive, calculating root_length ({length}) which gives a new path of {path} for {root}",
                        root=Green.paint(root_file.to_string()),
                        length=Blue.paint(root_length.to_string()),
                        path=Blue.paint(filename[root_length..].to_string())
                      );

                    }
                  }

                }
              }
            }
          }
        }
      }
    }
  }

  // calling this twice because I can't figure out how not ot consume the entries. I need to iterate over them twice, the first time to figure out the root directory,
  // then the second to actually extract it.

  match tarcrate::Archive::new(&buffer[..]).entries() {
    Err(error) => { output_error!("Error getting entries from tarball: {}",Yellow.paint(error.to_string())); },
    Ok (entries) => { 
      // creates the files
      for file in entries {
        match file { 
          Err(error) => { output_error!("Error reading file: {}",Yellow.paint(error.to_string())); },
          Ok(mut file) => {
            let mut new_file_path : Option<PathBuf> = None;

            if let Ok(file_name) = file.header().path() {
              if let Some(filename) = file_name.to_str() {
                let mut new_file_path_working = des.clone();
                new_file_path_working.push(filename[root_length..].to_string());
                new_file_path = Some(new_file_path_working);
              }
            }

            if let Some(path) = new_file_path { 
              match file.unpack(&path) {
                Err(error) => { output_error!("Could not create file {}: {}",Red.paint(path.display().to_string()),Yellow.paint(error.to_string())); }
                Ok(_) => { }
              } 
            }
          }
        }
      }

    }
  } 

  None
}