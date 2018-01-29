const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

use super::Coin;
use std::fmt;

impl Coin {
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
        write!(f, "{:^8.3} -> ", self.buy_price_usd);
        write!(f, "{:.3} | ", self.price_usd);
        write!(f, "{} | ", self.calc_perc())
    }
}
