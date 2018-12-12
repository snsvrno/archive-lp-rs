extern crate archive_lp as archive;

use std::fs;
use std::path::PathBuf;

#[test]
fn gz_tar_extract_a_file() {
    assert!(archive::extract_to("tests/test_archives/a-file.tar.gz", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn gz_tar_extract_a_file_1() {
    assert!(archive::extract_to("tests/test_archives/1-folder-nest.tar.gz", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_folder/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(&extracted_file_path);
    let _ = fs::remove_dir(&extracted_file_path.parent().unwrap());
}

#[test]
fn gz_tar_extract_a_file_root_2() {
    assert!(archive::extract_root_to("tests/test_archives/2-folders-nest.tar.gz", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn gz_tar_extract_a_file_root_1() {
    assert!(archive::extract_root_to("tests/test_archives/1-folder-nest.tar.gz", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn gz_tar_extract_a_file_root() {
    assert!(archive::extract_root_to("tests/test_archives/a-file.tar.gz", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}


#[test]
fn gz_tar_check_file() {
    match archive::contains_file("tests/test_archives/a-file.tar.gz", "a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/1-folder-nest.tar.gz", "a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/2-folders-nest.tar.gz", "a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::contains_file("tests/test_archives/2-folders-nest.tar.gz", "a_files") {
        Ok(result) => assert!(result == false),
        Err(_) => assert!(false),
    }
}