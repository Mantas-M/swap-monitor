use crate::swap_monitor::messages::{EthSubscriptionResponse, MessageResponse, RPCResponse};
use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc::{self, Receiver};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub async fn subscribe_to_logs(
) -> Result<Receiver<EthSubscriptionResponse>, Box<dyn std::error::Error>> {
    let mut ws_stream = match connect_ws().await {
        Ok(ws_stream) => ws_stream,
        Err(e) => {
            panic!("Error - Failed to connect to websocket: {}", e)
        }
    };

    let request = generate_request();

    match ws_stream.send(Message::Text(request.to_string())).await {
        Ok(_) => println!("Sent subscription request"),
        Err(e) => {
            panic!("Error - Failed to send subscription request: {}", e);
        }
    }

    let (tx, rx) = mpsc::channel::<EthSubscriptionResponse>(100);

    tokio::spawn(async move {
        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(message) => match process_message(&message.to_string()).await {
                    Ok(MessageResponse::RPC(_rpc_response)) => {
                        println!("Subscription confirmed",);
                    }
                    Ok(MessageResponse::EthSubscription(sub_response)) => {
                        if let Err(_) = tx.send(sub_response).await {
                            break;
                        }
                    }
                    Err(_) => {
                        println!("Error - Failed to process message");
                    }
                },
                Err(e) => {
                    println!("Error - Failed to receive message: {}", e);
                }
            }
        }
    });

    Ok(rx)
}

pub fn generate_request() -> Value {
    return json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_subscribe",
        "params": [
            "logs",
            {
                "topics": [
                    ["0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822", "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"],
                ]
            }
        ]
    });
}

pub async fn process_message(json: &str) -> Result<MessageResponse, ()> {
    let json_value: Value = serde_json::from_str(json).expect("Failed to deserialize JSON");

    if let Some(_) = json_value.get("id").and_then(Value::as_u64) {
        let initial_message: RPCResponse =
            serde_json::from_value(json_value).expect("Failed to deserialize InitialMessage");

        Ok(MessageResponse::RPC(initial_message))
    } else if let Some(_) = json_value.get("method").and_then(Value::as_str) {
        let eth_subscription: EthSubscriptionResponse =
            serde_json::from_value(json_value).expect("Failed to deserialize EthSubscription");

        Ok(MessageResponse::EthSubscription(eth_subscription))
    } else {
        println!("Received unrecognized message: {:?}", json_value);
        Err(())
    }
}

pub async fn connect_ws() -> Result<
    WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    Box<dyn std::error::Error>,
> {
    let link: String = match env::var("WS_ETH_MAINNET") {
        Ok(link) => link,
        Err(e) => {
            panic!("Error - Websocket link not found: {}", e)
        }
    };

    println!("Connecting to provider...",);
    let (ws_stream, _) = match connect_async(link).await {
        Ok(result) => result,
        Err(e) => {
            panic!("Error - Websocket link not found: {}", e)
        }
    };
    println!("Connected!");

    Ok(WebSocketStream::from(ws_stream))
}
