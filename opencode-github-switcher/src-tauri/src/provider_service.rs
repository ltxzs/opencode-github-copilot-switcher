use crate::auth_config::{update_auth_json, read_current_token};
use crate::error::AppError;
use crate::github_oauth::{get_device_code, get_user_info, poll_for_token};
use crate::models::{DeviceCodeResponse, GitHubProvider};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_providers(pool: &SqlitePool) -> Result<Vec<GitHubProvider>, AppError> {
    let providers = sqlx::query_as::<_, GitHubProvider>("SELECT * FROM github_providers ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;
    Ok(providers)
}

pub async fn start_oauth_flow(client_id: &str) -> Result<DeviceCodeResponse, AppError> {
    get_device_code(client_id).await
}

pub async fn complete_oauth_flow(
    pool: &SqlitePool,
    client_id: &str,
    device_code: &str,
    interval: u64,
    expires_in: u64,
) -> Result<GitHubProvider, AppError> {
    let access_token = poll_for_token(client_id, device_code, interval, expires_in).await?;
    let user_info = get_user_info(&access_token).await?;

    let now = Utc::now().timestamp();
    let id = Uuid::new_v4().to_string();

    let provider = GitHubProvider {
        id: id.clone(),
        name: user_info.login,
        access_token,
        email: user_info.email,
        avatar_url: user_info.avatar_url,
        github_id: user_info.id,
        created_at: now,
        last_used_at: Some(now),
    };

    sqlx::query(
        "INSERT INTO github_providers (id, name, access_token, email, avatar_url, github_id, created_at, last_used_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&provider.id)
    .bind(&provider.name)
    .bind(&provider.access_token)
    .bind(&provider.email)
    .bind(&provider.avatar_url)
    .bind(provider.github_id)
    .bind(provider.created_at)
    .bind(provider.last_used_at)
    .execute(pool)
    .await?;

    // Also update auth.json immediately
    update_auth_json(&provider.access_token, &provider.name)?;

    Ok(provider)
}

pub async fn delete_provider(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM github_providers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn switch_provider(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    let provider = sqlx::query_as::<_, GitHubProvider>("SELECT * FROM github_providers WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::ProviderNotFound(id.to_string()))?;

    // Update auth.json
    update_auth_json(&provider.access_token, &provider.name)?;

    // Update last_used_at
    let now = Utc::now().timestamp();
    sqlx::query("UPDATE github_providers SET last_used_at = ? WHERE id = ?")
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn sync_active_account(pool: &SqlitePool) -> Result<(), AppError> {
    if let Some(token) = read_current_token() {
        // Try to fetch user info to verify token and get details
        if let Ok(user_info) = get_user_info(&token).await {
            let now = Utc::now().timestamp();
            
            // Check if this provider already exists
            let existing: Option<GitHubProvider> = sqlx::query_as(
                "SELECT * FROM github_providers WHERE github_id = ?"
            )
            .bind(user_info.id)
            .fetch_optional(pool)
            .await?;

            if let Some(mut provider) = existing {
                // Update token and last used time if it already exists
                sqlx::query(
                    "UPDATE github_providers SET access_token = ?, name = ?, email = ?, avatar_url = ?, last_used_at = ? WHERE github_id = ?"
                )
                .bind(&token)
                .bind(&user_info.login)
                .bind(&user_info.email)
                .bind(&user_info.avatar_url)
                .bind(now)
                .bind(user_info.id)
                .execute(pool)
                .await?;
            } else {
                // Insert new provider if it doesn't exist
                let id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO github_providers (id, name, access_token, email, avatar_url, github_id, created_at, last_used_at) 
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&id)
                .bind(&user_info.login)
                .bind(&token)
                .bind(&user_info.email)
                .bind(&user_info.avatar_url)
                .bind(user_info.id)
                .bind(now)
                .bind(now)
                .execute(pool)
                .await?;
            }
        }
    }
    Ok(())
}
