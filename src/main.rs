use std::io;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;


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

fn main() {
    config::config::main();
    let mut file = File::open("data.json").expect("Failed to open the file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read the file.");
    let data: Data = serde_json::from_str(&data).expect("Failed to deserialize JSON.");

    println!("Folder Path: {:?}", data.folder_path);
    println!("Editor Response: {}", data.editor_response);

let documents_dir = env::current_dir().unwrap().join(data.folder_path);
env::set_current_dir(&documents_dir).expect("Imposible to set documents directory");

let project_name = ask_string("Please enter the project name");

loop {
    let choice = ask_string("What language do you want to use for the project?, html/rust/go :").to_lowercase();

    if choice == "rust" {
        languages::rust::main(&project_name);
        break;
    } else if choice == "html" {
        let with_js = ask_string("Use javascript in this project? y/N");
        languages::html::main(&project_name, &with_js);
        break;
    } else if choice == "go" {
        languages::go::main(&project_name);
        break;
    } else {
        println!("This language is not supported");
    }
}
}






