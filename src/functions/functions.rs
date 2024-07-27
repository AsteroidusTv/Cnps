use std::fs::File;
use std::io::Write;

pub fn create_dir(folder: &str) {
    match std::fs::create_dir(folder) {
        Ok(_) => {},
        Err(e) => {
            println!("Error creating folder {:?}: {}", folder, e);
            return;
        }
    }
}

pub fn create_file(file: &str, content: &str) {
    match File::create(&file) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                println!("Error writing to file {}", e);
            } else {
            }
        }
        Err(e) => {
            println!("Error creating file {} : {}", file, e);
        }
    }
}

pub fn simple_create(project_name: &str, extension: &str) {
    let main_folder: String = String::from(project_name);
    let language_file: String = String::from(format!("{}/main.{}", main_folder, extension));

    let str_main_folder = main_folder.as_str();
    let str_language_file = language_file.as_str();

    create_dir(str_main_folder);
    create_file(str_language_file, "");

}