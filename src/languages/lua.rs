use crate::functions::functions;

pub fn main(project_name: &str) {
    let main_folder: String = String::from(project_name.clone());
    let lua_file: String = String::from(format!("{}/main.py", main_folder));

    let str_main_folder = main_folder.as_str();
    let str_lua_file = lua_file.as_str();

    functions::create_dir(str_main_folder);
    functions::create_file(str_lua_file, "");

}