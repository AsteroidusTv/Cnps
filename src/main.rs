use std::io;
use std::io::prelude::*;

mod languages {
    pub mod html;
    pub mod rust;
    pub mod go;
}
mod config {
    pub mod config;
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






