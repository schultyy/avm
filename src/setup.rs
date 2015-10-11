use std::path::{Path, PathBuf};
use std::io;
use std::fs;
use std::env;

pub fn prepare() -> Result<String, String> {
    let home_directory = match env::home_dir() {
        Some(home)  => home,
        None        => return Err("Could not get home directory".to_string())
    };

    let avm_directory = home_directory.join(".avm");

    match fs::create_dir(avm_directory.clone()) {
        Ok(result) => {
            let foo = avm_directory.to_str().unwrap();
            Ok(foo.into())
        },
        Err(err) => Err("Directory already exists".to_string())
    }
}
