use std::env::{Args, args};

mod cmds;
use crate::cmds::cmds as commands;

#[derive(Debug)]
struct Command {
    name : String,
    arg : Option<String>
}

fn main() {
    let mut args = args();
    let cmd = parse_args(&mut args);

    match cmd.name.as_str() {
        "help" => commands::help(cmd.arg),
        "list" => commands::list(cmd.arg),
        "add" => commands::add(cmd.arg),
        "mark" => commands::mark(cmd.arg),
        "remove" => commands::remove(),
        other => println!("Unknown Command {}!", other)
    };
}

fn parse_args(args: &mut Args) -> Command {
    let n = args.len();
    if n > 3 {
        panic!("Expected less than 3 arguments but recieved {} arguments. If you meant for a value, try wrapping it inside quotes.", n);
    }
    
    let cmd = args.nth(1)
        .unwrap_or("help".to_string());
    let val = args.next();

    Command {
        name: cmd,
        arg: val
    }
}