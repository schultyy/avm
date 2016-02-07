use std::process::{Command, Output};
use std::path::Path;

pub fn call_configure_script(working_dir: &str, prefix_path: &str) -> Output {
    Command::new("./configure")
        .arg(format!("--prefix={}", prefix_path))
        .current_dir(Path::new(&working_dir))
        .output()
        .expect("Failed to call Configure script")
}
