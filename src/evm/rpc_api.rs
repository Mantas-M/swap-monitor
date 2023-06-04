use crate::evm::messages::{RPCResponse, RPCResult, TokenBalancesResult};
use ethers::types::H256;
use ethers::utils::keccak256;
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};
use std::env;

fn get_rpc_url() -> String {
    match env::var("RPC_ETH_MAINNET") {
        Ok(link) => link,
        Err(e) => panic!("Error - Websocket link not found: {}", e),
    }
}

pub async fn get_block_number(client: &Client) -> Result<String, Error> {
    let url: String = get_rpc_url();

    let body: &str = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_blockNumber\",\"params\":[]}";
    let response: Response = client.post(url).body(body).send().await?;

    let block_number = match handle_response(response).await {
        Ok(RPCResult::String(block_number)) => block_number,
        Ok(_) => {
            panic!("Error - Failed to handle response");
        }
        Err(e) => {
            panic!("Error - Failed to handle response: {}", e)
        }
    };

    Ok(block_number)
}

pub async fn get_token_balances(client: &Client, address: String) -> Result<(), Error> {
    let url: String = get_rpc_url();

    let body = json!({
      "id": 1,
      "jsonrpc": "2.0",
      "method": "alchemy_getTokenBalances",
      "params": [
        address.to_string(),
      ]
    });

    let response: Response = client.post(url).body(body.to_string()).send().await?;

    match handle_response(response).await {
        Ok(RPCResult::TokenBalancesResult(balances)) => {
            println!("Balances: {:?}\n", balances);
        }
        Ok(_) => {
            panic!("Error - Failed to handle response");
        }
        Err(e) => {
            panic!("Error - Failed to handle response: {}", e)
        }
    };

    Ok(())
}

pub async fn get_pair_token_addresses(client: &Client, address: &String) -> Result<(), Error> {
    let url: String = get_rpc_url();

    let token_0_method = "token0()";
    let token_1_method = "token1()";

    let token_0_hash = H256::from_slice(keccak256(token_0_method).as_slice());
    let token_0_selector = &token_0_hash.as_bytes()[0..4];
    let token_0_selector_string: String = token_0_selector
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    let token_0_data = format!("0x{}", token_0_selector_string);

    let token_1_hash = H256::from_slice(keccak256(token_1_method).as_slice());
    let token_1_selector = &token_1_hash.as_bytes()[0..4];
    let token_1_selector_string: String = token_1_selector
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    let token_1_data = format!("0x{}", token_1_selector_string);

    let token_0_body = json!({
      "id": 1,
      "jsonrpc": "2.0",
      "method": "eth_call",
      "params": [
        {
          "to": address.to_string(),
          "data": token_0_data
        },
        "latest"
      ]
    });

    let token_1_body = json!({
      "id": 1,
      "jsonrpc": "2.0",
      "method": "eth_call",
      "params": [
        {
          "to": address.to_string(),
          "data": token_1_data
        },
        "latest"
      ]
    });

    let token_0_response: Response = client
        .post(&url)
        .body(token_0_body.to_string())
        .send()
        .await?;

    match handle_response(token_0_response).await {
        Ok(RPCResult::String(address)) => {
            println!("Address of token 0: {:?}\n", address);
        }
        Ok(_) => {
            panic!("Error - Failed to handle response");
        }
        Err(e) => {
            panic!("Error - Failed to handle response: {}", e)
        }
    };

    let token_1_response: Response = client
        .post(&url)
        .body(token_1_body.to_string())
        .send()
        .await?;

    match handle_response(token_1_response).await {
        Ok(RPCResult::String(address)) => {
            println!("Address of token 1: {:?}\n", address);
        }
        Ok(_) => {
            panic!("Error - Failed to handle response");
        }
        Err(e) => {
            panic!("Error - Failed to handle response: {}", e)
        }
    };

    Ok(())
}

async fn handle_response(response: Response) -> Result<RPCResult, Error> {
    if response.status().is_success() {
        let body: String = response.text().await?;

        println!("Response: {}", body);

        let rpc_data: RPCResponse = match serde_json::from_str(&body) {
            Ok(rpc_data) => rpc_data,
            Err(e) => {
                panic!("Error - Failed to parse response: {}", e)
            }
        };

        match rpc_data.result {
            RPCResult::String(s) => Ok(RPCResult::String(s)),
            RPCResult::TokenBalancesResult(tbr) => Ok(RPCResult::TokenBalancesResult(tbr)),
        }
    } else {
        let status = response.status();
        let reason = response.text().await?;
        panic!(
            "Request failed with status: {} - Reason: {}",
            status, reason
        )
    }
}

impl<'de> Deserialize<'de> for RPCResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        if value.is_string() {
            Ok(RPCResult::String(value.as_str().unwrap().to_string()))
        } else {
            Ok(RPCResult::TokenBalancesResult(
                serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            ))
        }
    }
}
