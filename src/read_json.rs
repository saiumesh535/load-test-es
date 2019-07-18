use std::path::{PathBuf, Path};
use std::fs;
use std::fs::{File};
use std::ffi::OsStr;
use serde_json::{ Value };
use std::io::Read;

fn get_all_json_file_paths(cwd: &PathBuf) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let data_folder = cwd.join("src").join("data");
    let files = fs::read_dir(data_folder).expect("unable to locale data folder");
    for file in files {
        let file_entry = file.expect("unable to get file inside data folder");
        let path = &file_entry.path();
        let file_type = match Path::new(path).extension() {
            Some(t) => t,
            None => OsStr::new("")
        };
        if file_type == "json" {
            file_paths.push(file_entry.path())
        }
    };
    return file_paths;
}

fn read_json_file_content(path: &PathBuf) -> String {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_err) => panic!("unable to read file at path {}", _err)
    };
    let mut contents = String::new();
    let _ = match f.read_to_string(&mut contents) {
        Ok(bytes) => bytes,
        Err(_err) => panic!("unable to put content in string {}", _err)
    };
    contents
}

pub fn read_json_files(cwd: PathBuf) {
    let mut json_file_paths = get_all_json_file_paths(&cwd);
    let mut json_all_data: Vec<Vec<Value>> = Vec::new();
    json_file_paths.sort();
    for path in json_file_paths {
        let  file_name = path.file_name();
        let json_data: Vec<Value> = match serde_json::from_str(read_json_file_content(&path).as_str()) {
            Ok(value) => value,
            Err(_err) => panic!("unable convert for fil {:?} with err {:?}", file_name, _err)
        };
        json_all_data.push(json_data);
    };
    println!("{:?}", json_all_data);
}