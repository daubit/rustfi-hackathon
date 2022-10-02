# tracy

This crate contains an API for fetching data from various blockchains.
Currently there are API for the Juno and the Osmosis Blockchain.

How to use:

## Example

```rs
use tracy::dex::DexAgg;

#[tokio::main]
async fn main() {
    let dexes = DexAgg::new(None).unwrap();
    let amount = "1000000";
    let denom1 = "ujuno".to_string();
    let denom2 = "uatom".to_string();
    let pools = dexes
        .with_denoms(vec![denom1.clone(), denom2.clone()])
        .await
        .clone();
    let mut quotes = vec![];
    for pool in pools {
        let quote = pool
            .get_quote(
                u128::from_str_radix(&amount, 10).unwrap(),
                &denom1,
                &denom2,
                dexes
                    .config
                    .get(&pool.chain())
                    .expect(&format!("No config for chain {}", pool.chain())),
            )
            .await;
        quotes.push(quote);
    }
    println!("{:?}", quotes);
}

```

How to add a new Blockchain:

- implement the Pool Trait for your new Pool Type
- The pub should have the pool_address and chain fields to work with tracy-web
- when creating DexAgg supply your pools to the function or fork the project add your pool in `DexAgg::new()`

# tracy-cli

cli to make it easier to interact with tracy, refer to --help for more info

Commandos:

### quote

Command fetches the token_out amount for the amount of token_in. <br>
`cargo run quote --token_in < token1 > --token_out < token2 > --amount < amount > --chain < chain > --node < node >`

### load

Fetches data into json file for caching. <br>
`cargo run load --chain < chain > --node < node >`

### graph

Outputs graph.dot into the current directory. Nodes are Tokens and edges are pools. Address for Juno Pools and numbers for Osmosis pools.
`cargo run graph`

# tracy-web

Website so visualize data. The pool tab displays every currently fetched pool. In Swap you can choose two tokens and if there are liquidity pools for the tokens they will be displayed.

Start with:
In tracy/:

- cargo run --bin server

in tracy-web/:

- yarn install
- yarn dev
