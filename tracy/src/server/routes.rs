use warp::Filter;

use crate::handler::{
    get_pool_by_address_handler, get_pools_handler, get_quotes, list_pools_for_denom,
    list_pools_for_denoms, with_db, Db,
};

fn pools_with_denom(
    dex_agg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("pools_for_denom")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(dex_agg))
        .and_then(list_pools_for_denom)
}

fn pools_with_denoms(
    dex_agg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("pools_for_denoms")
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::param())
        .and(with_db(dex_agg))
        .and_then(list_pools_for_denoms)
}

fn get_quotes_route(
    dex_agg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("quote" / String / String / String)
        .and(warp::get())
        .and(with_db(dex_agg))
        .and_then(get_quotes)
}

fn get_pool_by_address(
    dex_agg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("pool" / String)
        .and(warp::get())
        .and(with_db(dex_agg))
        .and_then(get_pool_by_address_handler)
}

fn get_pools(
    dex_agg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("pools")
        .and(warp::get())
        .and(with_db(dex_agg))
        .and_then(get_pools_handler)
}

pub fn all_routes(
    dex_agg: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    pools_with_denom(dex_agg.clone())
        .or(pools_with_denoms(dex_agg.clone()))
        .or(get_quotes_route(dex_agg.clone()))
        .or(get_pool_by_address(dex_agg.clone()))
        .or(get_pools(dex_agg.clone()))
}
