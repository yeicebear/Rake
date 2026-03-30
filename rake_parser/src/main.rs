/*
* what this does is:
* basically it just does is
* okayt so what it does is
* it uhm
* loads a file 'Rakefile' in the working directory and tokenizes it.
FOLDER STRUCTURE:

workingDIR
    - Rakefile 
    - src/
    - bullshitFolder/
    - example_folder_really_extra/
So load Rakefile and its contents.

*/


use std::fs;
use regex::Regex;
use std::path::Path;


fn main() {
    let file_name = "Rakefile";
    
    if !Path::new(file_name).exists() {
        
        eprintln!("[ERR] NO {} FOUND IN THE WORKING DIR.", file_name);

        std::process::exit(1);
    }

    let content = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("[ERR] COULD NOT READ {}.", file_name);
            return;
        }
    };

    let re_header = Regex::new(r"\[(.*?)\]").unwrap();
    let re_command = Regex::new(r"\d+\)\s+(.*)").unwrap();

    let mut results = Vec::new();
    let mut current_header = String::new();

    for line in content.lines() {

        if let Some(cap) = re_header.captures(line) {

            current_header = cap[1].to_string();

        } else if let Some(cap) = re_command.captures(line) {

            let cmd_raw = cap[1].to_string();

            let parts: Vec<&str> = cmd_raw.split_whitespace().collect();
            
            if parts.len() >= 2 {
                let formatted = format!("{}, {}({})", current_header, parts[0], parts[1]);
                results.push(formatted);
            }
        }
    }


    for item in results {
        println!("{}", item);
    }
}
