use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

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

/// Response from POST /copilot_internal/v2/token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotTokenResponse {
    pub token: String,
    pub expires_at: i64,
    pub refresh_in: i64,
}

/// Individual quota detail (premium_interactions, chat, completions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaDetail {
    pub entitlement: Option<f64>,
    pub remaining: Option<f64>,
    pub percent_remaining: Option<f64>,
    pub overage_count: Option<f64>,
    pub overage_permitted: Option<bool>,
    pub unlimited: Option<bool>,
}

/// Response from GET /copilot_internal/user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotUsageResponse {
    pub copilot_plan: Option<String>,
    pub quota_reset_date: Option<String>,
    pub quota_snapshots: Option<HashMap<String, QuotaDetail>>,
}

/// Frontend-facing quota data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotQuota {
    pub plan: String,
    pub reset_date: String,
    pub premium_used_percent: f64,
    pub premium_remaining: f64,
    pub premium_total: f64,
    pub unlimited: bool,
}
