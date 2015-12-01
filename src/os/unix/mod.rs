use std::os::unix::fs;
use std::path::Path;
use std::io::Error;

pub fn symlink<P: AsRef<Path>>(from: P, to: P) -> Result<(), Error> {
    fs::symlink(from, to)
}