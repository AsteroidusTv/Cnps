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
