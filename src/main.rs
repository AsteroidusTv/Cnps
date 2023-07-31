use std::io;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs;
use std::io::Read;
use std::process::Command;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

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
fn get_project_folders(paths: &str) -> Option<String> {
    let paths = fs::read_dir(paths).unwrap();
    let mut in_choices: Vec<String> = Vec::new();

    for path in paths {
        let entry = path.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        let path_components: Vec<&str> = path_str.split('/').collect();
        if let Some(last_folder) = path_components.last() {
            in_choices.push(last_folder.to_string());
        }
    }

    let choices: &Vec<String> = &in_choices;
    let mut choice = "";

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Subfolder to use")
        .items(choices)
        .default(0)
        .interact()
        .unwrap();

    for i in 0..in_choices.len() {
        if i == selection {
            choice = &in_choices[i];
            break;
        }
    }

    if !choice.is_empty() {
        Some(choice.to_string())
    } else {
        None
    }

}


fn main() {

    if !fs::metadata("data.json").is_ok() {
        config::config::main();
    }
    
    // Get data from data.json
    let mut file = File::open("data.json").expect("Failed to open the file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read the file.");
    let data: Data = serde_json::from_str(&data).expect("Failed to deserialize JSON.");

    // Change main directory to folder path from json file
    let folder_str = data.folder_path.to_str().unwrap_or("default_folder_path");
    change_directory(folder_str);

    // Ask for project name
    let project_name = ask_string("Please enter the project name :");

    let subfolder_response = ask_string("Would you like to create your project in a subfolder ?: y/n").to_lowercase();

    let open_args: Vec<String>; 

    if subfolder_response == "y" {
        if let Some(choice) = get_project_folders("/home/achille/Documents/Projects") {
            change_directory(choice.as_str());
            println!("Selected choice: {}", choice);
            open_args = vec![format!("{}/{}/{}/index.html", folder_str, choice, project_name)];
        } else {
            open_args = vec![format!("{}/{}/index.html", folder_str, project_name)];
    }
    } else {
        open_args = vec![format!("{}/{}/index.html", folder_str, project_name)];
    }
     
    // Command to execute
    let git_command = "git";
    let git_args = vec!["init"];
    let editor_command = data.editor_response.as_str();
    let editor_args = vec!["."];
    let open_command = "open";
    // For open args go to ligne 120
    

    // Convert
    let open_args_str: Vec<&str> = open_args.iter().map(|arg| arg.as_str()).collect();

    loop {
        
        let choices = &["rust", "html", "go"];
        let choice;
        let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Language to use")
        .items(choices)
        .default(0) 
        .interact()
        .unwrap();

        match selection {
            0 => choice = "rust",
            1 => choice = "html",
            2 => choice = "go",
            _ => unreachable!(),
        }

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
            command_execute(open_command, open_args_str);
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






