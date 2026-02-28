# OpenCode GitHub Copilot Switcher

[English](./README.md) | [简体中文](./README_zh.md) | [日本語](./README_ja.md)

OpenCodeユーザー向けに、GitHub Copilotアカウントの管理、切り替え、およびデバイス認証ログインを簡単に行うための、Tauriベースのクロスプラットフォームデスクトップアプリケーションです。

## 主な機能

- **マルチアカウント管理：** 複数のGitHubアカウントを簡単に追加・管理し、切り替えることができます。
- **シームレスなホットスイッチ：** OpenCodeを**再起動することなく**、現在のセッションで直接アカウントを切り替える（ホットスイッチ）ことが可能です。
- **ネイティブデバイス認証：** OpenCode公式のGitHub Copilot Client ID (`Ov23li8tweQw6odWQebz`) を使用し、公式拡張機能との完全な互換性を確保しています。
- **デバイスコードの自動コピー：** 認証時にデバイスコードを自動的にクリップボードにコピーし、トースト通知でユーザーに知らせることで、操作を簡略化します。
- **クロスプラットフォーム対応：** Windows、macOS、Linux向けのインストーラーを提供しています。

## なぜこのツールが必要なのか？

**直感的でない標準の切り替え手順を解決：** OpenCodeでGitHub Copilotアカウントを切り替えるための標準の手順は非常に複雑で直感的ではありません。このツールを使用すれば、わかりやすいUIで複数のアカウントを管理できるだけでなく、OpenCodeクライアントを再起動することなくアカウントをシームレスに変更できる**ホットスイッチ**が可能になり、ユーザー体験が大幅に向上します。

## インストールガイド

[Releases](https://github.com/ltxzs/opencode-github-copilot-switcher/releases) ページにアクセスし、お使いのオペレーティングシステムに合った最新バージョンをダウンロードしてください。

- **Windows:** `.exe` または `.msi` をダウンロード
- **macOS:** `.app` または `.dmg` をダウンロード
- **Linux:** `.AppImage` または `.deb` をダウンロード

## 開発環境の構築

このプロジェクトは以下の技術スタックを使用して構築されています：
- **Tauri** (Rust)
- **React** (Vite)
- **Tailwind CSS**

### 前提条件

- [Node.js](https://nodejs.org/)
- [Rust](https://rustup.rs/)

### セットアップ手順

1. リポジトリをクローンします：
   ```bash
   git clone https://github.com/ltxzs/opencode-github-copilot-switcher.git
   cd opencode-github-copilot-switcher/opencode-github-switcher
   ```

2. 依存関係をインストールします：
   ```bash
   npm install
   ```

3. 開発モードで実行します：
   ```bash
   npm run tauri dev
   ```

4. 本番用にビルドします：
   ```bash
   npm run tauri build
   ```

## ライセンス

このプロジェクトは MIT ライセンスの下で提供されています。詳細については [LICENSE](LICENSE) ファイルを参照してください。