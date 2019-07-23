use std::path::{PathBuf, Path};
use std::fs;
use std::fs::{File};
use std::ffi::OsStr;
use serde_json::{Value, Map};
use std::io::Read;
use reqwest::{ Client };
use rayon::prelude::*;

use crate::config::Config;

//const HTTP_CLIENT: Client = Client::new();

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

fn insert_es(input: &Map<String, Value>, url: &String, client: &Client, file_name: &OsStr) {
    let response = client.post(url.as_str()).json(input).send();
    if response.is_err() {
        panic!("unable insert data file name {:?} with error {:?}", file_name, response.unwrap_err());
    }
}

pub fn read_json_files(cwd: PathBuf, config: Config) {
    let mut json_file_paths = get_all_json_file_paths(&cwd);
    json_file_paths.sort();
    let url =  format!("{url}{index}/_doc", url = config.url, index=config.index);
    let client = Client::new();
    for path in json_file_paths {
        let  file_name = match path.file_name() {
            Some(filename) => filename,
            None => OsStr::new("unKnown")
        };
        let mut json_data: Vec<Value> = match serde_json::from_str(read_json_file_content(&path).as_str()) {
            Ok(value) => value,
            Err(_err) => panic!("unable convert for fil {:?} with err {:?}", file_name, _err)
        };
        json_data.par_iter_mut().for_each(|payload| {
             match payload.as_object_mut() {
                Some(payload) => insert_es(payload, &url, &client, &file_name),
                None => println!("could not get data from {:?}", &file_name)
            }
        });
        println!("success for {:?}", &file_name);
    };
}