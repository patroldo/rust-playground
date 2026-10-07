use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

pub fn create_file_if_not_exists(path: &PathBuf) -> std::io::Result<File> {
    OpenOptions::new()
        .create(true) // Creates the file if it doesn't exist
        .append(true)
        .open(path)
}
