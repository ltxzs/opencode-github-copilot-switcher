use crate::database::init_db;
use crate::error::AppError;
use crate::models::{DeviceCodeResponse, GitHubProvider};
use crate::provider_service;
use crate::state::AppState;
use tauri::State;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;
use sqlx::SqlitePool;

async fn get_db(state: &State<'_, AppState>) -> Result<SqlitePool, AppError> {
    let mut db_guard = state.db.lock().await;
    if let Some(pool) = db_guard.as_ref() {
        return Ok(pool.clone());
    }
    
    let pool = init_db().await?;
    *db_guard = Some(pool.clone());
    Ok(pool)
}

#[tauri::command]
pub async fn list_providers(state: State<'_, AppState>) -> Result<Vec<GitHubProvider>, AppError> {
    let pool = get_db(&state).await?;
    provider_service::list_providers(&pool).await
}

#[tauri::command]
pub async fn start_device_flow(client_id: String) -> Result<DeviceCodeResponse, AppError> {
    provider_service::start_oauth_flow(&client_id).await
}

#[tauri::command]
pub async fn complete_device_flow(
    state: State<'_, AppState>,
    client_id: String,
    device_code: String,
    interval: u64,
    expires_in: u64,
) -> Result<GitHubProvider, AppError> {
    let pool = get_db(&state).await?;
    provider_service::complete_oauth_flow(&pool, &client_id, &device_code, interval, expires_in).await
}

#[tauri::command]
pub async fn delete_provider(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let pool = get_db(&state).await?;
    provider_service::delete_provider(&pool, &id).await
}

#[tauri::command]
pub async fn switch_provider(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let pool = get_db(&state).await?;
    provider_service::switch_provider(&pool, &id).await
}

#[tauri::command]
pub async fn open_url(app: AppHandle, url: String) -> Result<(), String> {
    app.shell().open(url, None).map_err(|e| e.to_string())
}
