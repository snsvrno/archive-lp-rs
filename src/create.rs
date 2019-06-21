use zip as zipcrate;
use failure::Error;

use std::fs::File;
use std::path::{ Path, PathBuf };

pub fn zip<P1 : AsRef<Path>,P2 : AsRef<Path>>(src : P1, dest : P2) -> Result<(), Error> {
	//! creates a zip file for the given path. 
	//! 
	//! it will archive the files / folders inside, but not including the 
	//! supplied `src` folder. so for `src = "a/b/c"` it will include all
	//! the files in "c" but not include "c" into the archive.

	let src_path = PathBuf::from(src.as_ref());
	let dest_path = PathBuf::from(dest.as_ref());

	let file = File::create(dest_path)?;
	let mut zip_file = zipcrate::ZipWriter::new(file);
	let zip_options = zipcrate::write::FileOptions::default();

	load_files_into_archive(&mut zip_file, &zip_options, &src_path, &src_path)?;

	zip_file.finish()?;

	Ok(())
}

// PRIVATE

fn load_files_into_archive(	
	zip_file : &mut zipcrate::ZipWriter<File>, 
	zip_options : &zipcrate::write::FileOptions, 
	src : &PathBuf, 
	root : &PathBuf ) -> Result<(),Error> {

	//! loads files into the provided archive.
	//! 
	//! it will recursively call itself once it finds a folder and iterate
	//! through all subfolders to add to the archive.
	
	use std::io::{ Write, Read };
	use std::fs::read_dir;

	for entry in read_dir(src)? {
		let entry = entry?;
		let path = entry.path();

		match path.is_dir() {
			true => load_files_into_archive(zip_file, zip_options, &path, root)?,
			false => {
				let short_file_path : &Path = path.strip_prefix(root)?;

				zip_file.start_file(short_file_path.display().to_string(), *zip_options)?;

				let mut file : File = File::open(path)?;
				let mut bytes : Vec<u8> = Vec::new();
				file.read_to_end(&mut bytes)?;

				zip_file.write(&bytes)?; 
			}
		}
	}

	Ok(())
}