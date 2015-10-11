mod cli;
mod setup;
mod downloader;
extern crate hyper;
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

    match setup::prepare() {
        Ok(path) => {
            println!("Prepared avm directory at {}", path)
        },
        Err(err) => {
            println!("Could not prepare directory");
            println!("Reason: {}", err);
        }
    };
    // downloader::download_source(version);
}
