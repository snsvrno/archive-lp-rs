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
fn gz_tar_extract_a_file() {
    init_log();

    match archive::extract::to("tests/test_archives/a-file.tar.gz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/tar_gz_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn gz_tar_extract_a_file_1() {
    init_log();

    match archive::extract::to("tests/test_archives/1-folder-nest.tar.gz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/tar_gz_1_folder/tar_gz_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(&extracted_file_path);
    let _ = fs::remove_dir(&extracted_file_path.parent().unwrap());
}

#[test]
fn gz_tar_extract_a_file_root_2() {
    init_log();

    match archive::extract::root_to("tests/test_archives/2-folders-nest.tar.gz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }
    
    let extracted_file_path = PathBuf::from("tests/tar_gz_2_folder_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn gz_tar_extract_a_file_root_1() {
    init_log();

    match archive::extract::root_to("tests/test_archives/1-folder-nest.tar.gz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }

    let extracted_file_path = PathBuf::from("tests/tar_gz_1_folder_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn gz_tar_extract_a_file_root() {
    init_log();

    match archive::extract::root_to("tests/test_archives/a-file.tar.gz", "tests/") {
        Ok(path) => { info!("Extracted to '{:?}'",path); },
        Err(error) => { error!("Extraction error: {}",error.to_string()); assert!(false); },
    }

    let extracted_file_path = PathBuf::from("tests/tar_gz_a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}


#[test]
fn gz_tar_check_file() {
    init_log();

    match archive::contains_file("tests/test_archives/a-file.tar.gz", "tar_gz_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/1-folder-nest.tar.gz", "tar_gz_1_folder_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/2-folders-nest.tar.gz", "tar_gz_2_folder_1_folder_a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/2-folders-nest.tar.gz", "a_files") {
        Ok(result) => assert!(result == false),
        Err(_) => assert!(false),
    }
}