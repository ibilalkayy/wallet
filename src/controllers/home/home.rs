use reqwest::Client;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

// Define the structure of the JSON response
#[derive(Deserialize, Debug)]
struct AlchemyResponse {
    result: String,
}

pub async fn check_balance() -> Result<String, ()> {
    dotenv().ok();
    
    let api_key = env::var("ALCHEMY_API_KEY").expect("Missing API key");
    let wallet_address = env::var("WALLET_ADDRESS").expect("Missing wallet address");

    let client = Client::new();
    
    // Alchemy JSON-RPC URL
    let url = format!("https://eth-sepolia.g.alchemy.com/v2/{}", api_key);

    // JSON-RPC payload for eth_getBalance
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getBalance",
        "params": [wallet_address, "latest"],
        "id": 1
    });

    // Send the request
    match client.post(&url).json(&body).send().await {
        Ok(response) => {
            if let Ok(result) = response.json::<AlchemyResponse>().await {
                let balance_in_wei = u128::from_str_radix(&result.result.trim_start_matches("0x"), 16).unwrap_or(0);
                let balance_in_eth = balance_in_wei as f64 / 1e18;
                let formatted_balance = format!("{:.4} ETH", balance_in_eth);
                Ok(formatted_balance)
            } else {
                panic!("⚠️ Failed to parse response");
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}
