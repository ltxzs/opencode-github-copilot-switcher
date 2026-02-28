use crate::error::AppError;
use dirs_next::data_local_dir;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use std::str::FromStr;

pub async fn init_db() -> Result<SqlitePool, AppError> {
    let data_dir = data_local_dir().ok_or_else(|| AppError::System("Could not find local data directory".to_string()))?;
    let app_dir = data_dir.join("opencode-github-switcher");
    
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)?;
    }
    
    let db_path = app_dir.join("providers.db");
    let db_path_str = db_path.to_string_lossy().replace("\\", "/");
    let db_url = format!("sqlite://{}?mode=rwc", db_path_str);
    
    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);
        
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
        
    // Create tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS github_providers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            access_token TEXT NOT NULL,
            email TEXT,
            avatar_url TEXT,
            github_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            last_used_at INTEGER
        )"
    )
    .execute(&pool)
    .await?;
    
    // Add avatar_url column if it doesn't exist (for backwards compatibility with older local db)
    let _ = sqlx::query("ALTER TABLE github_providers ADD COLUMN avatar_url TEXT")
        .execute(&pool)
        .await;

    Ok(pool)
}
