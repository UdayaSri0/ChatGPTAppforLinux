# ChatGPT Shell

A Linux-first companion app for launching ChatGPT in your browser with quick prompt, tray, and hotkey utilities.

## Features
- System tray actions: open ChatGPT, toggle quick prompt, capture screenshot, quit
- Global shortcuts for chat launch, quick prompt, and screenshots
- Snippet search with clipboard helpers
- Screenshot capture to `~/Pictures/ChatGPT-Shots`
- No embedded webview and no browser automation

## Dependencies
Install prerequisite packages on Linux Mint/Ubuntu:

```bash
sudo apt update
sudo apt install -y curl pkg-config libgtk-3-dev libayatana-appindicator3-dev \
  libwebkit2gtk-4.1-dev build-essential scrot wmctrl xdotool
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

## Configuration
- Runtime config file: `~/.local/share/com.example.chatgptshell/config.json`
- Runtime snippets file: `~/.local/share/com.example.chatgptshell/snippets.json`
- Default ChatGPT URL: `https://chatgpt.com`
- Hotkey conflicts use deterministic priority: `openChat` first, then `quickPrompt`, then `screenshot`. Conflicting later bindings are ignored with one warning message.

See [privacy.md](src/privacy.md) for terms.
