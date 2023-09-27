
use std::path::PathBuf;
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





pub fn main() {

    let mut path_buf;

    loop {
        let path_str = ask_string("Enter your projects folder path : ");
        path_buf = PathBuf::from(&path_str);
        if path_buf.exists() {
            break;
            
        }
        println!("The folder that you entered doesn't exist :");
    }

    let mut data = Data {
        folder_path: path_buf,
        editor_response: String::new(),
    };

    let choices = &["Sublime text", "Vscode", "Notepad ++", "Intelij Idea"];
    let choice;
    let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("IDE to use")
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

    println!("Configuration succeeds")
}



