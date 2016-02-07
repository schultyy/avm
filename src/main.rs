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
mod autoselect;
mod compiler;
extern crate hyper;
extern crate regex;
extern crate os_type;
extern crate rustc_serialize;
extern crate semver;
extern crate docopt;

use docopt::Docopt;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
avm

Usage:
  avm install <language> <version>
  avm use <language> (<version>|system)
  avm ls <language>
  avm uninstall <language> <version>
  avm autoselect <language>
  avm [options]

Options:
  -h --help
  --version
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_ls: bool,
    cmd_use: bool,
    cmd_system: bool,
    cmd_install: bool,
    cmd_uninstall: bool,
    cmd_autoselect: bool,
    arg_version: String,
    arg_language: String,
    flag_version: bool
}

fn handle_node(args: Args) {
    if args.cmd_install {
        commands::node::install(&args.arg_version);
    }

    if args.cmd_use && args.cmd_system {
        commands::node::use_version("system".into());
    } else if args.cmd_use && !args.cmd_system {
        commands::node::use_version(args.arg_version.clone());
    }

    if args.cmd_ls {
        commands::node::list_versions();
    }

    if args.cmd_uninstall {
        commands::node::uninstall(args.arg_version.clone());
    }

    if args.cmd_autoselect {
        commands::node::autoselect_version();
    }
}

fn handle_ruby(args: Args) {
    if args.cmd_install {
        commands::ruby::install(&args.arg_version);
    }
    if args.cmd_use {
        commands::ruby::use_version(args.arg_version.clone());
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        logger::stdout(format!("avm -- v{}", VERSION));
        process::exit(0);
    }

    match &args.arg_language[..] {
        "ruby" => handle_ruby(args),
        "node" => handle_node(args),
        _ => {
            logger::stderr(format!("Unsupported language {}", args.arg_language));
            process::exit(1)
        }
    }
}
