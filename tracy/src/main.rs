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
