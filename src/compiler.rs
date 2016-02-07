use std::process::{Command, Output};
use std::path::Path;

pub struct Compiler {
    working_dir: String
}

impl Compiler {
    pub fn new(working_dir: &str) -> Compiler {
        Compiler {
            working_dir: working_dir.into()
        }
    }

    pub fn call_configure_script(&self, prefix_path: &str) -> Output {
        Command::new("./configure")
            .arg(format!("--prefix={}", prefix_path))
            .current_dir(Path::new(&self.working_dir))
            .output()
            .expect("Failed to call Configure script")
    }
}
