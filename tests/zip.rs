extern crate archive_lp as archive;

use std::fs;
use std::path::PathBuf;

#[test]
fn extract_a_file() {
    assert!(archive::extract_to("tests/a-file.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn extract_a_file_1() {
    assert!(archive::extract_to("tests/1-folder-nest.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_folder/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(&extracted_file_path);
    let _ = fs::remove_dir(&extracted_file_path.parent().unwrap());
}

#[test]
fn extract_a_file_root_2() {
    assert!(archive::extract_root_to("tests/2-folders-nest.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn extract_a_file_root_1() {
    assert!(archive::extract_root_to("tests/1-folder-nest.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}

#[test]
fn extract_a_file_root() {
    assert!(archive::extract_root_to("tests/a-file.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    let _ = fs::remove_file(extracted_file_path);
}


#[test]
fn check_file() {
    match archive::archive_contains_file("tests/a-file.zip", "a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::archive_contains_file("tests/1-folder-nest.zip", "a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::archive_contains_file("tests/2-folders-nest.zip", "a_file") {
        Ok(result) => assert!(result),
        Err(_) => assert!(false),
    }
    match archive::archive_contains_file("tests/2-folders-nest.zip", "a_files") {
        Ok(result) => assert!(result == false),
        Err(_) => assert!(false),
    }
}