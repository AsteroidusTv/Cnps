use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

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
            } else if let Some(folder_path) = find_directory(&path, target_directory1, target_directory2) {
                return Some(folder_path);
            }
        }
    }

    None
}


pub fn main() {

    let start_path = Path::new("/home/");
    let target_directory1 = "projects";
    let target_directory2 = "Projects";

    if let Some(folder_path) = find_directory(start_path, target_directory1, target_directory2) {
        let folder_response = ask_string(format!("Is {:?} your project folder ? yN:", folder_path).as_str()).to_lowercase();
        if folder_response == "y" {
            // Add found_path to json
        }
        else {
            let _folder_path = ask_string("Enter your projects folder path : ");
            // Add entered_path to json
        }
    }
    else {
        let _folder_path = ask_string("Enter your projects folder path: ");
        // Add entered_path to json
    }

    loop {
        let editor_response = ask_string("What's your IDE subl/code (Sublime text / Vscode)").to_lowercase();
        if editor_response != "subl" && editor_response != "code" {
            println!("Invalid IDE response: {} you must choose 'subl' or 'code'", editor_response);
        }
        else {
            // Add editor response to json
        }
    }
}
    

