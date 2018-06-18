extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate postgres;
extern crate chrono;

extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

mod util;
mod config;
mod db;
mod item;
mod auction;

use std::thread;
use std::time::Duration;
use config::Config;
use postgres::Connection;

fn main() {
    let c = config::load_config();
    let conn = db::init(&c);
    save_item_data(&c, &conn);

    loop {
        save_auction_data(&c, &conn);
        thread::sleep(Duration::from_secs(3600));
    }
}


fn save_item_data(c: &Config, conn: &Connection) {
    let ids = item::get_item_ids();
    let items = item::get_items(&ids, &c.bnet_key);
    item::insert_items(&conn, items);
}

fn save_auction_data(c: &Config, conn: &Connection) {
    let ids = item::get_item_ids();
    let (url, time) = auction::get_auction_data_url(&c.bnet_key, &c.realm);

    let auctions = auction::get_auction_data(&url, time);
    let mut all_auctions = Vec::new();

    for id in ids {
        let auctions_of_item = auction::get_auctions_of_item(id, &auctions);
        all_auctions.extend(auctions_of_item); 
    }
    auction::insert_auctions(conn, &all_auctions);

}


