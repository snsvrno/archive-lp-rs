#[macro_use] extern crate failure; use failure::Error;
#[macro_use] extern crate log;

extern crate zip as zipcrate;
extern crate xz2;
extern crate flate2;
extern crate tar as tarcrate;

#[cfg(feature = "indicate")]
extern crate indicatif;

mod formats;
mod utils;

mod extract; pub use crate::extract::{ extract_to, extract_root_to };
mod read; pub use crate::read::contains_file;