use std::process::Command;

pub fn main(project_name: &str) {
    let command = "cargo";
    let args = vec!["new", project_name];
    
    let mut cmd = Command::new(command);
    cmd.args(args);
    
    let output = cmd.output().expect("Failed to execute the command.");

    if output.status.success() {
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command error: {}", stderr);
    }
}
