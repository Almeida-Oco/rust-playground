extern crate futures;
extern crate native_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;

mod net_io;

fn main() {
    let btc = net_io::get_coin_info("bitcoin");
    println!("{:?}", btc);
    let all_coins = net_io::get_all_names();
    println!("Names = \n{:?}", all_coins);
}
