use home_directory::HomeDirectory;
use std::path::Path;
use std::os::unix::fs;
use std::io::Error;
use ls;
use system_node;

pub struct Symlink {
    home: HomeDirectory
}

impl Symlink {
    fn create_bin_dir(&self) -> Result<(), Error> {
        use std::fs;

        match self.home.prepare() {
            Ok(_) => { },
            Err(err) => return Err(err)
        };
        let bin_directory = Path::new(&self.home.language_dir).join("bin");
        fs::create_dir(bin_directory)
    }

    pub fn new(home: HomeDirectory) -> Symlink {
        Symlink {
            home: home
        }
    }

    pub fn points_to_version(&self, version: &String) -> bool {
        let ls = ls::Ls::new(self.home.clone());
        let current_version = match ls.current_version() {
            Some(v) => v,
            None => return false
        };

        &current_version.name == version
    }

    pub fn remove_symlink(&self) -> Result<(), Error> {
        use std::fs;
        let symlink_path = Path::new(&self.home.language_dir).join("bin");
        let remove_file_result = fs::remove_file(&symlink_path);
        if remove_file_result.is_err() {
            fs::remove_dir_all(&symlink_path)
        } else {
            remove_file_result
        }
    }

    pub fn symlink_to_version(&self, version_str: &String) -> Result<(), Error> {
        let destination_bin_path = Path::new(&self.home.language_dir)
            .join(version_str)
            .join("bin");
        let bin_directory = Path::new(&self.home.language_dir).join("bin");
        fs::symlink(destination_bin_path, bin_directory)
    }

    fn symlink_to_system_binary(&self, binary_name: String) -> Result<(), Error> {
        let bin_directory = Path::new(&self.home.language_dir).join("bin");
        let local_binary = bin_directory.join(&binary_name);
        let system_binary_path = system_node::path_for_binary(binary_name).unwrap_or_else(|_| String::new());
        fs::symlink(system_binary_path, local_binary)
    }


    pub fn create_symlinks_to_system_binaries(&self) -> Result<(), Error> {
        match self.create_bin_dir() {
            Ok(_) => { },
            Err(err) =>  return Err(err)
        }

        for binary in vec!["node", "npm"] {
            match self.symlink_to_system_binary(binary.into()) {
                Ok(_) => { },
                Err(err) => return Err(err)
            }
        }

        Ok(())
    }
}
