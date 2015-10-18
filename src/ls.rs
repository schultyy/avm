use std::path::Path;
use setup;
use std::fs;
use regex::Regex;

fn is_directory(path: &String) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_)       => false
    }
}

fn directory_name(full_path: &String) -> String {
    let components = Path::new(full_path).components();
    components.last().unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .into()
}

pub fn current_version() -> Option<String> {
    let home_directory = setup::avm_directory();
    let path = fs::read_link(Path::new(&home_directory).join("node")).unwrap();
    let re = Regex::new(r"\d\.\d\.\d").unwrap();
    let path_str = path.as_os_str().to_str()
        .unwrap()
        .into();
    println!("PATH STR {:?}", path_str);
    match re.captures_iter(path_str).next() {
        Some(m) => {
            match m.at(0) {
                Some(version) => Some(version.to_string()),
                None => None
            }
        },
        None => None
    }
}

pub fn ls_versions() -> Vec<String> {
    let home = setup::avm_directory();
    let mut paths = Vec::new();
    for path in fs::read_dir(home).unwrap() {
        let path_str = path.unwrap().path().display().to_string();
        if is_directory(&path_str) {
            paths.push(directory_name(&path_str));
        }
    }
    paths
}
