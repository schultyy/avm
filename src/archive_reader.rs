use std::process::{Command, Output};
use std::io::prelude::*;
use std::io::Error;

pub fn decompress(archive_path: String, destination_folder: String) -> Result<Output, Error> {
    Command::new("tar")
        .arg("-zxvf")
        .arg(archive_path)
        .arg("-C")
        .arg(destination_folder)
        .output()
}

