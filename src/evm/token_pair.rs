use crate::evm::rpc_api::{get_pair_token_addresses, get_token_info};
use dotenv::Error;
use num_bigint::BigUint;
use reqwest::Client;

pub struct TokenPair {
    pub address: String,
    pub token_0: Token,
    pub token_1: Token,
    pub num_updates: u32,
}

#[derive(Debug)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: BigUint,
    pub address: String,
}

pub async fn process_new_token_pair(client: &Client, address: &str) -> Result<TokenPair, Error> {
    let (token_0_address, token_1_address) = get_pair_token_addresses(client, address)
        .await
        .unwrap_or_else(|e| {
            panic!("Error - Failed to get token addresses: {}", e);
        });

    let token_0_info: Token = get_token_info(client, &token_0_address).await;
    let token_1_info: Token = get_token_info(client, &token_1_address).await;

    println!("Token 0: ");
    println!("Name: {}", token_0_info.name);
    println!("Symbol: {}", token_0_info.symbol);
    println!("Decimals: {}", token_0_info.decimals);
    println!("Total supply: {}", token_0_info.total_supply);
    println!("Address: {}", token_0_info.address);

    println!("Token 1:");
    println!("Name: {}", token_1_info.name);
    println!("Symbol: {}", token_1_info.symbol);
    println!("Decimals: {}", token_1_info.decimals);
    println!("Total supply: {}", token_1_info.total_supply);
    println!("Address: {}", token_1_info.address);

    Ok(TokenPair {
        address: address.to_string(),
        token_0: token_0_info,
        token_1: token_1_info,
        num_updates: 1,
    })
}
