use std::path::Path;
use std::fs;
use std::env;
use std::io::Error;
use language::Language;

pub struct HomeDirectory {
    language: Language,
    pub language_dir: String
}

impl HomeDirectory {
    fn language_directory(lang_name: &str) -> String {
        let language_dir = Path::new(&HomeDirectory::avm_directory()).join(lang_name);
        language_dir.as_path()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn avm_directory() -> String {
        let home_directory = env::home_dir().unwrap();
        let avm = home_directory.join(".avm");
        avm.as_path().to_str().unwrap().to_string()
    }


    pub fn new(language: Language) -> HomeDirectory {
        HomeDirectory {
            language: language.clone(),
            language_dir: HomeDirectory::language_directory(language.name.clone())
        }
    }

    fn version_path(&self, version: &str) -> String {
        Path::new(&self.language_dir).join(version)
            .as_path()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn is_present(&self) -> bool {
        match fs::metadata(&self.language_dir) {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false
        }
    }

    pub fn prepare(&self) -> Result<(), Error> {
        if self.is_present() {
            return Ok(())
        }
        match fs::create_dir_all(&self.language_dir) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }

    pub fn create_version_directory(&self, version: &String) -> Result<String, Error> {
        let path = self.version_path(version);
        match fs::create_dir(&path) {
            Ok(_) => Ok(path.clone()),
            Err(err) => Err(err)
        }
    }

    pub fn has_version(&self, version_str: &str) -> bool {
        match fs::metadata(self.version_path(version_str)) {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false
        }
    }

    pub fn remove_version(&self, version_str: &str) -> Result<(), Error> {
        fs::remove_dir_all(self.version_path(version_str))
    }
}

