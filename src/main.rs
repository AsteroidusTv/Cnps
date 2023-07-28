use std::io;
use std::io::prelude::*;

mod languages {
    pub mod html;
    pub mod rust;
}

fn main() {

    let choice = ask_string("What language do you want to use for the project?, html/rust").to_lowercase();
    let project_name = ask_string("Please enter the project name");
    
    
    if choice == "rust" {
        languages::rust::main(&project_name);
    }
    else if choice == "html" {
        let with_js = ask_string("Use javascript in this project? y/N");
        languages::html::main(&project_name, &with_js);
    }
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