extern crate archive_lp as archive;
#[macro_use] extern crate log; use log::LevelFilter;
extern crate env_logger;

use std::fs;
use std::path::PathBuf;

fn init_log() {
    if let Err(error) = env_logger::Builder::new().is_test(true).filter_level(LevelFilter::Trace).try_init() {
        trace!("Logger already initalized: {}",error.to_string());
    }
}

#[test]
fn xz_tar_extract_a_file() {
    init_log();

    match archive::extract_to("tests/test_archives/a-file.tar.xz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/tar_xz_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn xz_tar_extract_a_file_1() {
    init_log();

    match archive::extract_to("tests/test_archives/1-folder-nest.tar.xz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/tar_xz_1_folder/tar_xz_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(&extracted_file_path);
    let _ = fs::remove_dir(&extracted_file_path.parent().unwrap());
}

#[test]
fn xz_tar_extract_a_file_root_2() {
    init_log();

    match archive::extract_root_to("tests/test_archives/2-folders-nest.tar.xz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/tar_xz_2_folder_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn xz_tar_extract_a_file_root_1() {
    init_log();

    match archive::extract_root_to("tests/test_archives/1-folder-nest.tar.xz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }

    let extracted_file_path = PathBuf::from("tests/tar_xz_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn xz_tar_extract_a_file_root() {
    init_log();

    match archive::extract_root_to("tests/test_archives/a-file.tar.xz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }

    let extracted_file_path = PathBuf::from("tests/tar_xz_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}


#[test]
fn xz_tar_check_file() {
    init_log();

    match archive::contains_file("tests/test_archives/a-file.tar.xz", "tar_xz_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/1-folder-nest.tar.xz", "tar_xz_1_folder_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/2-folders-nest.tar.xz", "tar_xz_2_folder_1_folder_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/2-folders-nest.tar.xz", "a_files") {
        Ok(result) => assert!(result == false),
        Err(_) => assert!(false),
    }
}