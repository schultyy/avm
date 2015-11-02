mod cli;
mod symlink;
mod setup;
mod downloader;
mod archive_reader;
mod ls;
mod logger;
extern crate hyper;
extern crate regex;
extern crate os_type;
use std::env;

fn install(version: String) {
    let home_directory = match setup::prepare() {
        Ok(directory) => {
            logger::stdout(format!("Prepared avm directory at {}", directory));
            directory
        },
        Err(err) => {
            logger::stderr("Failed to initialize home directory");
            logger::stderr(format!("{:?}", err));
            std::process::exit(1)
        }
    };

    if setup::has_version(&version) {
        logger::stderr(format!("Version {} is already installed", version));
        std::process::exit(1)
    }

    let archive_path = match downloader::download_source(&version, &home_directory) {
        Ok(path)  => path,
        Err(err)    => {
            logger::stderr(format!("Download failed:\n{}", err));
            std::process::exit(1)
        }
    };
    let destination_path = setup::avm_directory();
    logger::stdout(format!("Unzipping to {}", destination_path));

    match setup::create_version_directory(&version) {
        Ok(_) => { },
        Err(err) => {
            logger::stderr(format!("Failed to create directory for version\n{}", err));
            std::process::exit(1)
        }
    };

    match archive_reader::decompress(&archive_path, destination_path, &version) {
        Ok(_) => { },
        Err(err) => logger::stderr(format!("Error occured\n{}", err))
    };

    match archive_reader::remove_archive_file(&archive_path) {
        Ok(_) => { },
        Err(err) => logger::stderr(format!("Error occured while removing archive file\n{}", err))
    };

    logger::stdout(format!("Successfully installed version {}", version));
    logger::stdout(format!("Run avm use {} to use it", version));
}

fn use_version(version: String) {
    if setup::has_version(&version) {
        for executable in vec!["node", "npm"] {
            match symlink::remove_symlink(&executable.to_string()) {
                Err(err) => {
                    if err.kind() != std::io::ErrorKind::NotFound {
                        logger::stderr(format!("Failed to remove symlink {}", executable));
                        logger::stderr(format!("{:?}", err));
                        std::process::exit(1)
                    }
                },
                _ => { }
            };

            match symlink::symlink_to_version(&version, executable.to_string()) {
                Ok(_) => logger::stdout(format!("Now using {} {}", executable, version)),
                Err(err) => {
                    logger::stderr(format!("Failed to set symlink for {}", executable));
                    logger::stderr(format!("{:?}", err));
                    std::process::exit(1)
                }
            };
        }
    } else {
        logger::stdout(format!("Version {} not installed", version));
        std::process::exit(1)
    }
}

fn list_versions() {
    let current_version = match ls::current_version() {
        Some(v) => v,
        None => String::new()
    };
    logger::stdout(format!("Listing all installed versions:"));
    logger::stdout(format!("(=>): current version"));
    for version in ls::ls_versions() {
        if version == current_version {
            logger::stdout(format!("=> {}", version));
        }
        else {
            logger::stdout(format!("- {}", version));
        }
    }
}

fn uninstall(version: String) {
    if setup::has_version(&version) {

        if symlink::points_to_version(&version) {
            for executable in vec!["node", "npm"] {
                match symlink::remove_symlink(&executable.to_string()) {
                    Err(err) => {
                        if err.kind() != std::io::ErrorKind::NotFound {
                            logger::stderr(format!("Failed to remove symlink {}", executable));
                            logger::stderr(format!("{:?}", err));
                            std::process::exit(1)
                        }
                    },
                    _ => { }
                }
            }
        }

        match setup::remove_version(&version) {
            Ok(_) => logger::stdout(format!("Successfully removed version {}", version)),
            Err(err) => {
                logger::stderr(format!("Failed to remove version {}", version));
                logger::stderr(format!("{:?}", err));
            }
        }
    }
    else {
        logger::stderr(format!("Version {} is not installed", version));
        std::process::exit(1)
    }
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
            install(version);
        },
        cli::CmdOption::Use => {
            let version = cmd_args.args.first().unwrap().clone();
            use_version(version);
        },
        cli::CmdOption::Ls => {
            list_versions();
        },
        cli::CmdOption::Uninstall => {
            let version = cmd_args.args.first().unwrap().clone();
            uninstall(version);
        },
        cli::CmdOption::Help => {
            cli::help();
        }
        _ => {
            cli::help();
        }
    };

}
