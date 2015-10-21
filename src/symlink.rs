use setup;
use std::path::Path;
use std::os::unix::fs;
use std::io::Error;

pub fn remove_symlink(executable: &String) -> Result<(), Error> {
    use std::fs;
    let symlink_path = Path::new(&setup::avm_directory())
        .join(executable)
        .as_path().to_str().unwrap().to_string();
    fs::remove_file(symlink_path)
}

pub fn symlink_to_version(version_str: &String, executable: String) -> Result<(), Error> {
    let executable_path= Path::new(&setup::avm_directory())
        .join(version_str)
        .join("bin")
        .join(&executable)
        .as_path().to_str().unwrap().to_string();

    let dest_executable_path = Path::new(&setup::avm_directory())
        .join(executable)
        .as_path().to_str().unwrap().to_string();
    fs::symlink(executable_path, dest_executable_path)
}
