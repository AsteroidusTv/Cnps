use std::io;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::process::Command;


mod languages {
    pub mod html;
    pub mod rust;
    pub mod go;
}
mod config {
    pub mod config;
}
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

fn command_execute(command: &str, args: Vec<&str>) {
    let mut cmd = Command::new(command);
    cmd.args(args);
    
    let output = cmd.output().expect("Failed to execute the command.");

    if output.status.success() {
        println!("Project created successfully")
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command error: {}", stderr);
    }
}

fn change_directory(dir: &str) {
    let documents_dir = env::current_dir().unwrap().join(dir);
    env::set_current_dir(&documents_dir).expect("Imposible to set documents directory");
}

fn main() {

    // Get data from data.json
    config::config::main();
    let mut file = File::open("data.json").expect("Failed to open the file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read the file.");
    let data: Data = serde_json::from_str(&data).expect("Failed to deserialize JSON.");
    // Command to execute
    let git_command = "git";
    let git_args = vec!["init"];
    let editor_command = data.editor_response.as_str();
    let editor_args = vec!["."];

    // Change main directory to folder path from json file
    let folder_str = data.folder_path.to_str().unwrap_or("default_folder_path");
    change_directory(folder_str);

    // Ask for project name
    let project_name = ask_string("Please enter the project name");

    loop {
        let choice = ask_string("What language do you want to use for the project?, html/rust/go :").to_lowercase();

        if choice == "rust" {
            languages::rust::main(&project_name);
            change_directory(project_name.as_str());
            command_execute(git_command, git_args);
            command_execute(editor_command, editor_args);
            break;
        } else if choice == "html" {
            let with_js = ask_string("Use javascript in this project? y/N");
            languages::html::main(&project_name, &with_js);
            change_directory(project_name.as_str());
            command_execute(git_command, git_args);
            command_execute(editor_command, editor_args);
            break;

        } else if choice == "go" {
            languages::go::main(&project_name);
            change_directory(project_name.as_str());
            command_execute(git_command, git_args);
            command_execute(editor_command, editor_args);
            break;
        } else {
            println!("This language is not supported");
        }
    }
}






