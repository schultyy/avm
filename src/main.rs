mod cli;
mod symlink;
mod downloader;
mod archive_reader;
mod ls;
mod system_node;
mod logger;
mod node_version;
mod commands;
mod language;
mod home_directory;
extern crate hyper;
extern crate regex;
extern crate os_type;
use std::env;

fn print_version() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    logger::stdout(format!("v{}", VERSION));
}

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    let cmd_args = cli::process_arguments(&args);

    match cmd_args.option
    {
        cli::CmdOption::Install => {
            let version = cmd_args.args.first().unwrap().clone();
            commands::node::install(&version);
        },
        cli::CmdOption::Use => {
            let version = cmd_args.args.first().unwrap().clone();
            commands::node::use_version(version);
        },
        cli::CmdOption::Ls => {
            commands::node::list_versions();
        },
        cli::CmdOption::Uninstall => {
            let version = cmd_args.args.first().unwrap().clone();
            commands::node::uninstall(version);
        },
        cli::CmdOption::Version => {
            print_version();
        },
        cli::CmdOption::Help => {
            cli::help();
        }
        _ => {
            cli::help();
        }
    };

}
