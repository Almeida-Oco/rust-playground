extern crate futures;
extern crate native_tls;
extern crate rustc_serialize;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;

mod net_io;

fn main() {
    net_io::get_coin_info("bitcoin");
}
