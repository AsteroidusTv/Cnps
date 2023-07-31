use crate::functions::functions;

pub fn main(project_name: &str) {
    let main_folder: String = String::from(project_name.clone());
    let assembly_file: String = String::from(format!("{}/main.asm", main_folder));

    let str_main_folder = main_folder.as_str();
    let str_assembly_file = assembly_file.as_str();

    functions::create_dir(str_main_folder);
    functions::create_file(str_assembly_file, "");

}