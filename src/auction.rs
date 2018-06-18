use item;
use serde_json;
use std::fs::File;
use std::io::Read;

use postgres::Connection;
use chrono::NaiveDateTime;
use util;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auction {
    auc: i32,
    item: i32,
    owner: String,
    #[serde(rename = "ownerRealm")]
    owner_realm: String,
    bid: i64,
    buyout: i64,
    quantity: i16,
    #[serde(rename = "timeLeft")]
    time_left: String,
    #[serde(skip)]
    time: u64
}

impl Auction {
    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }
    pub fn insert(&self, conn: &Connection) {
        let native_time = NaiveDateTime::from_timestamp(self.time as i64, 0);
        match conn.execute("
            INSERT INTO auction 
                (auc, item, owner, ownerRealm, bid, buyout, quantity, timeLeft, time)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (auc)
                DO NOTHING",
                &[
                    &self.auc, &self.item, &self.owner, &self.owner_realm, 
                    &self.bid, &self.buyout, &self.quantity, &self.time_left, &native_time
                ] 
        )
        {
            Ok(res) => {},
            Err(err) => println!("{}", err)
        }
    }
}

pub fn insert_auctions(conn: &Connection, auctions: &Vec<Auction>) {
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

pub fn get_auction_data(url: &str, time: u64) -> Vec<Auction> {
    let data = util::get_url_content_https(url);
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    let mut auctions: Vec<Auction> = serde_json::from_str(&v["auctions"].to_string()).unwrap();
    for a in auctions.iter_mut() {
        a.set_time(time);
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
pub fn get_auction_data_url(key: &str, realm: &str) -> (String, u64) {
    let base = "https://eu.api.battle.net/wow/auction/data/";
    let url = format!("{}{}?locale=en_GB&apikey={}", base, realm, key);

    let data = util::get_url_content_https(&url);

    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    (
        v["files"][0]["url"].as_str().unwrap().into(),
        v["files"][0]["lastModified"].as_u64().unwrap()
    )
}
