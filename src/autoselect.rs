use std::path::{PathBuf, Path};
use std::fs;

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
}
