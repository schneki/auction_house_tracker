use std::fs::File;
use std::io::Read;

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bnet_key: String,
    pub bnet_secret: String,
    pub realm: String
}

pub fn load_config() -> Config {
    let mut f = File::open("config.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}
