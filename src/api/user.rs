use crate::models::user_model::UserInfo;
use crate::utils::http_client::HttpClient;
use anyhow::Result;
use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};
use log::{debug, error};
use serde::Deserialize;

const API_URL: &str = "https://open-api.123pan.com/api/v1/user/info";

#[derive(Debug, Deserialize)]
struct UserResponse {
    code: i32,
    message: String,
    data: UserInfo, // 嵌套的用户数据
}

pub async fn get_user_info(access_token: &str) -> Result<UserInfo> {
    debug!("Fetching user info with access token: {}", access_token);

    let client = HttpClient::new();
    let response = client
        .get(API_URL)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token))
        .header("platform", "open_platform")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    debug!("User info response status: {}", status);
    debug!("User info response body: {}", response_text);

    if status.is_success() {
        // 解析嵌套的 JSON 响应
        match serde_json::from_str::<UserResponse>(&response_text) {
            Ok(user_response) => {
                debug!("Successfully fetched user info: {:?}", user_response.data);
                Ok(user_response.data)
            }
            Err(e) => {
                error!("JSON parsing error: {}", e);
                Err(anyhow::anyhow!("Failed to parse user info: {}", e))
            }
        }
    } else {
        error!("Failed to fetch user info: {}", response_text);
        Err(anyhow::anyhow!("API call failed: {}", response_text))
    }
}