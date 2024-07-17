use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;

pub fn run(script: &str) {
    let filename = script.to_owned() + ".txt";
    let path = Path::new(&filename);

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("error: no script found");
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
                eprintln!("Failed to read line: {}", e);
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
}

pub fn add() {
    //creates a script 
    let mut script_lines: Vec<String> = Vec::new();

    print!("> script name --> ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");

    let name = name.trim();

    let mut path = String::new();

    print!("> path where script will execute --> ");
    io::stdout().flush().expect("failed to flush stdout");

    io::stdin()
    .read_line(&mut path)
    .expect("failed to read line");

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
            .expect("failed to read line");

        // Trim the trailing newline character
        let command = command.trim().to_string();

        // Break the loop if the input is empty
        if command.is_empty() {
            break;
        }

        script_lines.push(command);
    }

    let script_content = script_lines.join("\n");

    let mut file = File::create(name.to_owned() + ".txt").expect("failed to create file");

    // Write the content to the file
    file.write_all(script_content.as_bytes()).expect("failed to write to file");

    println!("script saved successfully (try running crow {})", name);
}

pub fn ls(script: Option<&str>) -> io::Result<()> {

    match script {
        Some(scr) => {
            println!("> reading contents of {} script", scr);

            match fs::read_to_string(format!("{}.txt", scr)) {
                Ok(contents) => {
                    println!("{}", contents);
                }
                Err(_e) => {
                    eprintln!("error: no script found");
                }
            }
        },
        None => {     
            let path = ".";

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
                eprintln!("error listing files: {}", e);
            }
        
            print!("> what script would you like to remove? --> ");
            io::stdout().flush().expect("failed to flush stdout");
        
            io::stdin()
                .read_line(&mut name)
                .expect("failed to read line");
        }
    }

    let filename = name.trim().to_owned() + ".txt"; 
    let path = Path::new(&filename);

    if !path.exists() {
        eprintln!("error: no script found");
        return;
    }

    match fs::remove_file(&path) {
        Ok(_) => println!("> removed successfully"),
        Err(e) => eprintln!("error deleting file: {}", e),
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