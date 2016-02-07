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
  avm install <version>
  avm use (<version>|system)
  avm ls
  avm uninstall <version>
  avm autoselect
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
    flag_version: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_install {
        commands::node::install(&args.arg_version);
        process::exit(0);
    }

    if args.cmd_use && args.cmd_system {
        commands::node::use_version("system".into());
        process::exit(0);
    } else if args.cmd_use && !args.cmd_system {
        commands::node::use_version(args.arg_version.clone());
        process::exit(0);
    }

    if args.cmd_ls {
        commands::node::list_versions();
        process::exit(0);
    }

    if args.cmd_uninstall {
        commands::node::uninstall(args.arg_version.clone());
        process::exit(0);
    }

    if args.cmd_autoselect {
        commands::node::autoselect_version();
        process::exit(0);
    }

    if args.flag_version {
        println!("avm -- v{}", VERSION);
        process::exit(0);
    }

}
