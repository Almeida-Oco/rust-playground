extern crate futures;
extern crate native_tls;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;

use std::env;
use std::cmp;
use std::fmt;

mod net_io;
mod db_io;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[derive(Debug)]
struct Coin {
    name: String,
    symbol: String,
    rank: u32,
    price_usd: f64,
    market_cap: f64,
    change_1h: f32,
    change_24h: f32,
    change_7d: f32,
    amount_bought: f64,
    buy_price_usd: f64,
}

fn args_correct(args: &Vec<String>) -> bool {
    let print_usage = || {
        println!("Usage: ./portfolio <user_name>");
        false
    };
    if args.len() != 2 {
        return print_usage();
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args_correct(&args) {
        return;
    }
    let user = args.get(1).unwrap();
    let db = db_io::Database::new("database.db");

    let user_coins = get_user_coins_info(&db, user).unwrap();
    let mut coins_iter = user_coins.iter();

    while let Some(coin) = coins_iter.next() {
        println!("{}", coin);
    }
}

fn get_user_coins_info(db: &db_io::Database, user: &str) -> Option<Vec<Coin>> {
    let mut ret: Vec<Coin> = Vec::new();
    if let Some(coins) = db.get_user_coins(user) {
        for (coin_name, coin_info) in coins.iter() {
            match net_io::get_coin_info(coin_name) {
                Ok(info) => ret.push(Coin::new(info, coin_info)),
                Err(err_msg) => {
                    eprintln!("{}", err_msg);
                    return None;
                }
            };
        }
        ret.sort();
        Some(ret)
    } else {
        None
    }
}

impl Coin {
    pub fn new(info: net_io::CoinInfo, u_coin: &db_io::UserCoin) -> Coin {
        Coin {
            name: info.get_name().to_string(),
            symbol: info.get_symbol().to_string(),
            rank: info.get_rank(),
            price_usd: info.get_price_usd() as f64,
            market_cap: info.get_market_cap(),
            change_1h: info.get_change_1h(),
            change_24h: info.get_change_24h(),
            change_7d: info.get_change_7d(),
            amount_bought: u_coin.get_amount(),
            buy_price_usd: u_coin.get_buy_price_usd(),
        }
    }

    pub fn calc_perc(&self) -> String {
        if self.price_usd > self.buy_price_usd {
            format!(
                "{}{:.2}%{}",
                GREEN,
                (1.0 - (self.buy_price_usd / self.price_usd)) * 100.0,
                RESET
            )
        } else {
            format!(
                "{}{:.2}%{}",
                RED,
                (1.0 - (self.price_usd / self.buy_price_usd)) * 100.0,
                RESET
            )
        }
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ", self.name);
        write!(f, "({}) | ", self.symbol);
        write!(f, "{} -> ", self.buy_price_usd);
        write!(f, "{:.2} | ", self.price_usd);
        write!(f, "{} | ", self.calc_perc())
    }
}

impl cmp::Ord for Coin {
    fn cmp(&self, other: &Coin) -> cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl cmp::PartialOrd for Coin {
    fn partial_cmp(&self, other: &Coin) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for Coin {
    fn eq(&self, other: &Coin) -> bool {
        self.name == other.name && self.rank == other.rank
    }
}

impl cmp::Eq for Coin {}
