use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

#[derive(Serialize, Deserialize)]
struct Data {
    folder_path: PathBuf,
    editor_response: String,
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
            } else if let Some(folder_path) = find_directory(&path, target_directory1, target_directory2) {
                return Some(folder_path);
            }
        }
    }

    None
}

fn folder_exists(file: &PathBuf) -> bool {
    file.exists()
}

pub fn main() {
    let start_path = Path::new("/home/");
    let target_directory1 = "projects";
    let target_directory2 = "Projects";

    let folder_path = if let Some(path) = find_directory(start_path, target_directory1, target_directory2) {
        let folder_response = ask_string(format!("Is {:?} your project folder ? yN:", path).as_str()).to_lowercase();
        if folder_response == "y" {
            path
        } else {
            loop {
                let path_str = ask_string("Enter your projects folder path : ");
                let path_buf = PathBuf::from(&path_str);
                if folder_exists(&path_buf) {
                    break path_buf;
                }
                println!("The folder that you entered doesn't exist :");
            }
        }
    } else {
        loop {
            let path_str = ask_string("Enter your projects folder path : ");
            let path_buf = PathBuf::from(&path_str);
            if folder_exists(&path_buf) {
                break path_buf;
            }
            println!("The folder that you entered doesn't exist :");
        }
    };

    let mut data = Data {
        folder_path,
        editor_response: String::new(),
    };

    let choices = &["Sublime text", "Vscode", "Notepad ++", "Intelij Idea"];
    let choice;
    let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Language to use")
    .items(choices)
    .default(0) 
    .interact()
    .unwrap();

    match selection {
        0 => choice = "subl",
        1 => choice = "code",
        2 => choice = "start notepad++",
        3 => choice = "idea",
        _ => unreachable!(),
    }

    data.editor_response = choice.to_string();

    let json_string = serde_json::to_string(&data).expect("Failed to serialize data to JSON");
    let file_path = ".data.json";
    let mut file = File::create(file_path).expect("Failed to create file");

    file.write_all(json_string.as_bytes()).expect("Failed to write data to file");
}



