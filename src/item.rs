use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use postgres::Connection;


#[derive(Serialize, Deserialize)]
pub struct Item {
    id: i32,
    name: String,
    description: String,
    stackable: i16,
    icon: String
}
impl Item {
    pub fn insert(&self, conn: &Connection) {
        match conn.execute("
            INSERT INTO item
                (id, name, description, stackable, icon)
            VALUES
                ($1, $2, $3, $4, $5)
            ON CONFLICT (id)
                DO NOTHING",
                &[
                    &self.id, &self.name, &self.description, &self.stackable, &self.icon
                ] 
        ) 
        {
            Ok(res) => {},
            Err(err) => println!("{}", err)
        }
    }
}

pub fn insert_items(conn: &Connection, items: Vec<Item>) {
    for item in items {
        item.insert(&conn)
    }
}

pub fn get_item_ids() -> Vec<i32> {
    let f = File::open("ids.txt").unwrap();
    let mut ids = Vec::new();
    for line in BufReader::new(f).lines() {
        match line.unwrap().parse() {
            Ok(id) => ids.push(id),
            Err(_) => {}
        }
    }
    ids 
}

pub fn get_items(ids: &Vec<i32>, key: &str) -> Vec<Item> {
    let mut items = Vec::new();
    for id in ids {
        items.push(get_item(id, key))
    }
    items

}

use util;

pub fn get_item(id: &i32, key: &str) -> Item {
    let base = "https://eu.api.battle.net/wow/item/";
    let url = format!("{}{}?locale=en_GB&apikey={}", base, id, key);
    let data = util::get_url_content_https(&url);
    serde_json::from_str(&data).unwrap()
}
