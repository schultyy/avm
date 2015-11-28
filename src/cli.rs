use logger;

pub enum CmdOption {
    Install,
    Use,
    Ls,
    Help,
    Uninstall,
    Version,
    Unknown
}

pub fn help() {
    logger::stdout(format!("Usage:\n"));
    logger::stdout(format!("Install a new version: "));
    logger::stdout(format!("avm install <version>\n"));
    logger::stdout(format!("Use a version: "));
    logger::stdout(format!("avm use <version>\n"));
    logger::stdout(format!("Use system version: "));
    logger::stdout(format!("avm use system\n"));
    logger::stdout(format!("List all installed versions:"));
    logger::stdout(format!("avm ls\n"));
    logger::stdout(format!("Uninstall a version:"));
    logger::stdout(format!("avm uninstall <version>\n"));
    logger::stdout(format!("Print version number:"));
    logger::stdout(format!("avm -v\n"));
    logger::stdout(format!("Print this help menu:"));
    logger::stdout(format!("avm help"));
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
    else if command == "uninstall" {
        Command{option: CmdOption::Uninstall, args: vec!(args[1].clone()) }
    }
    else if command == "-v" {
        Command{option: CmdOption::Version, args: vec!() }
    }
    else {
        Command { option: CmdOption::Unknown, args: vec!() }
    }
}
