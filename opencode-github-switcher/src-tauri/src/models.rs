use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct GitHubProvider {
    pub id: String,
    pub name: String,
    pub access_token: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub github_id: i64,
    pub created_at: i64,
    pub last_used_at: Option<i64>,
}

impl GitHubProvider {
    pub fn new(
        id: String,
        name: String,
        access_token: String,
        email: Option<String>,
        avatar_url: Option<String>,
        github_id: i64,
        created_at: i64,
        last_used_at: Option<i64>,
    ) -> Self {
        Self {
            id,
            name,
            access_token,
            email,
            avatar_url,
            github_id,
            created_at,
            last_used_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeInfo {
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

impl DeviceCodeResponse {
    pub fn to_info(&self) -> DeviceCodeInfo {
        DeviceCodeInfo {
            user_code: self.user_code.clone(),
            verification_uri: self.verification_uri.clone(),
            expires_in: self.expires_in,
            interval: self.interval,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubEmail {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}
