use std::env::{self};
use crow;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && &args[1] == "add" {
        crow::add();
        return;
    } else if args.len() == 2 && &args[1] == "ls" {
        if let Err(e) = crow::ls() {
            eprintln!("error: {}", e);
        }
        return;
    } else if args.len() >= 2 && &args[1] == "remove" {
        let n = args.len();
        match n {
            2 => crow::remove(None),
            3 => crow::remove(Some(&args[2])),
            _ => crow::help(),
        }
        
        return;
    } else if args.len() < 2 || &args[1] == "help" {
        crow::help();
        return;
    } 

    crow::run(&args[1])
}

