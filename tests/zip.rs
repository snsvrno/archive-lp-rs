use archive_lp as archive;
use env_logger;
use log::{error, info, trace};

use std::fs;
use std::path::PathBuf;

fn init_log() {
    use log::LevelFilter;
    use env_logger::Builder;

    if let Err(error) = Builder::new().is_test(true).filter_level(LevelFilter::Trace).try_init() {
        trace!("Logger already initalized: {}",error.to_string());
    }
}

#[test]
fn zip_extract_a_file() {
    init_log();

    match archive::extract::to("tests/test_archives/a-file.zip", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/zip_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn zip_extract_a_file_1() {
    init_log();

    match archive::extract::to("tests/test_archives/1-folder-nest.zip", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/zip_1_folder/zip_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(&extracted_file_path);
    let _ = fs::remove_dir(&extracted_file_path.parent().unwrap());
}

#[test]
fn zip_extract_a_file_root_2() {
    init_log();

    match archive::extract::root_to("tests/test_archives/2-folders-nest.zip", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/zip_2_folder_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn zip_extract_a_file_root_1() {
    init_log();

    match archive::extract::root_to("tests/test_archives/1-folder-nest.zip", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }

    let extracted_file_path = PathBuf::from("tests/zip_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn zip_extract_a_file_root() {
    init_log();

    match archive::extract::root_to("tests/test_archives/a-file.zip", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }

    let extracted_file_path = PathBuf::from("tests/zip_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}


#[test]
fn zip_check_file() {
    init_log();

    match archive::read::contains_file("tests/test_archives/a-file.zip", "zip_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::read::contains_file("tests/test_archives/1-folder-nest.zip", "zip_1_folder_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::read::contains_file("tests/test_archives/2-folders-nest.zip", "zip_2_folder_1_folder_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::read::contains_file("tests/test_archives/2-folders-nest.zip", "a_files") {
        Ok(result) => assert!(result == false),
        Err(_) => assert!(false),
    }
}

#[test]
fn create_a_zip() {
    init_log();
    
    use std::fs::{create_dir_all, File};
    use std::io::Write;

    // makes some garbage to extract.
    let garbage_file = PathBuf::from("zip_create/file.txt");
    let file_content_set : String = String::from("this is some content for the file");
    let garbage_root = garbage_file.parent().unwrap();
    let _ = create_dir_all(&garbage_root);
    match File::create(&garbage_file) {
        Err(error) => { error!("Can't create file: {}",error.to_string()); assert!(false); }
        Ok(mut file) => match file.write_all(file_content_set.as_bytes()) {
            Err(error) => { error!("Can't write to file: {}",error.to_string()); assert!(false); }
            Ok(_) => { }
        }
    }

    // creates the archive
    if let Err(error) = archive::create::zip(garbage_root,"tests/garbage.zip") { 
        error!("Can't create the archive: {}",error.to_string());
    }

    // checks that the archive is correctly put together
    let content = archive::read::get_file_contents("tests/garbage.zip","file.txt").unwrap();
    assert_eq!(String::from_utf8_lossy(&content),file_content_set);

    // cleanup
    let _ = fs::remove_file("tests/garbage.zip");
    let _ = fs::remove_file(&garbage_file);
    let _ = fs::remove_dir(&garbage_root);
}