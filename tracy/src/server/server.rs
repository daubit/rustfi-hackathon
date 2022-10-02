use std::net::{Ipv4Addr, SocketAddrV4};

use handler::Db;
use tracy::dex::DexAgg;

use crate::routes::all_routes;

mod handler;
mod routes;

#[tokio::main]
async fn main() {
    println!("server");

    // TODO: do we need arc?
    let dexes: Db = DexAgg::new(None).unwrap();
    let api = all_routes(dexes);
    warp::serve(api)
        .run(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
        .await;
}
