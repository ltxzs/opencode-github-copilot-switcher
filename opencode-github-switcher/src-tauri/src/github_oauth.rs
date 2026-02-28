use crate::error::AppError;
use crate::models::{AccessTokenResponse, DeviceCodeResponse, GitHubUser};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

const GITHUB_API_URL: &str = "https://api.github.com";

pub async fn get_device_code(client_id: &str) -> Result<DeviceCodeResponse, AppError> {
    let client = Client::new();
    let res = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id),
            ("scope", "read:user user:email repo"),
        ])
        .send()
        .await?;

    let response: DeviceCodeResponse = res.json().await?;
    Ok(response)
}

pub async fn poll_for_token(
    client_id: &str,
    device_code: &str,
    interval: u64,
    expires_in: u64,
) -> Result<String, AppError> {
    let client = Client::new();
    let max_retries = expires_in / interval;
    let mut retries = 0;

    loop {
        if retries >= max_retries {
            return Err(AppError::OAuth("Device code expired".to_string()));
        }

        let res = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", client_id),
                ("device_code", device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await?;

        let response: AccessTokenResponse = res.json().await?;

        if let Some(token) = response.access_token {
            return Ok(token);
        }

        if let Some(error) = response.error {
            match error.as_str() {
                "authorization_pending" => {
                    // Keep polling
                }
                "slow_down" => {
                    sleep(Duration::from_secs(interval + 5)).await;
                    continue;
                }
                "expired_token" => {
                    return Err(AppError::OAuth("Device code expired".to_string()));
                }
                "access_denied" => {
                    return Err(AppError::OAuth("Access denied by user".to_string()));
                }
                _ => {
                    return Err(AppError::OAuth(format!("OAuth error: {}", error)));
                }
            }
        }

        sleep(Duration::from_secs(interval)).await;
        retries += 1;
    }
}

pub async fn get_user_info(access_token: &str) -> Result<GitHubUser, AppError> {
    let client = Client::new();
    let res = client
        .get(format!("{}/user", GITHUB_API_URL))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "OpenCode-GitHub-Switcher")
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(AppError::OAuth(format!("Failed to fetch user info: {}", res.status())));
    }

    let user: GitHubUser = res.json().await?;
    Ok(user)
}
