use std::path::Path;
use std::fs;
use regex::Regex;
use node_version::NodeVersion;
use home_directory::HomeDirectory;

fn is_directory<P: AsRef<Path>>(path: P) -> bool {
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

fn is_version_directory(path: &String) -> bool {
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    re.is_match(path)
}

fn follow_symlink(home: &HomeDirectory) -> Option<String> {
    let path = fs::read_link(Path::new(&home.language_dir).join("bin"));
    if path.is_err() {
        match fs::read_link(Path::new(&home.language_dir).join("bin").join("node")) {
            Ok(p) => Some(p.as_os_str()
                          .to_str()
                          .unwrap()
                          .into()),
                          Err(_) => None
        }
    } else {
        Some(path.unwrap()
             .as_os_str()
             .to_str()
             .unwrap()
             .into())
    }
}

pub fn current_version(home: &HomeDirectory) -> Option<NodeVersion> {
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let path_str = match follow_symlink(home) {
        Some(p) => p,
        None => return None
    };
    match re.captures_iter(&path_str).next() {
        Some(m) => {
            match m.at(0) {
                Some(version) => {
                    Some(NodeVersion {
                        name: version.to_string(),
                        path: path_str.replace("/bin", "").to_string()
                    })
                }
                None => None
            }
        },
        None => Some(NodeVersion{
            name: path_str.to_string(),
            path: path_str.to_string()
        })
    }
}

pub fn ls_versions(home: &HomeDirectory) -> Vec<NodeVersion> {
    if !home.is_present() {
        return vec!();
    }
    let mut installed_versions = Vec::new();
    for path in fs::read_dir(&home.language_dir).unwrap() {
        let path_str = path.unwrap().path().display().to_string();
        if is_directory(&path_str) && is_version_directory(&path_str) {
            let version = NodeVersion{
                name: directory_name(&path_str),
                path: path_str.to_string()
            };
            installed_versions.push(version);
        }
    }
    installed_versions
}
