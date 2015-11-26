use std::env;
use std::io::Error;
use std::process::Command;

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

fn set_new_path_variable(paths: Vec<String>) {
    let new_path = env::join_paths(paths).unwrap();
    env::set_var("PATH", &new_path);
}

fn call_system_node() -> Result<String, Error> {
    let output = match Command::new("node").arg("-v").output() {
        Ok(r) => r,
        Err(err) => return Err(err)
    };
    let foo = String::from_utf8_lossy(&output.stdout);
    Ok(foo.to_string())
}

pub fn version() -> Result<String, Error> {
    let path_env = read_env().unwrap();
    let paths_without_avm = remove_avm_paths(split_path(&path_env));
    set_new_path_variable(paths_without_avm);
    call_system_node()
}
