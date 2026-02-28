# OpenCode GitHub Copilot Switcher

[English](./README.md) | [简体中文](./README_zh.md)

A seamless Tauri-based desktop application designed to manage, switch, and authenticate GitHub Copilot accounts for use within OpenCode. 

## Features

- **Multi-Account Management:** Easily add and switch between multiple GitHub accounts.
- **Native Device Flow Authentication:** Utilizes the official OpenCode GitHub Copilot Client ID (`Ov23li8tweQw6odWQebz`). This natively bypasses any server-side "unsupported model" restrictions, granting full access to premium models like GPT-4 and Claude 3.5.
- **Auto-Copy Device Code:** Automatically copies your authentication device code to the clipboard and provides an intuitive Toast notification.
- **Cross-Platform:** Available for Windows, macOS, and Linux.

## Why this app?

OpenCode enforces a strict backend whitelist for GitHub Copilot tokens. If you use a custom OAuth App Client ID, the OpenCode backend will recognize it as unofficial and block access to advanced models (e.g., throwing "The requested model is not supported" errors). 

This app uses the **extracted official native Client ID** to mint tokens, ensuring that you get the exact same premium AI capabilities as the official OpenCode extension, but with the added flexibility of managing multiple accounts easily.

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
