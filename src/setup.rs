use std::path::{Path, PathBuf};
use std::fs::Metadata;
use std::io;
use std::fs;
use std::env;

pub fn avm_directory() -> String {
    let home_directory = env::home_dir().unwrap();
    let avm = home_directory.join(".avm");
    avm.as_path().to_str().unwrap().to_string()
}

pub fn prepare() -> String {
    fs::create_dir(avm_directory().clone());
    avm_directory()
}

pub fn create_version_directory(version: &String) {
    let path = Path::new(&avm_directory()).join(version)
        .as_path().to_str().unwrap().to_string();
    fs::create_dir(path);
}

pub fn has_version(version_str: &String) -> bool {
    let path = Path::new(&avm_directory()).join(version_str)
        .as_path().to_str().unwrap().to_string();
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false
    }
}

