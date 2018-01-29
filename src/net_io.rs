use std::io;
use std::net::ToSocketAddrs;
use futures::Future;
use native_tls::TlsConnector;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_tls::TlsConnectorExt;
use tokio_io;
use serde_json;

const COINMARKETCAP: &str = "api.coinmarketcap.com";
const HTTPS_PORT: &str = ":443";
const PATH_START: &str = "/v1/ticker/";

#[derive(Deserialize, Debug)]
struct CoinJsonHolder {
    id: String,
    name: String,
    symbol: String,
    rank: String,
    price_usd: String,
    price_btc: String,
    market_cap_usd: String,
    percent_change_1h: String,
    percent_change_24h: String,
    percent_change_7d: String,
}

#[derive(Deserialize, Debug)]
struct NameHolder {
    id: String,
}

impl NameHolder {
    pub fn get_name(self) -> String {
        self.id
    }
}

#[derive(Debug)]
pub struct CoinInfo {
    name: String,
    symbol: String,
    rank: u32,
    price_usd: f32,
    price_btc: f32,
    market_cap: f64,
    change_1h: f32,
    change_24h: f32,
    change_7d: f32,
}

impl CoinInfo {
    fn from_coin_holder(holder: &CoinJsonHolder) -> Result<CoinInfo, String> {
        let rank = holder.rank.parse::<u32>();
        let price_usd = holder.price_usd.parse::<f32>();
        let price_btc = holder.price_btc.parse::<f32>();
        let market_cap = holder.market_cap_usd.parse::<f64>();
        let change_1h = holder.percent_change_1h.parse::<f32>();
        let change_24h = holder.percent_change_24h.parse::<f32>();
        let change_7d = holder.percent_change_7d.parse::<f32>();

        if let Err(err) = rank {
            Err(format!("Error parsing 'rank': {:?}", err))
        } else if let Err(err) = price_usd {
            Err(format!("Error parsing 'price_usd': {:?}", err))
        } else if let Err(err) = price_btc {
            Err(format!("Error parsing 'price_btc': {:?}", err))
        } else if let Err(err) = market_cap {
            Err(format!("Error parsing 'market_cap': {:?}", err))
        } else if let Err(err) = change_1h {
            Err(format!("Error parsing 'change_1h': {:?}", err))
        } else if let Err(err) = change_24h {
            Err(format!("Error parsing 'change_24h': {:?}", err))
        } else if let Err(err) = change_7d {
            Err(format!("Error parsing 'change_7d': {:?}", err))
        } else {
            Ok(CoinInfo {
                name: holder.name.clone(),
                symbol: holder.symbol.clone(),
                rank: rank.unwrap(),
                price_usd: price_usd.unwrap(),
                price_btc: price_btc.unwrap(),
                market_cap: market_cap.unwrap(),
                change_1h: change_1h.unwrap(),
                change_24h: change_24h.unwrap(),
                change_7d: change_7d.unwrap(),
            })
        }
    }

	pub fn get_name(&self) -> &String {
		&self.name
	}

    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }

    pub fn get_rank(&self) -> u32 {
        self.rank
    }

    pub fn get_price_usd(&self) -> f32 {
        self.price_usd
    }

    pub fn get_price_btc(&self) -> f32 {
        self.price_btc
    }

    pub fn get_market_cap(&self) -> f64 {
        self.market_cap
    }

    pub fn get_change_1h(&self) -> f32 {
        self.change_1h
    }

    pub fn get_change_24h(&self) -> f32 {
        self.change_24h
    }

    pub fn get_change_7d(&self) -> f32 {
        self.change_7d
    }
}

pub fn get_coin_info(coin: &str) -> Result<CoinInfo, String> {
    let path = String::from(PATH_START) + coin + "/";
    let response = do_request(COINMARKETCAP, HTTPS_PORT, &path);
    match response.find("\r\n\r\n[") {
        Some(index) => {
            let json_data = response.get((index + 4)..).unwrap();
            match serde_json::from_str::<Vec<CoinJsonHolder>>(json_data) {
                Ok(result) => CoinInfo::from_coin_holder(result.iter().next().unwrap()),
                Err(error) => Err(format!("Error parsing response: {}", error)),
            }
        }
        None => Err(format!("{}", response)),
    }
}

pub fn get_all_names() -> Result<Vec<String>, String> {
    let path = String::from(PATH_START) + "?start=0&limit=0";
    let response = do_request(COINMARKETCAP, HTTPS_PORT, &path);
    match response.find("\r\n\r\n[") {
        Some(index) => {
            let json_data = response.get((index + 4)..).unwrap();
            match serde_json::from_str::<Vec<NameHolder>>(json_data) {
                Ok(vec) => {
                    let mut ret: Vec<String> = Vec::new();
                    vec.into_iter()
                        .for_each(|name_holder| ret.push(name_holder.get_name()));
                    Ok(ret)
                }
                Err(error) => Err(format!("Error parsing response: {}", error)),
            }
        }
        None => Err(format!("{}", response)),
    }
}

fn do_request(address: &str, port: &str, path: &str) -> String {
    let mut core = Core::new().expect("Error creating new Core");
    let handle = core.handle();
    let real_address = String::from(address) + port;
    let addr = real_address
        .to_socket_addrs()
        .expect(format!("Error resolving address to IP:Port ('{}')", real_address).as_str())
        .next()
        .expect("No next entry found!");

    let cx = TlsConnector::builder().unwrap().build().unwrap();
    let socket = TcpStream::connect(&addr, &handle);

    let tls_handshake = socket.and_then(|socket| {
        let tls = cx.connect_async(address, socket);
        tls.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    });

    let msg = format!(
        "\
         GET {} HTTP/1.0\r\n\
         Host: {}\r\n\
         \r\n\
         ",
        path, address
    );
    let request =
        tls_handshake.and_then(|socket| tokio_io::io::write_all(socket, msg.as_str().as_bytes()));

    let response = request.and_then(|(socket, _)| tokio_io::io::read_to_end(socket, Vec::new()));

    let (_, data) = core.run(response).unwrap();
    String::from_utf8_lossy(&data).into_owned() //TODO is this cloning?
}
