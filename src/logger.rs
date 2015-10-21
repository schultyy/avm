use std::convert::AsRef;
extern crate term;
use std::io::prelude::*;

pub enum MessageType {
    Stdout,
    Stderr
}

pub fn stdout<P: AsRef<str>>(message: P) {
    print_message(message, MessageType::Stdout);
}

pub fn stderr<P: AsRef<str>>(message: P) {
    print_message(message, MessageType::Stderr);
}

pub fn print_message<P: AsRef<str>>(message: P, message_type: MessageType) {
    let mut stdout_terminal = term::stdout().unwrap();
    let mut stderr_terminal = term::stderr().unwrap();
    stderr_terminal.fg(term::color::RED).unwrap();
    stdout_terminal.fg(term::color::GREEN).unwrap();

    match message_type {
        MessageType::Stdout => {
            writeln!(stdout_terminal, "{}", message.as_ref()).unwrap();
        },
        MessageType::Stderr => {
            writeln!(stderr_terminal, "{}", message.as_ref()).unwrap();
        }
    }
    stderr_terminal.reset().unwrap();
    stdout_terminal.reset().unwrap();
}
