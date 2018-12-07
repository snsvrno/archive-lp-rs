extern crate archive_lp as archive;

use std::fs;
use std::path::PathBuf;

#[test]
fn extract_a_file() {
    assert!(archive::extract_to_root("tests/a-file.zip", "tests/").is_ok());
    
    let extracted_file_path = PathBuf::from("tests/a_file");
    assert!(extracted_file_path.exists());

    // cleanup
    fs::remove_file(extracted_file_path);
}