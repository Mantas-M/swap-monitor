mod swap_monitor;
use crossterm::{cursor, execute, terminal};
use dotenv::dotenv;
use reqwest::Client;
use std::io::stdout;
use swap_monitor::table::generate_table;
use swap_monitor::token_pair::{process_new_token_pair, TokenPair};
use swap_monitor::ws_api::subscribe_to_logs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client: Client = Client::new();
    let mut token_pairs: Vec<TokenPair> = Vec::new();

    let mut rx = subscribe_to_logs()
        .await
        .expect("Failed to subscribe to logs");

    let mut counter = 0;

    while let Some(message) = rx.recv().await {
        counter += 1;
        println!("counter: {}", counter);

        let pair_address = message.params.result.address;

        match token_pairs
            .iter_mut()
            .find(|pair| pair.address == pair_address)
        {
            Some(pair) => {
                pair.num_updates += 1;
                println!("Found a pair with address {}: {:?}", pair_address, pair);
            }
            None => {
                let token_pair = process_new_token_pair(&client, &pair_address)
                    .await
                    .unwrap_or_else(|e| {
                        panic!("Error - Failed to process new token pair: {}", e);
                    });

                token_pairs.push(token_pair);
            }
        }

        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        token_pairs.sort_by(|a, b| b.num_updates.cmp(&a.num_updates));
        let table = generate_table(&token_pairs);

        println!("{}", table);
    }

    Ok(())
}
