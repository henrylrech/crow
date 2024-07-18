use std::env::{self};
use crow;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && &args[1] == "add" {
        crow::add();
        return;
    } else if args.len() >= 2 && &args[1] == "ls" {
        let n = args.len();

        match n {
            2 => crow::ls(None).expect("error on ls command with no argument"),
            3 => crow::ls(Some(&args[2])).expect("error on ls command with argument"),
            _ => crow::help(),
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

    let joined_args = args[1..].join(" ");

    println!("{}", &joined_args);

    crow::run(&joined_args)
}

