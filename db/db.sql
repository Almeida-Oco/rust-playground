pragma foreign_keys=ON;

DROP TABLE IF EXISTS User;
DROP TABLE IF EXISTS Coin;

CREATE TABLE User (
	name TEXT PRIMARY KEY,
	start_amount REAL
);

CREATE TABLE Coin (
	name TEXT PRIMARY KEY,
	amount REAL,
	buy_price_usd REAL,
	buy_price_btc REAL,
	owner TEXT,
	FOREIGN KEY (owner) REFERENCES User (name)
);


.save database.db
