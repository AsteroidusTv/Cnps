use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::io::prelude::*;

fn ask_string(message: &str) -> String {
    println!("{}", message);
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Failed to read line");
    line.trim().to_string()
}

fn find_directory(start_path: &Path, target_directory1: &str, target_directory2: &str) -> Option<PathBuf> {
    let entries = fs::read_dir(start_path).ok()?;
    
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();
        
        if path.is_dir() {
            if path.ends_with(target_directory1) || path.ends_with(target_directory2) {
                return Some(path);
            } else if let Some(found_path) = find_directory(&path, target_directory1, target_directory2) {
                return Some(found_path);
            }
        }
    }
    
    None
}

pub fn main() {
    let start_path = Path::new("/home/");
    let target_directory1 = "rust-sqlx";
    let target_directory2 = "rust-sqlx";
    
    if let Some(found_path) = find_directory(start_path, target_directory1, target_directory2) {
        println!("Found directory: {:?}", found_path);
        let folder_response = ask_string(format!("Is {:?} your project folder ? yN:", found_path).as_str()).to_lowercase();
        if folder_response == "y" {
            // add in config file
        }
        else {
            let folder_response = ask_string("Enter your projects folder : ");
            // add in config file
            println!("{}", folder_response)
        }
    }
         
     else {
        let folder_response = ask_string("Enter your projects folder : ");
        // add in config file
        println!("{}", folder_response)
    }
    

    loop {
        let editor_response = ask_string("What's your IDE subl/code (Sublime text / Vscode)").to_lowercase();
        if editor_response != "subl" && editor_response != "code" {
            println!("Invalid IDE response: {} you must choice 'subl' or 'code'", editor_response);
        }
        else {
            println!("{}", editor_response);
            break;
            // Add in config file
        }
    }
}