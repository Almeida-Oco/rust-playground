use std::io;
use std::net::ToSocketAddrs;
use futures::Future;
use native_tls::TlsConnector;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_tls::TlsConnectorExt;
use tokio_io;
use rustc_serialize::json;

const COINMARKETCAP: &str = "api.coinmarketcap.com";
const HTTPS_PORT: &str = ":443";
const PATH_START: &str = "/v1/ticker/";

pub fn get_coin_info(coin: &str) -> String {
    let path = String::from(PATH_START) + coin + "/";
    let response = do_request(COINMARKETCAP, HTTPS_PORT, &path);

    println!("Response {}", response.get(0..20).unwrap());

    let json_data = response
        .get((response.find("\r\n\r\n[").unwrap() + 4)..)
        .unwrap();

    json::from_str(json_data);
    let mut it = json::Parser::new(json_data.chars());
    let builder = json::Builder::new(it);
    println!("Built!");
    if let Ok(result) = builder.build() {
        println!("{}", result);
    }

    String::new()
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
