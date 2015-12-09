use std::path::Path;
use std::fs;

pub fn has_package_json(cwd: &Path) -> bool {
    let package_file = cwd.join("package.json");
    match fs::metadata(package_file) {
        Ok(metadata) => metadata.is_file(),
        Err(_)      => false
    }
}
