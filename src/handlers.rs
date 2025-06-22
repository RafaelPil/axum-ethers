use axum::{extract::Path, Json};
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::middleware::Middleware; // ✅ Required to use get_balance
use serde::Serialize;
use std::{env, str::FromStr, sync::Arc};
use serde_json::{json, Value};
use reqwest::Client;

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance_eth: String,
}

pub async fn get_eth_balance(Path(addr): Path<String>) -> Json<BalanceResponse> {
    let provider_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set in .env");
    let provider = match Provider::<Http>::try_from(provider_url) {
        Ok(p) => Arc::new(p),
        Err(_) => {
            return Json(BalanceResponse {
                address: addr,
                balance_eth: "Provider error".into(),
            })
        }
    };

    match Address::from_str(&addr) {
        Ok(address) => {
            match provider.get_balance(address, None).await {
                Ok(balance) => {
                    let balance_eth = ethers::utils::format_units(balance, "ether").unwrap_or("0".into());
                    Json(BalanceResponse {
                        address: addr,
                        balance_eth,
                    })
                }
                Err(_) => Json(BalanceResponse {
                    address: addr,
                    balance_eth: "Error fetching balance".into(),
                }),
            }
        }
        Err(_) => Json(BalanceResponse {
            address: addr,
            balance_eth: "Invalid address".into(),
        }),
    }
}

pub async fn get_token_balances(Path(addr): Path<String>) -> Json<Value> {
    let api_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set in .env");
    let client = Client::new();

    let body = json!({
        "jsonrpc": "2.0",
        "method": "alchemy_getTokenBalances",
        "params": [
            addr,
            "erc20",
            { "maxCount": 100 }
        ],
        "id": 0
    });

    match client.post(api_url)
        .json(&body)
        .send()
        .await {
        Ok(resp) => {
            let json: Value = resp.json().await.unwrap_or(json!({ "error": "Failed to parse response" }));
            Json(json)
        }
        Err(_) => Json(json!({ "error": "Request to Alchemy failed" })),
    }
}
