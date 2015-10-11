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

    let home_directory = setup::prepare();
    println!("Prepared avm directory at {}", home_directory);

    downloader::download_source(version, &home_directory);
}
