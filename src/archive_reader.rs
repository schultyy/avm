use std::process::{Command, Output};
use std::io::Error;
use std::path::Path;

pub fn decompress(archive_path: String, destination_folder: String, version_str: &String) -> Result<Output, Error> {
    Command::new("tar")
        .arg("-zxf")
        .arg(archive_path)
        .arg("-C")
        .arg(version_str)
        .arg("--strip")
        .arg("1")
        .current_dir(Path::new(&destination_folder))
        .output()
}

