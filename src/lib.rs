mod formats;
mod utils;

pub mod extract;
mod read; pub use crate::read::contains_file;

// remove these after removing the deprecated functions
use failure::Error;
use std::path::{ PathBuf, Path };

#[deprecated(since="0.3.0", note="please use `extract::to` instead")]
pub fn extract_to<P:AsRef<Path>>(src : P, des : P) -> Result<PathBuf,Error>
    where std::path::PathBuf: std::convert::From<P>, P : std::fmt::Display + Copy, 
{
	crate::extract::to(src,des)
}

#[deprecated(since="0.3.0", note="please use `extract::root_to` instead")]
pub fn extract_root_to<P:AsRef<Path>>(src : P, des : P) -> Result<PathBuf,Error>
    where std::path::PathBuf: std::convert::From<P>, P : std::fmt::Display + Copy, 
{
	crate::extract::root_to(src,des)
}