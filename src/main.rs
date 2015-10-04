mod cli;
use std::env;

fn main() {
    println!("avm");
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    cli::process_arguments(program, args);
}
