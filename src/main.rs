use std::fs::File;
use std::path::{PathBuf};
use std::env;
use std::io::Read;

mod read_json;
mod config;

use config::Config;


fn get_config(path: PathBuf) -> String {
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

fn main() {
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(_err) => panic!("unable to get CWD {}", _err)
    };
    let config_file_path = cwd.join("src").join("config.json");
    let config: Config = match serde_json::from_str(get_config(config_file_path).as_str()) {
        Ok(data) => data,
        Err(err) => panic!("{:?}", err)
    };
    read_json::read_json_files(cwd, config);
}
