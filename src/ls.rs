use setup;
use std::fs;

fn is_directory(path: &String) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_)       => false
    }
}

pub fn ls_versions() -> Vec<String> {
    let home = setup::avm_directory();
    let mut paths = Vec::new();
    for path in fs::read_dir(home).unwrap() {
        let path_str = path.unwrap().path().display().to_string();
        if is_directory(&path_str) {
            paths.push(path_str);
        }
    }
    paths
}
