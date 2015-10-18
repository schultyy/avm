mod cli;
mod symlink;
mod setup;
mod downloader;
mod archive_reader;
mod ls;
extern crate hyper;
extern crate regex;
extern crate os_type;
use std::env;

fn install(version: String) {
    let home_directory = match setup::prepare() {
        Ok(directory) => {
            println!("Prepared avm directory at {}", directory);
            directory
        },
        Err(err) => {
            println!("Failed to initialize home directory");
            println!("{:?}", err);
            std::process::exit(1)
        }
    };

    let archive_path = match downloader::download_source(&version, &home_directory) {
        Ok(path)  => path,
        Err(err)    => {
            println!("Download failed:\n{}", err);
            std::process::exit(1)
        }
    };
    println!("Wrote archive to {}", archive_path);
    let destination_path = setup::avm_directory();
    println!("Unzipping to {}", destination_path);

    match setup::create_version_directory(&version) {
        Ok(_) => { },
        Err(err) => {
            println!("Failed to create directory for version\n{}", err);
            std::process::exit(1)
        }
    };

    match archive_reader::decompress(&archive_path, destination_path, &version) {
        Ok(some) => {
            println!("Successfully unpacked archive");
            let stdout = String::from_utf8(some.stderr);
            println!("Exit status: {}", some.status);
            println!("{}", stdout.unwrap());
        },
        Err(err) => println!("Error occured\n{}", err)
    };

    match archive_reader::remove_archive_file(&archive_path) {
        Ok(_) => { },
        Err(err) => println!("Error occured while removing archive file\n{}", err)
    };
}

fn use_version(version: String) {
   if setup::has_version(&version) {
       match symlink::remove_symlink() {
           Err(err) => {
               if err.kind() != std::io::ErrorKind::NotFound {
                   println!("{:?}", err);
                   std::process::exit(1)
               }
           },
           _ => { }
       };
       match symlink::symlink_to_version(&version) {
           Ok(_) => println!("Now using node {}", version),
           Err(err) => {
               println!("{:?}", err);
               std::process::exit(1)
           }
       };
   } else {
       println!("Version {} not installed", version);
   }
}

fn list_versions() {
    let current_version = match ls::current_version() {
        Some(v) => v,
        None => String::new()
    };
    for version in ls::ls_versions() {
        if version == current_version {
            println!("=> {}", version);
        }
        else {
            println!("- {}", version);
        }
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
        cli::CmdOption::Help => {
            cli::help();
        }
        _ => {
            cli::help();
            std::process::exit(1)
        }
    };

}
