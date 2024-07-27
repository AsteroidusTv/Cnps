use std::process::Command;
use std::env;

use crate::functions::functions;

pub fn main(project_name: &str) {
    let base_folder: String = String::from(project_name);
    let cmd_folder: String = String::from(format!("{}/cmd", base_folder));
    let main_folder: String = String::from(format!("{}/main", cmd_folder));
    let go_file: String = String::from(format!("{}/main.go", main_folder));

    let str_base_folder = base_folder.as_str();
    let str_cmd_folder = cmd_folder.as_str();
    let str_main_folder = main_folder.as_str();
    let str_go_file = go_file.as_str();

    functions::create_dir(str_base_folder);
    functions::create_dir(str_cmd_folder);
    functions::create_dir(str_main_folder);
    functions::create_file(str_go_file, "");

    if let Err(e) = env::set_current_dir(str_base_folder) {
        println!("Error changing directory: {}", e);
        return;
    }

    let command = "go";
    let args = vec!["mod", "init", project_name];

    let mut cmd = Command::new(command);
    cmd.args(args);

    let output = cmd.output().expect("Failed to execute the command.");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command error: {}", stderr);
    }
}

