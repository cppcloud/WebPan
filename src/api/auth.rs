use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use log::{debug, error};

#[derive(Debug, Serialize, Deserialize)]
struct TokenData {
    accessToken: String,
    expiredAt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    code: i32,
    message: String,
    data: TokenData,
}


const TOKEN_URL: &str = "https://open-api.123pan.com/api/v1/access_token";

pub async fn fetch_access_token(client_id: &str, client_secret: &str) -> Result<String> {
    debug!("Fetching access token from {}", TOKEN_URL);

    let client = Client::new();
    let body = serde_json::json!({
        "clientID": client_id,
        "clientSecret": client_secret
    });

    let response = client
        .post(TOKEN_URL)
        .header("Content-Type", "application/json")
        .header("platform", "open_platform")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    debug!("Response status: {}", status);
    debug!("Response body: {}", response_text);

    if status.is_success() {
        // 解析嵌套的 JSON 响应
        match serde_json::from_str::<TokenResponse>(&response_text) {
            Ok(token_response) => {
                debug!("Successfully parsed token: {:?}", token_response.data);
                Ok(token_response.data.accessToken)
            }
            Err(e) => {
                error!("Failed to parse JSON: {}", e);
                Err(anyhow::anyhow!("JSON parsing error: {}", e))
            }
        }
    } else {
        error!("Failed to fetch access token: {}", response_text);
        Err(anyhow::anyhow!("API call failed: {}", response_text))
    }
}
