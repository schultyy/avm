use std::path::{Path, PathBuf};
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
