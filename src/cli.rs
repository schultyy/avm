pub enum CmdOption {
    Install,
    Use,
    Ls,
    Unknown
}

pub fn help() {
    println!("Usage:");
    println!("avm install <version>");
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
    else {
        Command { option: CmdOption::Unknown, args: vec!() }
    }
}
