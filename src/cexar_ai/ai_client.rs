use binance::model::AccountInformation;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize)]
pub struct OpenAIRequest {
    pub prompt: String,
    pub max_tokens: u16,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub text: String,
}

#[derive(Debug)]
struct OpenAIError {
    status: reqwest::StatusCode,
    message: String,
}

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OpenAIError: {} - {}", self.status, self.message)
    }
}

impl std::error::Error for OpenAIError {}

pub async fn get_openai_prediction(
    prompt: String,
    api_key: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let _request_body = OpenAIRequest {
        prompt: prompt.to_string(),
        max_tokens: 100,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo-16k",
            "messages": [{
                "role": "user",
                "content": prompt
            }],
            "temperature": 1,
            "max_tokens": 256,
            "top_p": 1,
            "frequency_penalty": 0,
            "presence_penalty": 0
        }))
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        eprintln!("Failed request: {}", text);
        return Err(Box::new(OpenAIError {
            status,
            message: format!("Failed with status: {}", status),
        }));
    }

    // Print the raw response for debugging
    println!("Raw response: {}", text);

    match serde_json::from_str::<OpenAIResponse>(&text) {
        Ok(parsed_response) => Ok(parsed_response.choices[0].text.clone()),
        Err(e) => {
            eprintln!("Failed to parse response: {}", text);
            Err(Box::new(OpenAIError {
                status,
                message: format!("Failed to parse response: {}", e),
            }))
        }
    }
}

pub async fn get_account_information_response(
    account_info: AccountInformation,
    api_key: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let balances = account_info
        .balances
        .iter()
        .map(|balance| format!("{}: {}", balance.asset, balance.free))
        .collect::<Vec<String>>()
        .join(", ");

    let prompt = format!(
        "You have the following balances: {}. Your account can trade: {}, can withdraw: {}, can deposit: {}.",
        balances, account_info.can_trade, account_info.can_withdraw, account_info.can_deposit
    );

    let response = get_openai_prediction(prompt, api_key).await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::secrets::Config;
    use binance::model::Balance;
    use std::error::Error;

    fn get_api_key() -> String {
        let config = Config::load().unwrap();
        config.openai_api_key
    }

    #[tokio::test]
    async fn test_get_openai_prediction() {
        let api_key = get_api_key();
        let prompt = String::from("Give me a summary of the latest market trends.");

        match get_openai_prediction(prompt, api_key).await {
            Ok(result) => {
                println!("Prediction: {}", result);
                assert!(!result.is_empty());
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                // Retry logic or handling different kinds of errors can go here
                // For now, we will print the raw response if decoding failed
                if let Some(status) = e.downcast_ref::<OpenAIError>() {
                    eprintln!("Status: {}", status);
                }
                if let Some(source) = e.source() {
                    eprintln!("Source: {:?}", source);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_account_information_response() {
        let api_key = get_api_key();
        let account_info = AccountInformation {
            maker_commission: 10.0,
            taker_commission: 10.0,
            buyer_commission: 0.0,
            seller_commission: 0.0,
            can_trade: true,
            can_withdraw: true,
            can_deposit: true,
            balances: vec![
                Balance {
                    asset: "BTC".to_string(),
                    free: "0.1".to_string(),
                    locked: "0.0".to_string(),
                },
                Balance {
                    asset: "ETH".to_string(),
                    free: "1.0".to_string(),
                    locked: "0.0".to_string(),
                },
            ],
        };
        let result = get_account_information_response(account_info, api_key)
            .await
            .unwrap();
        println!("Response: {}", result);
        assert!(!result.is_empty());
    }
}
