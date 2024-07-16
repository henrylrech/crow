use std::env::{self};
use crow;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || &args[1] == "help" {
        crow::help();
        return;
    }

    if &args[1] == "add" {
        crow::add();
        return;
    }

    if &args[1] == "ls" {
        if let Err(e) = crow::ls() {
            eprintln!("error: {}", e);
        }
        return;
    }

    if &args[1] == "remove" {
        crow::remove();
        return;
    }

    crow::run(&args[1])
}

