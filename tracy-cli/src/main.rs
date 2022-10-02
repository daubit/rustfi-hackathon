use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::{Arg, ArgAction, Command};
use petgraph::dot::Dot;
use petgraph::stable_graph::{DefaultIx, NodeIndex};
use petgraph::{Graph, Undirected};
use tracy::dex::DexAgg;
use tracy::pools::juno_pool::{fetch_juno_pools, load_juno_pools_from_file};
use tracy::pools::osmosis_pool::{fetch_osmosis_pools, load_osmo_pools_from_file_boxed};
use tracy::PoolConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("tracy-cli")
        .about("tracy cli")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Daubit")
        .subcommand(
            Command::new("quote")
                .about("Get the quote of two tokens")
                .arg(
                    Arg::new("token_in")
                        .long("token_in")
                        .help("Token in")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("token_out")
                        .long("token_out")
                        .help("Token out")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("amount")
                        .long("amount")
                        .help("Token in amount")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("chain")
                        .long("chain")
                        .help("Chain to query")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("node")
                        .long("node")
                        .help("Node to query from")
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("load")
                .about("Loading files.")
                .arg(
                    Arg::new("chain")
                        .short('s')
                        .long("chain")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("Chain to update"),
                )
                .arg(
                    Arg::new("node")
                        .short('n')
                        .long("node")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("Node to connect to"),
                ),
        )
        .subcommand(Command::new("graph").about("generator dotfile"))
        .get_matches();

    match matches.subcommand() {
        Some(("quote", query_matches)) => {
            let token_in = query_matches.get_one::<String>("token_in");
            let token_out = query_matches.get_one::<String>("token_out");
            let node = query_matches.get_one::<String>("node");
            let amount = query_matches.get_one::<String>("amount");
            if token_in.is_none() {
                println!("Provide token_in argument!");
                return Ok(());
            }
            if token_out.is_none() {
                println!("Provide token_out argument!");
                return Ok(());
            }
            if node.is_none() {
                println!("Provide node argument!");
                return Ok(());
            }
            if amount.is_none() {
                println!("Provide amount argument!");
                return Ok(());
            }
            let dex = DexAgg::new(None)?;
            let token_in = token_in.unwrap();
            let token_out = token_out.unwrap();
            let node = node.unwrap();
            let amount = amount.unwrap().parse::<u128>()?;
            let pools = dex
                .with_denoms(vec![token_in.to_string(), token_out.to_string()])
                .await;
            let config = PoolConfig {
                rest_url: Some(node.to_string()),
                grpc_url: None,
                rpc_url: None,
                estimate_quote: false,
            };
            for pool in pools {
                let quote = pool.get_quote(amount, token_in, token_out, &config).await;
                if quote.is_ok() {
                    println!(
                        "Chain: {}\nPool Address: {}\nPrice for {} {} -> {} {}\n\n",
                        pool.chain(),
                        pool.address()?,
                        token_in,
                        amount,
                        token_out,
                        quote?.token_out.unwrap(),
                    );
                }
            }
        }
        Some(("load", query_matches)) => {
            let chain = query_matches.get_one::<String>("chain");
            let node = query_matches.get_one::<String>("node");
            if chain.is_none() {
                println!("Provide a chain!");
                return Ok(());
            }
            if node.is_none() {
                println!("Provide a node!");
                return Ok(());
            }
            let chain = chain.unwrap();
            let node = node.unwrap();
            println!("Loading...");
            match chain.to_string().as_str() {
                "juno" => {
                    let res = fetch_juno_pools(node).await;
                    if res.is_err() {
                        println!("Something went wrong while fetching the data for Juno")
                    } else {
                        println!("Successfully fetched the data for Juno");
                    }
                }
                "osmosis" => {
                    let res = fetch_osmosis_pools("https://lcd.osmosis.zone").await;
                    if res.is_err() {
                        println!("Something went wrong while fetching the data for Osmosis")
                    } else {
                        println!("Successfully fetched the data for Osmosis");
                    }
                }
                _ => println!("Chain not yet implemented!"),
            }
        }
        Some(("graph", _)) => {
            // TODO: use DexAgg
            let osmo_pools =
                load_osmo_pools_from_file_boxed(&Path::new("./osmosis_pools_hackathon.json"))?;
            let juno_pools = load_juno_pools_from_file(&Path::new("./juno_pools.json"))?;

            let mut graph = Graph::<String, String, Undirected>::new_undirected();

            let mut token_map: HashMap<String, NodeIndex<DefaultIx>> = HashMap::new();

            for pool in osmo_pools.clone() {
                for asset in pool.pool_assets.clone() {
                    if !token_map.contains_key(&asset.token.native_name.clone().unwrap()) {
                        let node = graph.add_node(asset.token.native_name.clone().unwrap());
                        token_map.insert(asset.token.native_name.clone().unwrap(), node);
                    }
                }
            }

            for pool in juno_pools.clone() {
                let token_1 = if pool.token1_denom.cw20.is_some() {
                    format!("cw20:{}", pool.token1_denom.cw20.clone().unwrap())
                } else {
                    pool.token1.clone().unwrap().symbol.clone().unwrap()
                };
                let token_2 = if pool.token2_denom.cw20.is_some() {
                    format!("cw20:{}", pool.token2_denom.cw20.clone().unwrap())
                } else {
                    pool.token2.clone().unwrap().symbol.clone().unwrap()
                };

                if !token_map.contains_key(&token_1) {
                    let node = graph.add_node(token_1.clone());
                    token_map.insert(token_1.clone(), node);
                }

                if !token_map.contains_key(&token_2) {
                    let node = graph.add_node(token_2.clone());
                    token_map.insert(token_2.clone(), node);
                }
            }

            for pool in osmo_pools {
                for asset in pool.pool_assets.clone() {
                    let index = pool
                        .pool_assets
                        .iter()
                        .position(|x| {
                            x.token.amount == asset.token.amount
                                && x.token.denom == asset.token.denom
                                && x.token.native_name == asset.token.native_name
                        })
                        .unwrap();
                    if pool.pool_assets.len() < index + 1 {
                        println!("{} {}", pool.pool_assets.len(), index);
                        continue;
                    }
                    for other_asset in &pool.pool_assets.clone()[index + 1..] {
                        let node_1 = token_map
                            .get(&asset.token.native_name.clone().unwrap())
                            .unwrap();
                        let node_2 = token_map
                            .get(&other_asset.token.native_name.clone().unwrap())
                            .unwrap();

                        graph.add_edge(*node_1, *node_2, pool.id.clone());
                    }
                }
            }

            for pool in juno_pools {
                let token_1 = if pool.token1_denom.cw20.is_some() {
                    format!("cw20:{}", pool.token1_denom.cw20.unwrap())
                } else {
                    pool.token1.unwrap().symbol.unwrap()
                };
                let token_2 = if pool.token2_denom.cw20.is_some() {
                    format!("cw20:{}", pool.token2_denom.cw20.unwrap())
                } else {
                    pool.token2.unwrap().symbol.unwrap()
                };

                let node_1 = token_map.get(&token_1).unwrap();
                let node_2 = token_map.get(&token_2).unwrap();

                graph.add_edge(*node_1, *node_2, pool.pool_address.unwrap());
            }

            let dot_config = &vec![];
            let dot = Dot::with_config(&graph, dot_config);

            let mut f = File::create(Path::new("graph.dot"))?;
            f.write_all(dot.to_string().as_bytes())?;
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    };
    Ok(())
}
