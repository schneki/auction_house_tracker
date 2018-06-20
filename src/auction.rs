use reqwest;
use item;
use serde_json;
use std::fs::File;
use std::io::Read;

use chrono::NaiveDateTime;
use models::auction::{NewAuction, Auction};
use diesel;
use diesel::RunQueryDsl;
use diesel::PgConnection;

impl Auction {
    pub fn insert(&self, conn: &PgConnection) {
        use schema::auction;

        let new_auction = NewAuction::from(self);

        match diesel::insert_into(auction::table)
            .values(&new_auction)
            .execute(conn)
        {
            Ok(_) => {},
            Err(_) => {}
        }
    }
}

pub fn insert_auctions(conn: &PgConnection, auctions: &Vec<Auction>) {
    for a in auctions {
        a.insert(&conn)
    }
}


pub fn get_auctions_of_item(id: i32, auctions: &Vec<Auction>) -> Vec<Auction> {
    let mut new_auctions = Vec::new();
    for auction in auctions {
        if auction.item == id {
            new_auctions.push(auction.clone())
        }
    }
    new_auctions
}

pub fn get_auction_data(url: &str, time: i64) -> Vec<Auction> {
    let data = reqwest::get(url).unwrap().text().unwrap();
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let mut auctions: Vec<Auction> = serde_json::from_str(&v["auctions"].to_string()).unwrap();
    for a in auctions.iter_mut() {
        a.time = time
    }
    auctions
}

pub fn get_auction_data_from_file() -> Vec<Auction> {
    let mut f = File::open("test.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).unwrap();
    serde_json::from_str(&v["auctions"].to_string()).unwrap()
}

//TODO: make less error prone
pub fn get_auction_data_url(key: &str, realm: &str) -> (String, i64) {
    let base = "https://eu.api.battle.net/wow/auction/data/";
    let url = format!("{}{}?locale=en_GB&apikey={}", base, realm, key);
    let data = reqwest::get(&url).unwrap().text().unwrap();

    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    (
        v["files"][0]["url"].as_str().unwrap().into(),
        v["files"][0]["lastModified"].as_i64().unwrap()
    )
}
