use niru::action;
use niru::action::CliAction as caction;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = args.get(1).map_or("", |s| s);

    let arg = args.get(2).map_or("".to_string(), |s| s.to_string());

    let action = match cmd {
        "push" => caction::Push(arg),
        "pull" => caction::Pull(arg),
        _ => caction::NotFound(cmd.to_owned()),
    };

    match action::evaluate(action) {
        Ok(_) => print!(""),
        Err(s) => eprintln!("{}", s),
    }
}
