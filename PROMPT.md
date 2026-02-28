# 任务：创建 OpenCode GitHub Copilot 账号切换器

## 项目目标
创建一个使用 Tauri (Rust + React) 的桌面应用，允许用户管理多个 GitHub Copilot 账号，并通过 Device Authorization Flow (RFC 8628) 授权后，自动切换 OpenCode 的认证配置。

**支持平台**: Windows、Linux、macOS

## 核心功能
1. 添加 GitHub 账号 - 通过 OAuth Device Flow 授权
2. 列出所有已添加的账号 - 显示用户名、邮箱、头像
3. 切换账号 - 更新 OpenCode 的 auth.json 文件
4. 删除账号 - 从本地数据库移除

## 技术栈
- **前端**: React 18 + JSX + Tailwind CSS
- **后端**: Tauri 2.0 + Rust
- **数据库**: SQLite (sqlx 0.8)
- **HTTP**: reqwest 0.12
- **包管理**: npm / cargo

## 项目结构
```
opencode-github-switcher/
├── package.json
├── vite.config.ts
├── tailwind.config.js
├── index.html
├── src/
│   ├── App.jsx        # 主 UI 组件
│   ├── index.css      # Tailwind 样式
│   └── main.jsx       # React 入口
├── src-tauri/
│   ├── Cargo.toml     # Rust 依赖
│   ├── tauri.conf.json # Tauri 配置
│   ├── build.rs       # 构建脚本
│   └── src/
│       ├── main.rs          # 应用入口
│       ├── lib.rs           # 模块声明
│       ├── models.rs        # 数据结构
│       ├── error.rs         # 错误类型
│       ├── state.rs         # 应用状态
│       ├── database.rs      # SQLite 操作
│       ├── github_oauth.rs  # OAuth Device Flow
│       ├── auth_config.rs   # auth.json 读写
│       ├── provider_service.rs # 业务逻辑
│       └── commands.rs      # Tauri 命令处理器
└── README.md
```

## 关键实现细节

### 1. GitHub OAuth Device Flow 实现 (github_oauth.rs)
使用 GitHub OAuth Device Authorization Flow (https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow)

- Endpoint: `POST https://github.com/login/device/code`
- 请求参数: `client_id`, `scope` (read:user user:email repo)
- 响应包含: `device_code`, `user_code` (8位), `verification_uri`, `expires_in`, `interval`

- Endpoint: `POST https://github.com/login/oauth/access_token`
- 请求参数: `client_id`, `device_code`, `grant_type: urn:ietf:params:oauth:grant-type:device_code`
- 轮询直到授权成功或超时

- Endpoint: `GET https://api.github.com/user`
- 使用 Bearer token 获取用户信息

### 2. OpenCode auth.json 操作 (auth_config.rs)
平台路径差异：

| 平台 | auth.json 路径 |
|------|----------------|
| Windows | `C:\Users\<user>\AppData\Local\opencode\auth.json` |
| Linux | `~/.local/share/opencode/auth.json` |
| macOS | `~/Library/Application Support/opencode/auth.json` |

使用 `dirs-next` 库自动获取正确路径：
```rust
use dirs_next::data_local_dir;
let auth_dir = data_local_dir().ok_or(...)?.join("opencode");
```

```json
{
  "auth": {
    "github-copilot": {
      "type": "oauth",
      "refresh": "ghp_...",
      "access": "gho_...",  // 只更新这个字段
      "expires": 0
    }
  }
}
```
使用原子写入模式：写入临时文件 -> 重命名为目标文件

### 3. SQLite 数据库设计 (database.rs)
使用 `dirs-next` 获取平台相关的数据目录：

```rust
use dirs_next::data_local_dir;

let data_dir = data_local_dir().ok_or(...)?;
let app_dir = data_dir.join("opencode-github-switcher");
let db_path = app_dir.join("providers.db");
```

平台数据目录：
| 平台 | 数据目录 |
|------|----------|
| Windows | `C:\Users\<user>\AppData\Local\opencode-github-switcher\` |
| Linux | `~/.local/share/opencode-github-switcher/` |
| macOS | `~/Library/Application Support/opencode-github-switcher/` |

**Windows 路径处理**: 将反斜杠替换为正斜杠，添加 `?mode=rwc` 参数
```rust
let db_path_str = db_path.to_string_lossy().replace('\\', "/");
let db_url = format!("sqlite://{}?mode=rwc", db_path_str);
```

表结构:
```sql
CREATE TABLE github_providers (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  access_token TEXT NOT NULL,
  email TEXT,
  avatar_url TEXT,
  github_id INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  last_used_at INTEGER
)
```

### 4. Tauri IPC 命令 (commands.rs)
- `list_providers() -> Result<Vec<GitHubProvider>, String>`
- `add_github_account(client_id: String) -> Result<GitHubProvider, String>`
- `delete_provider(id: String) -> Result<(), String>`
- `switch_provider(id: String) -> Result<(), String>`

**注意**: 使用 `tokio::sync::Mutex` 而非 `std::sync::Mutex` 以支持 async

### 5. 前端 UI (App.jsx)
- 显示账号列表（头像、用户名、邮箱、token 预览）
- 添加、切换、删除按钮
- 错误提示（不要禁用按钮）
- 调用 Tauri API: `invoke('command_name', { args })`

### 6. 数据隔离设计
- 每个用户使用独立的 SQLite 数据库文件
- 每个用户操作自己的 GitHub 账号
- Access Token 仅存储在用户本地，不经过任何服务器
- 每个操作系统自动使用正确的数据目录

## 构建配置

### package.json
```json
{
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  }
}
```

### Cargo.toml 关键依赖
```toml
[dependencies]
tauri = "2.0"
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio"] }
reqwest = { version = "0.12", features = ["json"] }
dirs-next = "2"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

