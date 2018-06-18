use std::fs::File;
use std::io::Read;

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bnet_key: String,
    pub bnet_secret: String,
    db_name: String,
    db_user_name: String,
    db_password: String,
    db_host: String,
    db_port: u32,
    pub realm: String
}

impl Config {
    pub fn get_db_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", 
                self.db_user_name, 
                self.db_password, 
                self.db_host, 
                self.db_port,
                self.db_name
        )
    }
}

pub fn load_config() -> Config {
    let mut f = File::open("config.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}
