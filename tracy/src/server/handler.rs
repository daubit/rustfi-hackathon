use std::convert::Infallible;

use tracy::{dex::DexAgg, Quote};
use warp::{http::Response, Filter};

pub type Db = DexAgg;

pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub async fn list_pools_for_denom(param: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let db = db;
    let text = db
        .with_denom(&param)
        .await
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

pub async fn list_pools_for_denoms(
    denom1: String,
    denom2: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db;
    let text = db
        .with_denoms(vec![denom1, denom2])
        .await
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

pub async fn get_quotes(
    denom1: String,
    denom2: String,
    amount: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db;
    let pools = db
        .with_denoms(vec![denom1.to_string(), denom2.to_string()])
        .await
        .clone();
    let mut quotes = vec![];
    for pool in pools {
        let quote = pool
            .get_quote(
                u128::from_str_radix(&amount, 10).unwrap(),
                &denom1,
                &denom2,
                db.config
                    .get(&pool.chain())
                    .expect(&format!("No config for chain {}", pool.chain())),
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
}

pub async fn get_pool_by_address_handler(
    address: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let db = db;
    let pool = db.with_address(&address).await;
    let body = match pool {
        Ok(p) => p.to_json(),
        Err(e) => format!("{{\"error\": \"{}\"}}", e.to_string()),
    };

    Ok(Response::builder()
        .header("content-type", "application/json")
        .body(body)
        .unwrap())
}

pub async fn get_pools_handler(db: Db) -> Result<impl warp::Reply, Infallible> {
    let db = db;
    let objs = db
        .pools
        .lock()
        .await
        .iter()
        .map(|x| x.to_json())
        .reduce(|acc: String, x: String| format!("{},{}", acc, x));

    Ok(Response::builder()
        .header("content-type", "application/json")
        .body(format!("[{}]", objs.unwrap_or("".to_owned())))
        .unwrap())
}
