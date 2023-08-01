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

mod functions {
    pub mod functions;
}
mod config {
    pub mod config;
}
mod languages {
    pub mod html;
    pub mod rust;
    pub mod go;
    pub mod python;
    pub mod assembly;
    pub mod lua;
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

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command error: {}", stderr);
    }
}
fn change_directory(dir: &str) {
    let documents_dir = env::current_dir().unwrap().join(dir);
    env::set_current_dir(&documents_dir).expect("Imposible to set documents directory");
}
fn get_project_folders(paths: &str) -> Option<String> {
    
    // Get all the folders in the project directory.
    let paths = fs::read_dir(paths).unwrap();
    // Initalize the vector
    let mut in_choices: Vec<String> = Vec::new();

    // Push the folder name into the vector.
    for path in paths {
        let entry = path.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        let path_components: Vec<&str> = path_str.split('/').collect();
        if let Some(last_folder) = path_components.last() {
            in_choices.push(last_folder.to_string());
        }
    }
    // Push the create subfolder to the vector.
    in_choices.push("Add a new subfolder".to_string());

    // Let choice = reference of in_choices
    let choices: &Vec<String> = &in_choices;
    let mut choice = "";

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Subfolder to use")
        .items(choices)
        .default(0)
        .interact()
        .unwrap();

    // Show all folder + create folder into the selection.
    for i in 0..in_choices.len() {
        if i == selection {
            choice = &in_choices[i];
            break;
        }
    }
    // Return
    if !choice.is_empty() {
        Some(choice.to_string())
    } else {
        None
    }

}

fn main() {

    // Run config.rs if .data.json doesn't exist
    if !fs::metadata(".data.json").is_ok() {
        config::config::main();
    }
    
    // Get data from data.json
    let mut file = File::open(".data.json").expect("Failed to open the file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read the file.");
    let data: Data = serde_json::from_str(&data).expect("Failed to deserialize JSON.");

    // Change main directory to folder path from json file
    let folder_str = data.folder_path.to_str().unwrap_or("default_folder_path");
    change_directory(folder_str);

    // Ask for project name
    let project_name = ask_string("Please enter the project name :");

    // Ask if the user wants to organize the project
    let subfolder_response = ask_string("Would you like to create your project in a subfolder ?: y/n").to_lowercase();

    // Initialize the vector for the open arguments command.
    let open_args: Vec<String>; 

    // Subfolder creation 
    if subfolder_response == "y" {
        if let Some(choice) = get_project_folders(folder_str) {
            if choice == "Add a new subfolder" {
                fs::create_dir_all(ask_string("Name for the new folder : ")).unwrap();
                if let Some(new_choice) = get_project_folders(folder_str) {
                    change_directory(new_choice.as_str());
                    println!("Selected choice: {}", new_choice);
                    open_args = vec![format!("{}/{}/{}/index.html", folder_str, new_choice, project_name)];
                } else {
                    // Handle the case where get_project_folders() returns None after creating a new subfolder
                    println!("Failed to get the newly created subfolder choice.");
                    return; // or perform other error handling
                }
            } else {
                change_directory(choice.as_str());
                println!("Selected choice: {}", choice);
                open_args = vec![format!("{}/{}/{}/index.html", folder_str, choice, project_name)];
            }
        } else {
            println!("Failed to get project folders.");
            return; // or perform other error handling
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
    

    // Selection 
    let choices = &["rust", "html", "go", "python", "assembly", "lua", ];
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
        3 => choice = "python",
        4 => choice = "assembly",
        5 => choice = "lua",
        _ => unreachable!(),
    }

    // Call main fucntion in the files for languages. 
    if choice == "rust" {
        languages::rust::main(&project_name);
    } else if choice == "html" {
        let with_js = ask_string("Use javascript in this project? y/N");
        languages::html::main(&project_name, &with_js);
        command_execute(editor_command, editor_args.clone());
        command_execute(open_command, open_args_str.clone());
    } else if choice == "go" {
        languages::go::main(&project_name);
    }  else if choice == "python"  {
        languages::python::main(&project_name);
    } else if choice == "assembly" {  
        languages::assembly::main(&project_name);
    } else if choice == "lua" {
        languages::lua::main(&project_name);
    }

    // git init + open IDE
    change_directory(project_name.as_str());
    command_execute(git_command, git_args.clone());
    command_execute(editor_command, editor_args.clone());

}






