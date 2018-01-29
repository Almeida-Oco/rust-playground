use rusqlite::Connection;
use std::collections::HashMap;

pub struct Database {
    file_name: String,
    conn: Connection,
}

#[derive(Debug)]
pub struct UserCoin {
    name: String,
    amount: f64,
    buy_price_usd: f64,
	buy_price_btc: f64,
}

//Public methods
impl Database {
    pub fn new(file_name: &str) -> Database {
        let file_name = String::from("db/") + file_name;
        Database {
            conn: Connection::open(&file_name).expect("Failed to open connection!"),
            file_name,
        }
    }

    pub fn get_user_info(&self, name: &str) -> Option<(String, f64)> {
        let statement = format!("SELECT * FROM User WHERE User.name == \"{}\"", name);

        let mut stmt = self.conn
            .prepare(&statement)
            .expect(&format!("Failed to prepare statement: '{}'", statement));
        let mut person_it = stmt.query_map(&[], |row| (row.get(0), row.get(1)))
            .expect(&format!("Failed to query statement: '{}'", statement));

        match person_it.next() {
            Some(ret) => ret.ok(),
            None => None,
        }
    }

    pub fn get_user_coins(&self, name: &str) -> Option<HashMap<String, UserCoin>> {
        let statement = format!(
            "SELECT name,amount,buy_price_btc,buy_price_usd FROM Coin WHERE Coin.owner == \"{}\"",
            name
        );

        if self.user_exists(name) {
            let mut stmt = self.conn
                .prepare(&statement)
                .expect(&format!("Failed to prepare statemet: '{}'", statement));
            let person_it = stmt.query_map(&[], |row| UserCoin {
                name: row.get(0),
                amount: row.get(1),
				buy_price_btc: row.get(2),
                buy_price_usd: row.get(3),
            }).expect(&format!("Failed to query statement: '{}'", statement));

            let ret: HashMap<String, UserCoin> = person_it
                .filter_map(|elem| elem.map(|coin| (coin.get_name().to_string(), coin)).ok())
                .collect();
            Some(ret)
        } else {
            eprintln!("No user '{}' found in database!", name);
            None
        }
    }

    pub fn insert_user(&self, name: &str, start_amount: f64) -> bool {
        let statement = format!("INSERT INTO User VALUES (\"{}\", {})", name, start_amount);
        if !self.user_exists(name) {
            self.conn
                .execute(&statement, &[])
                .expect(&format!("Failed to execute statement: '{}'", statement));
            true
        } else {
            eprintln!("User '{}' already exists!", name);
            false
        }
    }

    pub fn insert_coin(&self, user_name: &str, coin: UserCoin) -> bool {
        let statement = format!("INSERT INTO Coin VALUES {}", coin.to_str());
        if self.user_exists(user_name) {
            self.conn
                .execute(&statement, &[])
                .expect(&format!("Failed to execute statement: '{}'", statement));
            true
        } else {
            eprintln!("User '{}' does not exist!", user_name);
            false
        }
    }

    pub fn user_exists(&self, name: &str) -> bool {
        let statement = format!("SELECT name FROM User WHERE User.name == \"{}\"", name);

        let mut stmt = self.conn
            .prepare(&statement)
            .expect(&format!("Failed to prepare statement: '{}'", statement));
        let mut person_it = stmt.query(&[])
            .expect(&format!("Failed to query statement: '{}'", statement));

        person_it.next().is_some()
    }
}

impl UserCoin {
    fn to_str(&self) -> String {
        format!(
            "(\"{}\", {}, {}, {})",
            self.name, self.amount, self.buy_price_usd, self.buy_price_btc
        )
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_buy_price_usd(&self) -> f64 {
        self.buy_price_usd
    }

    pub fn get_buy_price_btc(&self) -> f64 {
        self.buy_price_btc
    }
}
