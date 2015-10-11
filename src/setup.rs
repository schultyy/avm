use std::path::{Path, PathBuf};
use std::io;
use std::fs;
use std::env;

pub fn prepare() -> String {
    let home_directory = env::home_dir().unwrap();
    let avm_directory = home_directory.join(".avm");
    fs::create_dir(avm_directory.clone());
    avm_directory.to_str()
        .unwrap()
        .into()
}
