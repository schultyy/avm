pub enum CmdOption {
    Install,
    Use,
    Ls,
    Help,
    Unknown
}

pub fn help() {
    println!("Usage:\n");
    println!("Install a new version: ");
    println!("avm install <version>\n");
    println!("Use a version: ");
    println!("avm use <version>\n");
    println!("List all installed versions:");
    println!("avm ls\n");
    println!("Print this help menu:");
    println!("avm help");
}

pub struct Command {
    pub option: CmdOption,
    pub args: Vec<String>
}

pub fn process_arguments(args: &Vec<String>) -> Command {
    if args.len() < 1 {
        return Command { option: CmdOption::Unknown, args: vec!() }
    }
    let command = args[0].clone();
    if command == "install" {
        Command{option: CmdOption::Install, args: vec!(args[1].clone()) }
    }
    else if command == "use" {
        Command{option: CmdOption::Use, args: vec!(args[1].clone()) }
    }
    else if command == "ls" {
        Command{option: CmdOption::Ls, args: vec!()}
    }
    else if command == "help" {
        Command{option: CmdOption::Help, args: vec!()}
    }
    else {
        Command { option: CmdOption::Unknown, args: vec!() }
    }
}
