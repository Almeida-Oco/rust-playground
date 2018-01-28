pragma foreign_keys=ON;

DROP TABLE IF EXISTS User;
DROP TABLE IF EXISTS Coin;

CREATE TABLE User (
	name TEXT PRIMARY KEY,
	start_amount NUMBER
);

CREATE TABLE Coin (
	name TEXT PRIMARY KEY,
	amount NUMBER,
	buy_price_usd NUMBER,
	buy_price_btc NUMBER,
	owner TEXT,
	FOREIGN KEY (owner) REFERENCES User (name)
);


.save database.db
