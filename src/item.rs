use reqwest;
use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use diesel::{self, PgConnection};
use models::item::{Item, NewItem};
use diesel::RunQueryDsl;

use diesel::prelude::*;


impl Item {
    pub fn insert(&self, conn: &PgConnection) {
        use schema::item;

        let new_item = NewItem::from(self);

        match diesel::insert_into(item::table)
            .values(&new_item)
            .execute(conn)
        {
            Ok(_) => {},
            Err(_) => {}
        }
    }
}

pub fn insert_items(conn: &PgConnection, items: &Vec<Item>) {
    for item in items {
        item.insert(&conn);
    }
}

pub fn get_and_insert_items(conn: &PgConnection, key: &str, ids: &Vec<i32>) {
    for id in ids {
        match get_item(id, key) {
            Ok(item) => item.insert(&conn),
            Err(err) => println!("{}", err)
        }
    }
}

pub fn get_missing_ids(conn: &PgConnection, ids: &Vec<i32>) -> Vec<i32> {
    let mut missing_ids = Vec::new();
    let mut db_ids = get_item_ids_from_db(conn);

    for id in ids {
        if let None = db_ids.iter().find(|dbi| &id == dbi) {
            missing_ids.push(id.to_owned())
        }
    }
    missing_ids
}

use models::auction::Auction;
pub fn get_item_ids_from_auctions(auctions: &Vec<Auction>) -> Vec<i32> {
    let mut ids: Vec<i32> = auctions.iter().map(|a| a.item).collect();
    ids.sort();
    ids.dedup();
    ids
}

pub fn get_item_ids_from_db(conn: &PgConnection) -> Vec<i32> {
    use schema::item::dsl::*;

    item.select(id)
        .load(conn)
        .expect("error loading ids")
}

pub fn get_average_price(id: i32, conn: &PgConnection) -> f64 {
    use schema::auction::dsl::*;
    let qb: Vec<(i16, Option<i64>)> = auction
        .filter(item.eq(id))
        .select((quantity, buyout))
        .load(conn)
        .expect("error loading item");
    let mut prices = Vec::new();
    for (q, b) in qb  {
        match b {
            Some(b) => {
                let price = b / q as i64;
                println!("{}", price);
                prices.push(price); 
            },
            None => {}
        }
    }
    prices.iter().sum::<i64>() as f64 / prices.len() as f64
}

pub fn get_daily_average_price(id: i32) -> u32 {
    1
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

use std::thread;
use num_cpus;

pub fn get_items_threaded(key: &str, ids: &Vec<i32>) -> Vec<Item> {
    let mut threads = Vec::new();
    let mut items = Vec::new();
    let chunk_len = ids.len() / num_cpus::get();
    println!("{}", chunk_len);
    let chunks = ids.chunks(chunk_len);
    for chunk in chunks.clone() {
        let k = key.to_owned();
        let c = chunk.to_owned();
        threads.push(thread::spawn(move || {
            get_items(&k, &c)
        }));
    }
    for t in threads {
        match t.join() {
            Ok(it) => items.extend(it),
            Err(_) => {}
        }
    }
    items
}

pub fn get_items(key: &str, ids: &Vec<i32>) -> Vec<Item> {
    let mut items = Vec::new();
    for id in ids {
        match get_item(id, key) {
            Ok(item) => items.push(item),
            Err(err) => println!("{}", err)
        }
    }
    items

}

pub fn get_item(id: &i32, key: &str) -> Result<Item, String> {
    let base = "https://eu.api.battle.net/wow/item/";
    let url = format!("{}{}?locale=en_GB&apikey={}", base, id, key);
    let data = match reqwest::get(&url) {
        Ok(mut req) => { 
            match req.text() {
                Ok(text) => text.to_owned(),
                Err(err) => return Err(err.to_string())
            }
        },
        Err(err) => return Err(err.to_string())
    };
    match serde_json::from_str(&data) {
        Ok(item) => Ok(item),
        Err(err) => Err(err.to_string())
    }
}
