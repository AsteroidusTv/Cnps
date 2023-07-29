use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Configuration {
    found_path: String,
    editor: String,
}

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

fn add_to_json(found_path: &str, editor: &str) {
    // Create an instance of Configuration with the provided values
    let config = Configuration {
        found_path: found_path.to_string(),
        editor: editor.to_string(),
    };

    // Convert the structure to JSON
    let serialized_config = serde_json::to_string(&config).unwrap();

    // Specify the path to the configuration file
    let file_path = "config.json";

    // Write the data to the configuration file
    let mut file = File::create(file_path).expect("Failed to create the file");
    file.write_all(serialized_config.as_bytes())
        .expect("Failed to write to the file");

    // Load the data from the configuration file
    let mut file = File::open(file_path).expect("Failed to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read from the file");

    // Deserialize the JSON data into the Configuration struct
    let deserialized_config: Configuration =
        serde_json::from_str(&content).expect("Failed to deserialize the data");

    // Use the deserialized_config as needed
    println!("Found Path: {}", deserialized_config.found_path);
    println!("Editor: {}", deserialized_config.editor);
}

pub fn main() {
    let start_path = Path::new("/home/");
    let target_directory1 = "projects";
    let target_directory2 = "Projects";

    if let Some(found_path) = find_directory(start_path, target_directory1, target_directory2) {
        println!("Found directory: {:?}", found_path);
        let folder_response = ask_string(format!("Is {:?} your project folder ? yN:", found_path).as_str()).to_lowercase();
        if folder_response == "y" {
            add_to_json(found_path.to_str().unwrap(), "code");
        }
        else {
            let folder_response = ask_string("Enter your projects folder path : ");
        }
    }
    else {
        let folder_response = ask_string("Enter your projects folder path: ");
        add_to_json(&folder_response, "");
    }

    loop {
        let editor_response = ask_string("What's your IDE subl/code (Sublime text / Vscode)").to_lowercase();
        if editor_response != "subl" && editor_response != "code" {
            println!("Invalid IDE response: {} you must choose 'subl' or 'code'", editor_response);
        }
            break;
        }
    }
    

