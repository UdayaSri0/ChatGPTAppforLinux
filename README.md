# ChatGPT Native

A Linux desktop app that opens ChatGPT directly inside a native Tauri window.

## Features
- Native desktop window that loads `https://chatgpt.com`
- No API key needed; sign in with your ChatGPT account in the app
- System tray actions: focus window, toggle window, capture screenshot, quit
- Global shortcuts for focus/toggle/screenshot actions
- Screenshot capture to `~/Pictures/ChatGPT-Shots`
- No browser automation; the app only hosts the official ChatGPT web app

## Dependencies
Install prerequisite packages on Linux Mint/Ubuntu:

```bash
sudo apt update
sudo apt install -y curl pkg-config libgtk-3-dev libayatana-appindicator3-dev \
  libwebkit2gtk-4.0-dev libjavascriptcoregtk-4.0-dev libsoup2.4-dev \
  build-essential scrot wmctrl xdotool
```

Install Rust and Node.js if not already:

```bash
curl https://sh.rustup.rs -sSf | sh
sudo apt install -y nodejs npm
```

## Development

```bash
npm install
npm run tauri:dev
```

## Build

```bash
npm run tauri:build
```

## Install and Use
- Build the installer with `npm run tauri:build`
- Install the generated package from `src-tauri/target/release/bundle/`
- Open the app and log in to your ChatGPT account

## Runtime Configuration
- Runtime config file: `~/.local/share/com.example.chatgptnative/config.json`
- Default ChatGPT URL: `https://chatgpt.com`
- Hotkey conflicts use deterministic priority: `openChat` first, then `quickPrompt` (toggle window), then `screenshot`. Conflicting later bindings are ignored with one warning message.

See [privacy.md](src/privacy.md) for terms.
