# AGENTS.md — OpenCode GitHub Copilot Switcher

## Project Overview

Tauri v2 desktop app for managing/switching GitHub Copilot accounts in OpenCode.
- **Frontend**: React 19 + Vite + Tailwind CSS (JSX, no TypeScript)
- **Backend**: Rust (Tauri v2, SQLite via SQLx, reqwest for HTTP)
- **Platforms**: Windows, macOS, Linux

## Directory Structure

```
opencode-githubcopilot-switcher/          # Git root
├── opencode-github-switcher/             # App root (run commands here)
│   ├── src/                              # React frontend
│   │   ├── App.jsx                       # Main (and only) component
│   │   ├── main.jsx                      # React entry point
│   │   ├── i18n.js                       # i18n translations (en/zh/ja)
│   │   ├── index.css                     # Tailwind directives + base styles
│   │   └── App.css                       # Legacy CSS (mostly unused)
│   ├── src-tauri/                        # Rust backend
│   │   ├── src/
│   │   │   ├── lib.rs                    # Tauri app builder, command registration
│   │   │   ├── main.rs                   # Entry point (calls lib::run)
│   │   │   ├── commands.rs               # Tauri IPC command handlers
│   │   │   ├── models.rs                 # Data structs (GitHubProvider, etc.)
│   │   │   ├── error.rs                  # AppError enum (thiserror)
│   │   │   ├── database.rs              # SQLite init/migrations
│   │   │   ├── github_oauth.rs          # GitHub Device Flow OAuth
│   │   │   ├── auth_config.rs           # Read/write OpenCode auth.json
│   │   │   ├── provider_service.rs      # Business logic layer
│   │   │   └── state.rs                 # AppState (db connection pool)
│   │   └── Cargo.toml
│   ├── package.json
│   ├── vite.config.js
│   ├── tailwind.config.js
│   └── eslint.config.js
└── .github/workflows/release.yml         # CI: cross-platform release builds
```

## Build / Dev / Lint Commands

All commands run from `opencode-github-switcher/` directory:

```bash
# Frontend only
npm run dev              # Vite dev server (frontend only)
npm run build            # Vite production build
npm run lint             # ESLint (JS/JSX files)

# Full Tauri app
npm run tauri dev        # Dev mode with hot reload (frontend + backend)
npm run tauri build      # Production build (creates platform installers)
```

Rust-specific (from `opencode-github-switcher/src-tauri/`):

```bash
cargo check              # Type check Rust code
cargo build              # Build Rust backend
cargo clippy             # Lint Rust code
cargo test               # Run Rust tests (none currently exist)
```

**No test framework is configured.** There are no frontend or backend tests.

## Frontend Code Style (React/JSX)

### General
- **No TypeScript** — plain JavaScript with `.jsx` extensions
- ES Modules (`"type": "module"` in package.json)
- React 19 with hooks (useState, useEffect)
- Single-file app architecture — `App.jsx` is the sole component

### Imports
- Named imports from libraries: `import { useState, useEffect } from 'react'`
- Tauri API: `import { invoke } from '@tauri-apps/api/core'`
- Icons from lucide-react: `import { Github, Plus, Trash2 } from 'lucide-react'`
- Local modules: `import { translations, languages } from './i18n'`
- CSS imported in entry: `import './index.css'`

### Components
- Functional components only, with hooks
- Default export: `export default function App() { ... }`
- Inline event handlers for simple logic
- State at component top level, grouped by concern

### Styling
- **Tailwind CSS v3** utility classes inline on JSX elements
- `clsx` and `tailwind-merge` available for conditional classes
- Minimal custom CSS in `index.css` (Tailwind directives only)
- `App.css` exists but is legacy/unused boilerplate

### i18n
- Simple key-value translation objects in `src/i18n.js`
- Translation helper: `const t = (key) => translations[currentLang]?.[key] || translations['en'][key]`
- Three languages: English, 简体中文, 日本語
- Language preference persisted in localStorage

### ESLint Rules
- `no-unused-vars` with `varsIgnorePattern: '^[A-Z_]'` (allows unused uppercase/underscore vars)
- react-hooks/recommended enabled
- react-refresh/vite enabled
- `dist/` directory ignored

### Tauri IPC Pattern
```jsx
// Frontend calls Rust commands via invoke()
const data = await invoke('command_name', { paramName: value });
// Parameter names use camelCase on JS side, snake_case on Rust side
// Tauri auto-converts between the two
```

## Backend Code Style (Rust)

### General
- Rust 2021 edition, minimum rustc 1.71
- Async runtime: Tokio (full features)
- Error handling via `thiserror` derive macro

### Naming
- `snake_case` for functions, variables, modules
- `PascalCase` for types, structs, enums
- Module files map 1:1 to concerns (one module per domain)

### Error Handling
- Central `AppError` enum in `error.rs` with `#[derive(Error)]`
- Variants use `#[from]` for automatic conversion from library errors
- `AppError` implements `serde::Serialize` (required for Tauri IPC)
- All fallible functions return `Result<T, AppError>`
- Use `?` operator for error propagation, avoid `.unwrap()` in production code

### Tauri Commands
```rust
#[tauri::command]
pub async fn command_name(state: State<'_, AppState>, param: String) -> Result<T, AppError> {
    let pool = get_db(&state).await?;
    service_function(&pool, &param).await
}
```
- Commands registered in `lib.rs` via `tauri::generate_handler![]`
- State accessed via `State<'_, AppState>` parameter
- DB pool lazy-initialized on first access via `get_db()` helper

### Architecture Layers
1. `commands.rs` — Tauri IPC handlers (thin, delegates to services)
2. `provider_service.rs` — Business logic
3. `github_oauth.rs` — GitHub API interactions
4. `auth_config.rs` — File system operations (auth.json read/write)
5. `database.rs` — SQLite pool initialization and migrations
6. `models.rs` — Data structures with Serialize/Deserialize/FromRow derives
7. `state.rs` — App state (`Arc<Mutex<Option<SqlitePool>>>`)

### Database
- SQLite via SQLx with compile-time unchecked queries (string SQL)
- Migrations handled inline in `database.rs` (CREATE TABLE IF NOT EXISTS)
- Schema evolution via `ALTER TABLE ... ADD COLUMN` with error silencing

### Dependencies
- `serde` + `serde_json` for serialization
- `reqwest` for HTTP (with JSON feature)
- `sqlx` for SQLite (runtime-tokio-rustls)
- `uuid` v4 for ID generation
- `chrono` for timestamps
- `dirs-next` for platform-specific directories
- `tauri-plugin-shell` for opening URLs

## CI/CD

- GitHub Actions workflow (`.github/workflows/release.yml`)
- Triggered on `v*` tag push or manual dispatch
- Builds on: macOS, Ubuntu 22.04, Windows
- Uses `tauri-apps/tauri-action@v0` for build + release asset upload
- Node.js 20, Rust stable

## Key Domain Knowledge

- The app manages GitHub OAuth tokens obtained via Device Flow
- Tokens are stored in a local SQLite database AND written to OpenCode's `auth.json`
- `auth.json` locations searched: `%LOCALAPPDATA%\opencode`, `~/.local/share/opencode`, `~/.config/opencode`
- After switching accounts, the app kills OpenCode's node processes to force token reload
- The GitHub Client ID (`Ov23li8tweQw6odWQebz`) is the official OpenCode Copilot client ID
- OAuth scope: `read:user user:email repo`
