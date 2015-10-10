pub fn help() {
    println!("Usage:");
    println!("avm install <version>");
}

pub fn process_arguments(args: &Vec<String>) -> Option<String> {
    if args.len() < 2 {
        return None
    }
    let command = args[0].clone();
    if command == "install" {
        Some(args[1].clone())
    }
    else {
        None
    }
}
