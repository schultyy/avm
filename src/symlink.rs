use setup;
use std::path::{Path, PathBuf};
use std::os::unix::fs;
use std::io::Error;

pub fn remove_symlink(version_str: &String) {
    use std::fs;
    let symlink_path = Path::new(&setup::avm_directory())
        .join("node")
        .as_path().to_str().unwrap().to_string();
    fs::remove_file(symlink_path);
}

pub fn symlink_to_version(version_str: &String) -> Result<(), Error> {
    let node_executable_path= Path::new(&setup::avm_directory())
        .join(version_str)
        .join("bin")
        .join("node")
        .as_path().to_str().unwrap().to_string();

    let dest_node_executable_path = Path::new(&setup::avm_directory())
        .join("node")
        .as_path().to_str().unwrap().to_string();
    fs::symlink(node_executable_path, dest_node_executable_path)
}
