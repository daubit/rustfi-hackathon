use std::{
    convert::Infallible,
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use tokio::sync::Mutex;
use tracy::{dex::DexAgg, PoolConfig, Quote};
use warp::{http::Response, Filter};

pub type Db = Arc<Mutex<DexAgg>>;

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn list_pools_for_denom(param: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;
    let text = db
        .with_denom(&param)
        .clone()
        .into_iter()
        .map(|x| x.to_json())
        .reduce(|acc, x| acc + "," + &x);
    let body = match text {
        Some(text) => format!("[{}]", text),
        None => format!("[]"),
    };

    Ok(Response::builder()
        .header("content-type", "application/json")
        .body(body)
        .unwrap())
}

async fn list_pools_for_denoms(
    denom1: String,
    denom2: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;
    let text = db
        .with_denoms(vec![denom1, denom2])
        .clone()
        .into_iter()
        .map(|x| x.to_json())
        .reduce(|acc, x| acc + "," + &x);
    let body = match text {
        Some(text) => format!("[{}]", text),
        None => format!("[]"),
    };
    Ok(Response::builder()
        .header("content-type", "application/json")
        .body(body)
        .unwrap())
}

async fn get_quotes(
    denom1: String,
    denom2: String,
    amount: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;
    let pools = db
        .with_denoms(vec![denom1.to_string(), denom2.to_string()])
        .clone();
    let mut quotes = vec![];
    for pool in pools {
        let quote = pool
            .get_quote(
                u128::from_str_radix(&amount, 10).unwrap(),
                &denom1,
                &denom2,
                PoolConfig {
                    grpc_url: None,
                    rest_url: None,
                    rpc_url: None,
                    estimate_quote: false,
                },
            )
            .await;
        quotes.push(quote);
    }

    let returnarray: Vec<Quote> = quotes
        .into_iter()
        .map(|x| match x {
            Ok(x) => x,
            Err(x) => Quote {
                error: Some(format!("{{{}}}", x).to_string()),
                token_in: None,
                token_out: None,
                pool_address: None,
            },
        })
        .collect();
    Ok(warp::reply::json(&returnarray))
    /*let body = match text {
        Some(text) => format!("[{}]", returnarray),
        None => format!("[]"),
    };
    Ok(Response::builder()
        .header("content-type", "application/json")
        .body(body)
        .unwrap())*/
}

fn pools_with_denom(
    dexAgg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("pools_for_denom")
        .and(warp::get())
        .and(warp::path::param())
        // TODO: hacky af
        .and(with_db(dexAgg))
        .and_then(list_pools_for_denom)
}

fn pools_with_denoms(
    dexAgg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("pools_for_denoms")
        .and(warp::path::param())
        .and(warp::path::param())
        // TODO: hacky af
        .and(with_db(dexAgg))
        .and_then(list_pools_for_denoms)
}

fn get_quotes_route(
    dexAgg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("quote" / String / String / String)
        // TODO: hacky af
        .and(with_db(dexAgg))
        .and_then(get_quotes)
}

fn init_routes(
    dexAgg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    pools_with_denom(dexAgg.clone())
        .or(pools_with_denoms(dexAgg.clone()))
        .or(get_quotes_route(dexAgg.clone()))
}

#[tokio::main]
async fn main() {
    println!("server");

    // TODO: do we need arc?
    let dexes: Db = Arc::new(Mutex::new(DexAgg::new().unwrap()));
    let api = init_routes(dexes);
    warp::serve(api)
        .run(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
        .await;
}
