#![recursion_limit="128"]
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate reqwest;

#[macro_use] extern crate diesel;
extern crate dotenv;

extern crate num_cpus;

pub mod schema;
pub mod models;

mod config;
mod db;
mod item;
mod auction;

use std::thread;
use std::time::Duration;
use config::Config;
use diesel::PgConnection;

fn main() {
    let c = config::load_config();
    let conn = db::establish_connection();
    loop {
        update_auctions(&c, &conn);
        thread::sleep(Duration::from_secs(3600));
    }
}


fn update_auctions(c: &Config, conn: &PgConnection) {
    let (url, time) = auction::get_auction_data_url(&c.bnet_key, &c.realm);
    let auctions = auction::get_auction_data(&url, time);

    let missing_auctions = auction::get_missing_auctions(&conn, &auctions);
    let ids = item::get_item_ids_from_auctions(&missing_auctions);
    let missing_ids = item::get_missing_ids(&conn, &ids);

    let items = item::get_items_threaded(&c.bnet_key, &missing_ids);
    item::insert_items(&conn, &items);
    auction::insert_auctions(&conn, &missing_auctions);
    println!("Auctions updated");
}
