mod evm;
use dotenv::dotenv;
use evm::rpc_api::{get_block_number, get_pair_token_addresses, get_token_balances};
use evm::ws_api::subscribe_to_logs;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = Client::new();

    let block_number = match get_block_number(&client).await {
        Ok(block_number) => block_number,
        Err(e) => {
            panic!("Error - Websocket link not found: {}", e);
        }
    };

    println!("Block number: {}\n", block_number);

    let mut rx = subscribe_to_logs()
        .await
        .expect("Failed to subscribe to logs");

    while let Some(message) = rx.recv().await {
        println!("Received message: {:?}", message.params.result.address);

        // get_token_balances(&client, message.params.result.address).await?;

        get_pair_token_addresses(&client, &message.params.result.address).await?;
    }

    Ok(())
}
