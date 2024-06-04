use binance::model::AccountInformation;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn get_openai_prediction(prompt: String, api_key: String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let request_body = OpenAIRequest {
        prompt: prompt.to_string(),
        max_tokens: 100,
    };

    let response = client
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?
        .json::<OpenAIResponse>()
        .await?;

    Ok(response.choices[0].text.clone())
}

pub async fn get_account_information_response(account_info: AccountInformation, api_key: String) -> Result<String, reqwest::Error> {
    let balances = account_info.balances.iter().map(|balance| {
        format!("{}: {}", balance.asset, balance.free)
    }).collect::<Vec<String>>().join(", ");

    let prompt = format!(
        "You have the following balances: {}. Your account can trade: {}, can withdraw: {}, can deposit: {}.",
        balances, account_info.can_trade, account_info.can_withdraw, account_info.can_deposit
    );

    let response = get_openai_prediction(prompt, api_key).await?;
    Ok(response)
}