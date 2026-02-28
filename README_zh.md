# OpenCode GitHub Copilot Switcher

[English](./README.md) | [简体中文](./README_zh.md)

一款基于 Tauri 构建的跨平台桌面端工具，旨在为 OpenCode 用户提供便捷的 GitHub Copilot 账号管理、切换与设备授权流登录功能。

## 功能特性

- **多账号管理：** 轻松添加和在多个 GitHub 账号之间切换。
- **原生设备授权登录：** 深度集成提取自官方的 OpenCode GitHub Copilot Client ID (`Ov23li8tweQw6odWQebz`)。完美绕过服务端的模型限制，确保随时可用 GPT-4 和 Claude 3.5 等高级模型。
- **自动复制设备码：** 在进行 GitHub 授权时，自动将验证码复制到剪贴板，并弹出 Toast 提示用户，简化操作流程。
- **跨平台支持：** 提供 Windows、macOS 和 Linux 三个平台的安装包。

## 为什么需要这个工具？

OpenCode 在其后端对 GitHub Copilot 的 Token 进行了严格的白名单校验。如果你通过自己创建的 OAuth App 获取授权（使用自定义的 Client ID），OpenCode 的服务器会将其识别为非官方 Token，并在请求高级 AI 模型时拒绝服务（提示：“The requested model is not supported”）。

为了解决这个问题，本工具逆向提取了 OpenCode 原生的插件 Client ID 来进行 Token 生成，从而确保你能享受到和官方扩展完全一样的高级 AI 能力，并额外拥有多账号自由切换的便利。

## 安装指南

请前往项目的 [Releases](https://github.com/ltxzs/opencode-github-copilot-switcher/releases) 页面，下载适合您操作系统的最新版本。

- **Windows:** 下载 `.exe` 或 `.msi` 
- **macOS:** 下载 `.app` 或 `.dmg`
- **Linux:** 下载 `.AppImage` 或 `.deb`

## 本地开发开发

本项目使用了以下主要技术栈：
- **Tauri** (Rust)
- **React** (Vite)
- **Tailwind CSS**

### 环境要求

- [Node.js](https://nodejs.org/)
- [Rust](https://rustup.rs/)

### 启动步骤

1. 克隆本仓库：
   ```bash
   git clone https://github.com/ltxzs/opencode-github-copilot-switcher.git
   cd opencode-github-copilot-switcher/opencode-github-switcher
   ```

2. 安装依赖：
   ```bash
   npm install
   ```

3. 启动开发环境：
   ```bash
   npm run tauri dev
   ```

4. 构建生产版本：
   ```bash
   npm run tauri build
   ```

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。
