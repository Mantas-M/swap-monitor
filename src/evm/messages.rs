use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RPCResponse {
    pub id: u32,
    pub result: RPCResult,
    pub jsonrpc: String,
}

#[derive(Debug)]
pub enum RPCResult {
    String(String),
    TokenBalancesResult(TokenBalancesResult),
}

#[derive(Debug, Deserialize)]
pub struct TokenBalancesResult {
    pub address: String,
    #[serde(rename = "tokenBalances")]
    pub token_balances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize)]
pub struct TokenBalance {
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "tokenBalance")]
    pub token_balance: String,
}

#[derive(Debug, Deserialize)]
pub struct EthSubscriptionResponse {
    pub jsonrpc: String,
    pub method: String,
    pub params: EthSubscriptionResponseParams,
}
#[derive(Debug, Deserialize)]
pub struct EthSubscriptionResponseParams {
    pub result: EthSubscriptionResult,
    pub subscription: String,
}
#[derive(Debug, Deserialize)]
pub struct EthSubscriptionResult {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "logIndex")]
    pub log_index: String,
    pub removed: bool,
}

pub enum MessageResponse {
    RPC(RPCResponse),
    EthSubscription(EthSubscriptionResponse),
}
