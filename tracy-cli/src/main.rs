use tracy::juno_pool::{JunoPoolConfig, WasmPool};
use clap::{Arg, ArgAction, Command};

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
            .short_flag('q')
            .long_flag("quote")
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
            .short_flag('l')
            .long_flag("load")
            .about("Loading files.")
            .arg(
                Arg::new("chain")
                    .short('s')
                    .long("search")
                    .action(ArgAction::Set)
                    .num_args(1..)
                    .help("search remote repositories for matching strings"),
            )
            .arg(
                Arg::new("node")
                    .short('n')
                    .long("node")
                    .action(ArgAction::Set)
                    .num_args(1..)
                    .help("search remote repositories for matching strings"),
            )
    )
    .get_matches();

    match matches.subcommand() {
        Some(("quote", query_matches)) => {
            let token_in = query_matches.get_one::<String>("token_in");
            let token_out = query_matches.get_one::<String>("token_out");
            let chain = query_matches.get_one::<String>("chain");
            let node = query_matches.get_one::<String>("node");
            let amount = query_matches.get_one::<u128>("amount");
            if token_in.is_none() {
                println!("Provide token_in argument!");
                return Ok(());
            }
            if token_out.is_none() {
                println!("Provide token_out argument!");
                return Ok(());
            }
            if chain.is_none() {
                println!("Provide chain argument!");
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
            let token_in = token_in.unwrap();
            let token_out = token_out.unwrap();
            let chain = chain.unwrap();
            let node = node.unwrap();
            let amount = amount.unwrap();
            match chain {
                "juno" => {
                    let pool = WasmPool::new();
                    let config = JunoPoolConfig {
                        path: "./assets/juno_pools.json".to_string(),
                        api: node.to_string(),
                    };
                    let quote = pool.get_quote(*amount, token_in, token_out, config).await?;
                    println!(
                        "Price for {} {} -> {} {}",
                        token_in, amount, token_out, quote.token_out
                    );
                },
                "osmosis" => {
                    todo!()
                }
            }

            println!("Token in {:?}...", token_in);
            println!("Token out {:?}...", token_out);
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
            println!("Chain {:?}...", chain);
            println!("Node {:?}...", node);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    };
    // let api = "https://lcd-juno.itastakers.com";
    // // let _res = fetch_juno_pools(api).await.unwrap();
    // let pool = JunoPool::new();
    // let config = JunoPoolConfig {
    //     path: "juno_pools.json".to_string(),
    //     api: api.to_string(),
    // };
    // let token_in = "ujuno";
    // let token_out = "uatom";
    // let amount = 1000000;
    // let quote = pool.get_quote(amount, token_in, token_out, config).await?;
    // println!(
    //     "Price for {} {} -> {} {}",
    //     token_in, amount, token_out, quote.token_out
    // );
    Ok(())
}
