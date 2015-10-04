extern crate getopts;
use self::getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} install <version>", program);
    print!("{}", opts.usage(&brief));
}

fn install_version(version_str: &String) {
    println!("Installing node {}", version_str);
}

pub fn process_arguments(program: String, args: Vec<String>) {
    let mut opts = Options::new();
    opts.optopt("", "install", "", "VERSION");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    println!("{:?}", matches.free);

    if !matches.opt_present("install") {
        print_usage(&program, opts);
        return;
    }

    let input = if matches.free.len() > 1 {
        matches.free[1].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    install_version(&input);
}
