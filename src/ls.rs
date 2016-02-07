use std::path::Path;
use std::fs;
use regex::Regex;
use node_version::NodeVersion;
use home_directory::HomeDirectory;

pub struct Ls {
    home: HomeDirectory
}

impl Ls {
    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool {
        match fs::metadata(path) {
            Ok(metadata) => metadata.is_dir(),
            Err(_)       => false
        }
    }

    fn directory_name(&self, full_path: &String) -> String {
        let components = Path::new(full_path).components();
        components.last().unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .into()
    }

    fn is_version_directory(&self, path: &String) -> bool {
        let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
        re.is_match(path)
    }

    fn follow_symlink(&self) -> Option<String> {
        let path = fs::read_link(Path::new(&self.home.language_dir).join("bin"));
        if path.is_err() {
            match fs::read_link(Path::new(&self.home.language_dir).join("bin").join("node")) {
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

    pub fn new(home: HomeDirectory) -> Ls {
        Ls {
            home: home
        }
    }

    pub fn current_version(&self) -> Option<NodeVersion> {
        let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
        let path_str = match self.follow_symlink() {
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

    pub fn ls_versions(&self) -> Vec<NodeVersion> {
        if !self.home.is_present() {
            return vec!();
        }
        let mut installed_versions = Vec::new();
        for path in fs::read_dir(&self.home.language_dir).unwrap() {
            let path_str = path.unwrap().path().display().to_string();
            if self.is_directory(&path_str) && self.is_version_directory(&path_str) {
                let version = NodeVersion{
                    name: self.directory_name(&path_str),
                    path: path_str.to_string()
                };
                installed_versions.push(version);
            }
        }
        installed_versions
    }
}
