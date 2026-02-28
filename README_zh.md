# OpenCode GitHub Copilot Switcher

[English](./README.md) | [简体中文](./README_zh.md) | [日本語](./README_ja.md)

一款基于 Tauri 构建的跨平台桌面端工具，旨在为 OpenCode 用户提供便捷的 GitHub Copilot 账号管理、切换与设备授权流登录功能。

## 功能特性

- **多账号管理：** 轻松添加并在多个 GitHub 账号之间切换。
- **无缝热切换：** 支持在当前会话中直接进行热切换账号，**无需重启 OpenCode** 即可生效。
- **原生设备授权登录：** 深度集成提取自官方的 OpenCode GitHub Copilot Client ID，确保与官方扩展完全兼容。
- **自动复制设备码：** 在进行 GitHub 授权时，自动将验证码复制到剪贴板，并弹出 Toast 提示用户，简化操作流程。
- **跨平台支持：** 提供 Windows、macOS 和 Linux 三个平台的安装包。

## 为什么需要这个工具？

**解决原生切换繁琐的问题：** OpenCode 原生切换 GitHub Copilot 账号的步骤非常复杂且不够直观。而本项目提供了一个清晰的界面，让你不仅能方便地管理账号，还能实现**热切换**（无缝切换账号而无需重启 OpenCode 客户端），极大地提升了使用体验。

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