use crate::error::AppError;
use crate::models::{CopilotQuota, CopilotTokenResponse, CopilotUsageResponse};
use reqwest::Client;

const COPILOT_HEADERS_USER_AGENT: &str = "GitHubCopilotChat/0.35.0";
const COPILOT_HEADERS_EDITOR_VERSION: &str = "vscode/1.107.0";
const COPILOT_HEADERS_PLUGIN_VERSION: &str = "copilot-chat/0.35.0";
const COPILOT_HEADERS_INTEGRATION_ID: &str = "vscode-chat";

fn build_copilot_client() -> Client {
    Client::new()
}

fn add_copilot_headers(
    req: reqwest::RequestBuilder,
) -> reqwest::RequestBuilder {
    req.header("User-Agent", COPILOT_HEADERS_USER_AGENT)
        .header("Editor-Version", COPILOT_HEADERS_EDITOR_VERSION)
        .header("Editor-Plugin-Version", COPILOT_HEADERS_PLUGIN_VERSION)
        .header("Copilot-Integration-Id", COPILOT_HEADERS_INTEGRATION_ID)
        .header("Accept", "application/json")
}

async fn exchange_copilot_token(oauth_token: &str) -> Result<String, AppError> {
    let client = build_copilot_client();
    let req = client
        .get("https://api.github.com/copilot_internal/v2/token")
        .header("Authorization", format!("token {}", oauth_token));
    let req = add_copilot_headers(req);

    let res = req.send().await?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(AppError::OAuth(format!(
            "Token exchange failed ({}): {}",
            status, body
        )));
    }

    let token_resp: CopilotTokenResponse = res.json().await?;
    Ok(token_resp.token)
}

async fn fetch_usage_with_token(token: &str, auth_prefix: &str) -> Result<CopilotUsageResponse, AppError> {
    let client = build_copilot_client();
    let req = client
        .get("https://api.github.com/copilot_internal/user")
        .header("Authorization", format!("{} {}", auth_prefix, token));
    let req = add_copilot_headers(req);

    let res = req.send().await?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(AppError::OAuth(format!(
            "Quota fetch failed ({}): {}",
            status, body
        )));
    }

    let usage: CopilotUsageResponse = res.json().await?;
    Ok(usage)
}

fn parse_usage_to_quota(usage: &CopilotUsageResponse) -> CopilotQuota {
    let plan = usage.copilot_plan.clone().unwrap_or_else(|| "unknown".to_string());
    let reset_date = usage.quota_reset_date.clone().unwrap_or_default();

    let (premium_used_percent, premium_remaining, premium_total, unlimited) =
        if let Some(snapshots) = &usage.quota_snapshots {
            if let Some(premium) = snapshots.get("premium_interactions") {
                let total = premium.entitlement.unwrap_or(0.0);
                let remaining = premium.remaining.unwrap_or(0.0);
                let is_unlimited = premium.unlimited.unwrap_or(false);
                let percent_remaining = premium.percent_remaining.unwrap_or(100.0);
                let used_percent = 100.0 - percent_remaining;
                (used_percent, remaining, total, is_unlimited)
            } else {
                (0.0, 0.0, 0.0, false)
            }
        } else {
            (0.0, 0.0, 0.0, false)
        };

    CopilotQuota {
        plan,
        reset_date,
        premium_used_percent,
        premium_remaining,
        premium_total,
        unlimited,
    }
}

/// Fallback strategy: try direct token first, then exchange if that fails
pub async fn fetch_copilot_quota(oauth_token: &str) -> Result<CopilotQuota, AppError> {
    if let Ok(usage) = fetch_usage_with_token(oauth_token, "token").await {
        return Ok(parse_usage_to_quota(&usage));
    }

    let session_token = exchange_copilot_token(oauth_token).await?;
    let usage = fetch_usage_with_token(&session_token, "Bearer").await?;
    Ok(parse_usage_to_quota(&usage))
}
