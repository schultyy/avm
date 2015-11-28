use setup;
use std::path::Path;
use std::os::unix::fs;
use std::io::Error;
use ls;
use system_node;

fn create_bin_dir() {
    use std::fs;
    let avm_directory = setup::avm_directory();
    setup::prepare();
    let bin_directory = Path::new(&avm_directory).join("bin");
    fs::create_dir(bin_directory);
}

pub fn points_to_version(version: &String) -> bool {
    let current_version = match ls::current_version() {
        Some(v) => v,
        None => return false
    };

    &current_version.name == version
}

pub fn remove_symlink() -> Result<(), Error> {
    use std::fs;
    let symlink_path = Path::new(&setup::avm_directory()).join("bin");
    let remove_file_result = fs::remove_file(&symlink_path);
    if remove_file_result.is_err() {
        fs::remove_dir_all(&symlink_path)
    } else {
        remove_file_result
    }
}

pub fn symlink_to_version(version_str: &String) -> Result<(), Error> {
    let avm_directory = setup::avm_directory();
    let destination_bin_path = Path::new(&avm_directory)
                                        .join(version_str)
                                        .join("bin");
    let bin_directory = Path::new(&avm_directory).join("bin");
    fs::symlink(destination_bin_path, bin_directory)
}

pub fn symlink_to_system_node() -> Result<(), Error> {
    let avm_directory = setup::avm_directory();
    let bin_directory = Path::new(&avm_directory).join("bin");
    create_bin_dir();

    let local_node  = bin_directory.join("node");
    let system_node_path = system_node::system_node_path().unwrap_or_else(|err| String::new());
    fs::symlink(system_node_path, local_node)
}

