use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;
use colored::*;

pub fn run(script: &str) {
    let filename = script.to_owned() + ".txt";

    let appdata_dir = dirs::data_local_dir().expect("could not find the AppData directory");

    let mut path = PathBuf::from(appdata_dir);
    path.push(".crow");
    path.push(filename);

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{} {}", "error: no script found".red(), e);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    let mut command_args: Vec<String> = Vec::new();

    let mut path: Option<String> = None;

    for line in reader.lines() {

        match line {
            Ok(line_content) => {
                if line_content.starts_with("cd ") {
                    path = Some(line_content[3..].trim_start().to_string());
                } else {
                    command_args.push(line_content);
                }
            }
            Err(e) => {
                eprintln!("{} {}", "error: failed to read line".red(), e);
            }
        }
    }

    let command_str = command_args.join("&&");

    let title = format!("crow {}", script);

    let mut command_args_vec = vec![
        "wt", "-w", "0", "nt", "--title", &title, "--suppressApplicationTitle", "cmd", "/k", &command_str,
    ];

    if let Some(ref path) = path {
        command_args_vec.insert(7, "-d"); // Insert -d before the path
        command_args_vec.insert(8, path);
    }

    Command::new("wt")
    .args(command_args_vec)
    .spawn()
    .unwrap();

    println!("{}", format!("executing crow {}", script).black());
}

pub fn add() {
    //creates a script 
    let mut script_lines: Vec<String> = Vec::new();

    print!("> script name --> ");
    io::stdout().flush().expect(&"error: failed to flush stdout".red().to_string());

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect(&"error: failed to read line".red().to_string());

    let name = name.trim();

    let mut path = String::new();

    print!("> path where script will execute --> ");
    io::stdout().flush().expect(&"error: failed to flush stdout".red().to_string());

    io::stdin()
    .read_line(&mut path)
    .expect(&"error: failed to read line".red().to_string());

    let path = path.trim().to_string();

    let path = if path.is_empty() { 
        ".".to_string() 
    } else { 
        path 
    };

    script_lines.push(format!("cd {}", path));

    println!("> command list ");

    loop {

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect(&"error: failed to read line".red().to_string());

        // Trim the trailing newline character
        let command = command.trim().to_string();

        // Break the loop if the input is empty
        if command.is_empty() {
            break;
        }

        script_lines.push(command);
    }

    let script_content = script_lines.join("\n");

    let appdata_dir = dirs::data_local_dir().expect(&"error: could not find the AppData directory".red().to_string());

    let mut file_path = PathBuf::from(appdata_dir);
    file_path.push(".crow");
    file_path.push(name.to_owned() + ".txt");

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect(&"error: could not create .crow directory".red().to_string());
    }

    let mut file = File::create(file_path).expect(&"error: failed to create file".red().to_string());

    // Write the content to the file
    file.write_all(script_content.as_bytes()).expect(&"error: failed to write to file".red().to_string());

    println!("script saved successfully (try running crow {})", name);
}

pub fn ls(script: Option<&str>) -> io::Result<()> {

    match script {
        Some(scr) => {
            println!("> reading contents of {} script", scr);

            let appdata_dir = dirs::data_local_dir().expect(&"error: could not find the AppData directory".red().to_string());

            let mut path = PathBuf::from(appdata_dir);
            path.push(".crow");
            path.push(scr.to_owned() + ".txt");

            match fs::read_to_string(path) {
                Ok(contents) => {
                    println!("{}", contents);
                }
                Err(_e) => {
                    eprintln!("{}", "error: no script found".red());
                }
            }
        },
        None => {     

            let appdata_dir = dirs::data_local_dir().expect("could not find the AppData directory");

            let mut path = PathBuf::from(appdata_dir);
            path.push(".crow");

            let entries = fs::read_dir(path)?
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().extension().map(|ext| ext == "txt").unwrap_or(false))
                .collect::<Vec<_>>();

            if entries.is_empty() {
                return Ok(());
            }

            println!("> list of scripts");

            for entry in entries {
                let file_name = entry.file_name();

                if let Some(name_str) = file_name.to_str() {
                    let name_without_extension = &name_str[..name_str.len() - 4];
                    println!("{}", name_without_extension);
                }
            }
        }
    };

    Ok(())
}


pub fn remove(script: Option<&str>) {

    let mut name = String::new();

    match script {
        Some(scr) => name = scr.to_string(),
        None => {
            if let Err(e) = ls(None) {
                eprintln!("{} {}", "error: listing files".red(), e);
            }
        
            print!("> what script would you like to remove? --> ");
            io::stdout().flush().expect(&"failed to flush stdout".red().to_string());
        
            io::stdin()
                .read_line(&mut name)
                .expect(&"error: failed to read line".red().to_string());
        }
    }

    let filename = name.trim().to_owned() + ".txt"; 

    let appdata_dir = dirs::data_local_dir().expect(&"error: could not find the AppData directory".red().to_string());

    let mut path = PathBuf::from(appdata_dir);
    path.push(".crow");
    path.push(filename);

    if !path.exists() {
        eprintln!("{}", "error: no script found".red());
        return;
    }

    match fs::remove_file(&path) {
        Ok(_) => println!("> removed successfully"),
        Err(e) => { eprintln!("{} {}", "error: deleting file".red(), e); }
    }
}

pub fn help() {
    eprintln!("
usage: 
crow <scriptname> -> runs a script
crow add -> adds a new script
crow ls <scriptname> -> lists all scripts or the contents of a script
crow remove <scriptname> -> removes a script
crow help -> current page
");
}