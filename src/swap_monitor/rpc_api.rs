use crate::swap_monitor::messages::{RPCResponse, RPCResult, TokenBalancesResult};
use crate::swap_monitor::utils::{
    generate_function_signature, hex_to_bigint, hex_to_decimal, hex_to_utf8, unpad,
};
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};
use std::env;

use super::token_pair::Token;

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
            panic!("Error - Failed to handle response: {}", e);
        }
    }

    Ok(())
}

pub async fn read_contract(
    client: &Client,
    contract_address: &str,
    property: &str,
) -> Result<String, Error> {
    let url: String = get_rpc_url();

    let function_signature: String = generate_function_signature(format!("{}()", &property));
    let body = generate_rpc_body(&contract_address, &function_signature, "eth_call");

    let response: Response = client.post(url).body(body.to_string()).send().await?;

    let result = if let Ok(RPCResult::String(address)) = handle_response(response).await {
        address
    } else {
        panic!("Error - Unexpected result type");
    };

    Ok(result)
}

pub async fn get_token_info(client: &Client, address: &str) -> Token {
    if address.to_lowercase() == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" {
        let total_supply_hex: String = read_contract(client, address, "totalSupply")
            .await
            .unwrap_or_else(|e| {
                panic!("Error - Failed to get token total supply: {}", e);
            });

        return Token {
            name: "Wrapped Ether".to_string(),
            symbol: "WETH".to_string(),
            decimals: 18,
            total_supply: hex_to_bigint(&unpad(&total_supply_hex)),
            address: address.to_string(),
        };
    }

    let name_hex: String = read_contract(client, address, "name")
        .await
        .unwrap_or_else(|e| {
            panic!("Error - Failed to get token name: {}", e);
        });

    let symbol_hex: String = read_contract(client, address, "symbol")
        .await
        .unwrap_or_else(|e| {
            panic!("Error - Failed to get token symbol: {}", e);
        });

    let decimals_hex: String = read_contract(client, address, "decimals")
        .await
        .unwrap_or_else(|e| {
            panic!("Error - Failed to get token decimals: {}", e);
        });

    let total_supply_hex: String = read_contract(client, address, "totalSupply")
        .await
        .unwrap_or_else(|e| {
            panic!("Error - Failed to get token total supply: {}", e);
        });

    Token {
        address: address.to_string(),
        name: hex_to_utf8(&unpad(&name_hex)),
        symbol: hex_to_utf8(&unpad(&symbol_hex)),
        decimals: hex_to_decimal(&unpad(&decimals_hex)),
        total_supply: hex_to_bigint(&unpad(&total_supply_hex)),
    }
}

pub async fn get_pair_token_addresses(
    client: &Client,
    pair_address: &str,
) -> Result<(String, String), Error> {
    let token_0_address = read_contract(client, pair_address, "token0")
        .await
        .unwrap_or_else(|e| {
            panic!("Error - Failed to get token 0 address: {}", e);
        });

    let token_1_address = read_contract(client, pair_address, "token1")
        .await
        .unwrap_or_else(|e| {
            panic!("Error - Failed to get token 1 address: {}", e);
        });

    Ok((unpad(&token_0_address), unpad(&token_1_address)))
}

pub fn generate_rpc_body(address: &str, function_signature: &str, rpc_method: &str) -> String {
    json!({
      "id": 1,
      "jsonrpc": "2.0",
      "method": rpc_method,
      "params": [
        {
          "to": address,
          "data": function_signature
        },
        "latest"
      ]
    })
    .to_string()
}

async fn handle_response(response: Response) -> Result<RPCResult, Error> {
    if response.status().is_success() {
        let body: String = response.text().await?;

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

fn get_rpc_url() -> String {
    match env::var("RPC_ETH_MAINNET") {
        Ok(link) => link,
        Err(e) => panic!("Error - Websocket link not found: {}", e),
    }
}
