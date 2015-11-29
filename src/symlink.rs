use setup;
use std::path::Path;
#[cfg(not(windows))]
use std::os::unix::fs;
#[cfg(windows)]
use std::os::windows::fs;
use std::io::Error;
use ls;
use system_node;

fn create_bin_dir() -> Result<(), Error> {
    use std::fs;
    let avm_directory = setup::avm_directory();
    match setup::prepare() {
        Ok(_) => { },
        Err(err) => return Err(err)
    };
    let bin_directory = Path::new(&avm_directory).join("bin");
    fs::create_dir(bin_directory)
}

#[cfg(target_os= "windows")]
fn symlink<P: AsRef<Path>>(from: P, to: P) -> Result<(), Error> {
    fs::symlink_dir(from, to)
}

#[cfg(not(target_os="windows"))]
fn symlink<P: AsRef<Path>>(from: P, to: P) -> Result<(), Error> {
    fs::symlink(from, to)
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

    symlink(destination_bin_path, bin_directory)
}

fn symlink_to_system_binary(binary_name: String) -> Result<(), Error> {
    let avm_directory = setup::avm_directory();
    let bin_directory = Path::new(&avm_directory).join("bin");
    let local_binary = bin_directory.join(&binary_name);
    let system_binary_path = system_node::path_for_binary(binary_name).unwrap_or_else(|_| String::new());
    symlink(system_binary_path, local_binary.into_os_string().into_string().unwrap())
}

pub fn create_symlinks_to_system_binaries() -> Result<(), Error> {
    match create_bin_dir() {
        Ok(_) => { },
        Err(err) =>  return Err(err)
    }

    for binary in vec!["node", "npm"] {
        match symlink_to_system_binary(binary.into()) {
            Ok(_) => { },
            Err(err) => return Err(err)
        }
    }

    Ok(())
}
