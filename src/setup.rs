use std::path::Path;
use std::fs;
use std::env;
use std::io::Error;
use language::Language;

fn version_path(version: &String) -> String {
    Path::new(&avm_directory()).join(version)
        .as_path()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn avm_directory() -> String {
    let home_directory = env::home_dir().unwrap();
    let avm = home_directory.join(".avm");
    avm.as_path().to_str().unwrap().to_string()
}

pub fn home_directory_present() -> bool {
    match fs::metadata(avm_directory()) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false
    }
}

pub fn prepare(lang: Language) -> Result<String, Error> {
    if home_directory_present() {
        return Ok(avm_directory());
    }
    let language_dir = Path::new(&avm_directory()).join(lang.name);
    match fs::create_dir_all(&language_dir) {
        Ok(_) => {
            Ok(language_dir.as_path()
               .to_str()
               .unwrap()
               .to_string())
        },
        Err(err) => Err(err)
    }
}

pub fn create_version_directory(version: &String) -> Result<String, Error> {
    let path = version_path(&version);
    match fs::create_dir(&path) {
        Ok(_) => Ok(path.clone()),
        Err(err) => Err(err)
    }
}

pub fn has_version(version_str: &String) -> bool {
    let path = version_path(&version_str);
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false
    }
}

pub fn remove_version(version_str: &String) -> Result<(), Error> {
    let path = version_path(&version_str);
    fs::remove_dir_all(path)
}
