use std::path::{PathBuf, Path};
use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::fs;
use rustc_serialize::json::Json;

pub struct Selector {
    package_path: PathBuf
}

impl Selector {
    pub fn new(cwd: &Path) -> Selector {
        let package_file = cwd.join("package.json");
        Selector { package_path: package_file }
    }
    pub fn has_package_json(&self) -> bool {
        match fs::metadata(&self.package_path){
            Ok(metadata) => metadata.is_file(),
            Err(_)      => false
        }
    }

    pub fn specified_version(&self) -> Result<String, String> {
        let package_file = match self.read_package_json() {
            Ok(file) => file,
            Err(_) => return Err("No package.json found".to_string())
        };

        match self.find_engine_key(&package_file) {
            Some(version) => Ok(version),
            None => Err("No node engine specified".to_string())
        }
    }

    fn find_engine_key(&self, json_file: &String) -> Option<String> {
        let json = Json::from_str(&json_file).unwrap();
        match json.find_path(&["engines", "node"]) {
            Some(key) => Some(key.to_string()),
            None => None
        }
    }

    fn read_package_json(&self) -> Result<String, Error> {
        let mut f = try!(File::open(&self.package_path));
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_)    => Ok(s),
            Err(err) => Err(err)
        }
    }
}
