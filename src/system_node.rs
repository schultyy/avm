use std::env;
use std::io::Error;
use std::process::Command;

fn paths_without_avm() -> Vec<String> {
    let path_env = read_env().unwrap();
    remove_avm_paths(split_path(&path_env))
}


fn read_env() -> Option<String> {
    match env::var("PATH") {
        Ok(val) => Some(val),
        Err(_)  => None
    }
}

fn split_path(path: &String) -> Vec<String> {
    env::split_paths(path)
        .map(|p| String::from(p.to_str().unwrap()))
        .collect::<Vec<_>>()
}

fn remove_avm_paths(all_paths: Vec<String>) -> Vec<String> {
    all_paths.iter()
        .filter(|p| !p.contains(".avm"))
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
}


fn call_system_node(paths_without_avm: Vec<String>) -> Result<String, Error> {
    let output = Command::new("node")
        .arg("-v")
        .env("PATH", env::join_paths(paths_without_avm).unwrap())
        .output();
    match output {
        Ok(r) => Ok(String::from_utf8_lossy(&r.stdout).to_string()),
        Err(err) => return Err(err)
    }
}

pub fn system_node_path() -> Result<String, Error> {
    let output = Command::new("which")
        .arg("node")
        .env("PATH", env::join_paths(paths_without_avm()).unwrap())
        .output();
    match output {
        Ok(r) => {
            let shellout = String::from_utf8_lossy(&r.stdout);
            Ok(shellout.replace("\n", "").to_string())
        }
        Err(err) => return Err(err)
    }
}

pub fn version() -> Result<String, Error> {
    call_system_node(paths_without_avm())
}
