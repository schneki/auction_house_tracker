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
    //let q = item::get_average_price(124101, &conn);
    //println!("{}", q);
    //
    let (url, time) = auction::get_auction_data_url(&c.bnet_key, &c.realm);
    let auctions = auction::get_auction_data(&url, time);

    let ids = item::get_item_ids_from_auctions(&auctions);
    let missing_ids = item::get_missing_ids(&conn, &ids);
    println!("{}", missing_ids.len());
    
    let items = 
        if missing_ids.len() >= num_cpus::get() {
            item::get_items_threaded(&c.bnet_key, &missing_ids)
        } else {
            item::get_items(&c.bnet_key, &missing_ids)
        };

    item::insert_items(&conn, &items);
    auction::insert_auctions(&conn, &auctions);

    
    /*
    let conn = db::init(&c);
    save_item_data(&c, &conn);

    loop {
        save_auction_data(&c, &conn);
        thread::sleep(Duration::from_secs(3600));
    }
    */
}
