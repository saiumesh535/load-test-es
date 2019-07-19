use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub index: String
}
