mod cli;
use std::process;
use std::env;

fn main() {
    println!("avm");
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    let version = match cli::process_arguments(&args) {
        Some(version) => version,
        None => {
            cli::help();
            std::process::exit(1)
        }
    };
}
