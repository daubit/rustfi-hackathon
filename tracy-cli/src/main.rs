use clap::{Arg, ArgAction, Command};
use tracy::dex::DexAgg;
use tracy::juno_pool::fetch_juno_pools;
use tracy::pools::fetch_osmosis_pools;
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
            let dex = DexAgg::new()?;
            let token_in = token_in.unwrap();
            let token_out = token_out.unwrap();
            let node = node.unwrap();
            let amount = amount.unwrap().parse::<u128>()?;
            let pools = dex.with_denoms(vec![token_in.to_string(), token_out.to_string()]);
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
                    let res = fetch_osmosis_pools().await;
                    if res.is_err() {
                        println!("Something went wrong while fetching the data for Osmosis")
                    } else {
                        println!("Successfully fetched the data for Osmosis");
                    }
                }
                _ => println!("Chain not yet implemented!"),
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    };
    Ok(())
}
