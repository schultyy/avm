use std::path::{PathBuf, Path};
use std::fs;

pub struct AutoSelect {
    package_path: PathBuf
}

impl AutoSelect {
    pub fn new(cwd: &Path) -> AutoSelect {
        let package_file = cwd.join("package.json");
        AutoSelect { package_path: package_file }
    }
    pub fn has_package_json(&self) -> bool {
        match fs::metadata(&self.package_path){
            Ok(metadata) => metadata.is_file(),
            Err(_)      => false
        }
    }
}
