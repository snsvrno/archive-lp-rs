extern crate archive_lp as archive;

use std::fs;
use std::path::PathBuf;

#[test]
fn extract_a_file() {
    assert!(archive::extract_to("tests/a-file.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    fs::remove_file(extracted_file_path);
}

#[test]
fn extract_a_file_1() {
    assert!(archive::extract_to("tests/1-folder-nest.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_folder/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    fs::remove_file(&extracted_file_path);
    fs::remove_dir(&extracted_file_path.parent().unwrap());
}

#[test]
fn extract_a_file_root_2() {
    assert!(archive::extract_to_root("tests/2-folders-nest.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    fs::remove_file(extracted_file_path);
}

#[test]
fn extract_a_file_root_1() {
    assert!(archive::extract_to_root("tests/1-folder-nest.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    fs::remove_file(extracted_file_path);
}

#[test]
fn extract_a_file_root() {
    assert!(archive::extract_to_root("tests/a-file.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    fs::remove_file(extracted_file_path);
}