mod evm;
use dotenv::dotenv;
use evm::rpc_api::{get_block_number, get_pair_token_addresses, get_token_balances};
use evm::token_pair::{process_new_token_pair, TokenPair};
use evm::ws_api::subscribe_to_logs;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client: Client = Client::new();
    let mut token_pairs: Vec<TokenPair> = Vec::new();

    let mut rx = subscribe_to_logs()
        .await
        .expect("Failed to subscribe to logs");

    while let Some(message) = rx.recv().await {
        println!("Received message: {:?}", message.params.result.address);

        let token_pair = process_new_token_pair(&client, &message.params.result.address)
            .await
            .unwrap_or_else(|e| {
                panic!("Error - Failed to process new token pair: {}", e);
            });

        token_pairs.push(token_pair);
    }

    Ok(())
}
