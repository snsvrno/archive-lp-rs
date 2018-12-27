# Archive-lp-rs

Rust library for working with archives, designed for Lovepack. Takes much of the middle work for accessing files inside a library.

## Functionality

Knows what to do for the following files

- *.zip
- *.tar.gz
- *.tar.xz

Can do the following

- extract archive to a directory: **archive_lp::extract_to()**
- extract root of an archive to a directory: **archive_lp::extract_root_to()**
- check if a file is in an archive: **archive_lp::contains_file()**

## Optional Features

- **indicate** : uses `indicatif` to show a progress spinner with some details during extraction. 