const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

use super::Coin;
use std::fmt;
use std::ops;

pub struct CoinsPrinter {
    coins: Vec<Coin>,
    name_len: usize,
    price_len: usize,
    perc_len: usize,
}

impl CoinsPrinter {
    pub fn new(coins: Vec<Coin>) -> CoinsPrinter {
        let mut printer = CoinsPrinter {
            coins,
            name_len: 0,
            price_len: 0,
            perc_len: 0,
        };
        printer.max_len_params();
        printer
    }

    fn max_len_params(&mut self) {
        for coin in self.coins.iter() {
            let name_len = coin.get_name().len();
            let price_len = number_digits(coin.get_price_usd());
            let perc_len = number_digits(coin.get_price_diff());
            if self.name_len < name_len {
                self.name_len = name_len;
            }
            if self.price_len < price_len {
                self.price_len = price_len;
            }
            if self.perc_len < perc_len {
                self.perc_len = perc_len;
            }
        }
		self.price_len += 3;
		self.perc_len += 3;
	}

	pub fn print(&self) {
		for coin in self.coins.iter() {
			let price_diff = coin.get_price_diff();
			let color;
			if price_diff > 0.0 {
				color = GREEN;
			}
			else if price_diff < 0.0 {
				color = RED;
			}
			else {
				color = YELLOW;
			}

			println!("{name:^namelen$}({symbol}) | {price:^pricelen$.2} | {color}{perc:>perclen$.1}%{reset}",
				name = coin.get_name(),
				namelen = self.name_len,
				symbol = coin.get_symbol(),
				price = coin.get_price_usd(),
				pricelen = self.price_len,
				color = color,
				perc = coin.get_price_diff(),
				perclen = self.perc_len,
				reset = RESET);
		}
	}
}

fn number_digits(number: f64) -> usize {
    let mut digits: usize = 0;
    let mut n = number;
    while n >= 1.0 {
        digits += 1;
        n = n / 10.0;
    }
    digits
}
