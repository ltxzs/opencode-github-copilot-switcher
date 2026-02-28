# OpenCode GitHub Copilot Switcher

[English](./README.md) | [简体中文](./README_zh.md) | [日本語](./README_ja.md)

A seamless Tauri-based desktop application designed to manage, switch, and authenticate GitHub Copilot accounts for use within OpenCode. 

## Features

- **Multi-Account Management:** Easily add and switch between multiple GitHub accounts.
- **Hot-Switching:** Seamlessly switch accounts directly in your current session without needing to restart OpenCode.
- **Native Device Flow Authentication:** Utilizes the official OpenCode GitHub Copilot Client ID to ensure full compatibility with the OpenCode extension.
- **Auto-Copy Device Code:** Automatically copies your authentication device code to the clipboard and provides an intuitive Toast notification.
- **Cross-Platform:** Available for Windows, macOS, and Linux.

## Why this app?

**Better UX than native OpenCode:** The native process for switching GitHub Copilot accounts in OpenCode is complex and unintuitive. This tool provides a clear, user-friendly interface to manage multiple accounts and supports **hot-switching**—meaning you can switch accounts on the fly without having to restart the OpenCode application.

## Installation

Download the latest release for your platform from the [Releases](https://github.com/ltxzs/opencode-github-copilot-switcher/releases) page.

- **Windows:** `.exe` or `.msi`
- **macOS:** `.app` or `.dmg`
- **Linux:** `.AppImage` or `.deb`

## Development

This project is built using:
- **Tauri** (Rust)
- **React** (Vite)
- **Tailwind CSS**

### Prerequisites

- [Node.js](https://nodejs.org/)
- [Rust](https://rustup.rs/)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/ltxzs/opencode-github-copilot-switcher.git
   cd opencode-github-copilot-switcher/opencode-github-switcher
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build for production:
   ```bash
   npm run tauri build
   ```

## License

MIT License. See [LICENSE](LICENSE) for more details.