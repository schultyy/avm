use std::path::{PathBuf, Path};
use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::fs;
use rustc_serialize::json::Json;
use semver::Version;
use semver::VersionReq;


fn match_current_node(package_version: String, installed_versions: Vec<String>) -> Option<String> {
    let installed_semver = installed_versions.iter().map(|v| Version::parse(&v).unwrap());
    let version_requirement = VersionReq::parse(&package_version).unwrap();

    for installed in installed_semver {
        if version_requirement.matches(&installed) {
            return Some(installed.to_string())
        }
    }
    None
}

fn match_legacy_node(package_version: String, installed_versions: Vec<String>) -> Option<String> {
    let installed_vers = installed_versions.iter().map(|v| Version::parse(&v).unwrap());
    let version_requirement = match Version::parse(&package_version) {
        Ok(v)  => v,
        Err(_) => return None
    };

    for installed in installed_vers {
        if version_requirement == installed {
            return Some(installed.to_string())
        }
    }
    None
}

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
            Some(version) => Ok(version.replace("\"", "")),
            None => Err("No node engine specified".to_string())
        }
    }

    pub fn match_version(&self, package_version: String, installed_versions: Vec<String>) -> Option<String> {
        let version = Version::parse(&package_version).unwrap();
        if version.major == 0 {
            match_legacy_node(package_version, installed_versions)
        }
        else {
            match_current_node(package_version, installed_versions)
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
