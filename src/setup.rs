use std::path::Path;
use std::fs;
use std::env;
use std::io::Error;

pub fn avm_directory() -> String {
    let home_directory = env::home_dir().unwrap();
    let avm = home_directory.join(".avm");
    avm.as_path().to_str().unwrap().to_string()
}

fn home_directory_existant() -> bool {
    match fs::metadata(avm_directory()) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false
    }
}

pub fn prepare() -> Result<String, Error> {
    if home_directory_existant() {
        return Ok(avm_directory());
    }
    match fs::create_dir(avm_directory().clone()) {
        Ok(_) => Ok(avm_directory()),
        Err(err) => Err(err)
    }
}

pub fn create_version_directory(version: &String) -> Result<String, Error> {
    let path = Path::new(&avm_directory()).join(version)
        .as_path().to_str().unwrap().to_string();
    match fs::create_dir(&path) {
        Ok(_) => Ok(path.clone()),
        Err(err) => Err(err)
    }
}

pub fn has_version(version_str: &String) -> bool {
    let path = Path::new(&avm_directory()).join(version_str)
        .as_path().to_str().unwrap().to_string();
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false
    }
}

