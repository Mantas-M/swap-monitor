use crate::evm::messages::RPCResponse;
use reqwest::{Client, Error, Response};
use std::env;

pub async fn get_block_number() -> Result<String, Error> {
    let client = Client::new();

    let url: String = match env::var("RPC_ETH_MAINNET") {
        Ok(link) => link,
        Err(e) => {
            panic!("Error - Websocket link not found: {}", e)
        }
    };

    let body: &str = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_blockNumber\",\"params\":[]}";
    let response: Response = client.post(url).body(body).send().await?;

    let block_number = match handle_response(response).await {
        Ok(block_number) => block_number,
        Err(e) => {
            panic!("Error - Failed to handle response: {}", e)
        }
    };

    Ok(block_number)
}

async fn handle_response(response: Response) -> Result<String, Error> {
    if response.status().is_success() {
        let body: String = response.text().await?;
        let rpc_data: RPCResponse =
            serde_json::from_str(&body).expect("Failed to deserialize JSON");

        Ok(rpc_data.result)
    } else {
        let status = response.status();
        let reason = response.text().await?;
        panic!(
            "Request failed with status: {} - Reason: {}",
            status, reason
        )
    }
}
