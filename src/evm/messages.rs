use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RPCResponse {
    pub id: u32,
    pub result: String,
    pub jsonrpc: String,
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