### tauri.conf.json
```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [{
      "title": "OpenCode GitHub Switcher",
      "width": 800,
      "height": 600
    }]
  },
  "bundle": {
    "targets": "all"
  }
}
```

## 跨平台构建

### Windows
```bash
npm run tauri build
```
输出：
- `src-tauri/target/release/opencode-github-switcher.exe` (约 13 MB)
- `src-tauri/target/release/bundle/msi/opencode-github-switcher_0.1.0_x64_en-US.msi` (约 5 MB)
- `src-tauri/target/release/bundle/nsis/opencode-github-switcher_0.1.0_x64-setup.exe` (约 3.5 MB)

### Linux
```bash
npm run tauri build
```
输出：
- `src-tauri/target/release/opencode-github-switcher` ELF 可执行文件
- `src-tauri/target/release/bundle/appimage/opencode-github-switcher_0.1.0_amd64.AppImage`
- `src-tauri/target/release/bundle/deb/opencode-github-switcher_0.1.0_amd64.deb`

### macOS
```bash
npm run tauri build
```
输出：
- `src-tauri/target/release/bundle/macos/opencode-github-switcher.app`
- `src-tauri/target/release/bundle/dmg/opencode-github-switcher_0.1.0_x64.dmg`

### 图标生成（跨平台）
```bash
# 将 PNG 转换为所有平台所需的图标格式
npx @tauri-apps/cli icon src-tauri/icons/icon.png
```
自动生成：
- Windows: icon.ico
- macOS: icon.icns
- Linux: 32x32.png, 128x128.png 等

## 关键注意事项

### 通用注意事项
1. **路径处理**: 使用 `dirs-next` 库自动处理不同平台路径
2. **Send Trait**: 使用 tokio::sync::Mutex 以支持 async await
3. **Type 推断**: 避免 collect 复杂类型，使用显式 for 循环
4. **错误处理**: 使用 thiserror 定义 AppError 枚举

### Windows 特定
1. **路径分隔符**: 将 `\` 替换为 `/` 用于 SQLite URL
2. **SQLite 参数**: 添加 `?mode=rwc` 确保读写创建权限
3. **图标**: 需要 .ico 格式 (Windows Resource)

### Linux 特定
1. **依赖**: 确保系统安装了 `libwebkit2gtk-4.0-dev`、`libssl-dev`、`libgtk-3-dev`
2. **X11/Wayland**: Tauri 自动支持两种显示服务器
3. **权限**: 可能需要 `chmod +x` 设置可执行权限

### macOS 特定
1. **代码签名**: 发布需要代码签名（开发模式不需要）
2. **沙箱**: App 已配置沙箱权限
3. **图标**: 需要 .icns 格式

### 通用跨平台问题
1. **CRLF vs LF**: 使用 Rust `std::fs` 自动处理换行符
2. **文件锁定**: SQLite 使用 `mode=rwc` 避免跨平台锁问题
3. **时间戳**: 使用 `chrono::Utc::now().timestamp()` (Unix 时间戳，跨平台一致)

## 开发流程

### 开发模式（所有平台）
```bash
npm run tauri dev
```
- 自动编译前端和后端
- 支持热重载

### 构建发布版本
```bash
# 构建当前平台
npm run tauri build

# 构建特定平台（需要对应平台环境）
# Windows 上只能构建 Windows
# Linux 上只能构建 Linux  
# macOS 上可以构建 macOS 和 iOS
```

## OAuth App 说明
开发者需要创建一个 GitHub OAuth App:
1. 已获得 Client ID (Ov23liC8F2cQhuOFCxLu)
2. 这个 Client ID 由应用使用，每个用户用自己的 GitHub 账号授权
3. 用户不需要创建自己的 OAuth App
4. 直接使用这个Client ID (Ov23liC8F2cQhuOFCxLu)来完成应用


### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### macOS
```bash
# Xcode Command Line Tools
xcode-select --install
```

## 验证步骤
1. `npm run tauri dev` - 开发模式测试
2. 添加 GitHub 账号流程完整
3. 切换账号后 auth.json 正确更新（检查对应平台路径）
4. 构建 Windows .exe / Linux AppImage / macOS .dmg
5. 应用可以独立运行，每个用户数据隔离

## 预期产出
- 完整的 Tauri 应用（支持三大平台）
- Windows: ~13 MB .exe 文件
- Linux: ~15 MB AppImage
- macOS: ~20 MB .app
- 用户无需配置即可使用
- 数据完全隔离在本地
- 跨平台路径自动适配
