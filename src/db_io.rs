use rusqlite::Connection;

struct Database {
    file_name: &str,
    conn: Option<Connection>,
}

struct Coin {
    name: String,
    amount: f64,
    buy_price_usb: f64,
    buy_price_btc: f64,
}

impl Database {
    pub fn new(file_name: &str) -> Database {
        Database {
            file_name,
            conn: None,
        }
    }

    fn check_conn(&mut self) -> &Connection {
        if let Some(connection) = self.conn {
            connection
        } else {
            self.conn = Connection::open_in_memory(self.file_name).ok();
            self.conn
                .expect("Error opening database: '{}'", self.file_name)
        }
    }

    fn get_user_info(&mut self, name: &str) -> Option<(String, f32)> {
        let conn = self.check_conn();
        let statement = format!("SELECT * FROM User WHERE User.name == \"{}\"", name);

        let stmt = conn.prepare(&statement)
            .expect("Failed to prepare statement: '{}'", statement);
        let person_it = stmt.query_map(&[], |row| (row.get(0), row.get(1)))
            .expect("Failed to query statement: '{}'", statement);

        person_it.next()
    }

    fn get_user_coins(&mut self, name: &str) -> Option<Vec<Coin>> {
        let conn = self.check_conn();
        let statement = format!(
            "SELECT name,amount,buy_price_btc,buy_price_usd FROM Coin WHERE Coin.owner == \"{}\"",
            name
        );

        if self.user_exists() {
            let stmt = conn.prepare(&statement)
                .expect("Failed to prepare statemet: '{}'", statement);
            let person_it = stmt.query_map(&[], |row| Coin {
                name: row.get(0),
                amount: row.get(1),
                buy_price_usd: row.get(2),
                buy_price_btc: row.get(3),
            }).expect("Failed to query statement: '{}'", statement);

            Some(person_it.collect::<Vec<Coin>>())
        } else {
            eprintln!("No user '{}' found in database!", name);
            None
        }
    }

    fn user_exists(&mut self, name: &str) -> bool {
        let conn = self.check_conn();
        let statement = format!("SELECT name FROM User WHERE User.name == \"{}\"", name);

        let stmt = conn.prepare(&statement)
            .expect("Failed to prepare statement: '{}'", statement);
        let person_it = stmt.query_map(&[], |row| row.get(0))
            .expect("Failed to query statement: '{}'", statement);

        person_it.next().is_some()
    }
}
