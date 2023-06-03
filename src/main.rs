mod evm;
use dotenv::dotenv;
use evm::rpc_api::get_block_number;
use evm::ws_api::subscribe_to_logs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let block_number = match get_block_number().await {
        Ok(block_number) => block_number,
        Err(e) => {
            panic!("Error - Websocket link not found: {}", e);
        }
    };

    println!("Block number: {}", block_number);

    subscribe_to_logs().await?;

    Ok(())
}
