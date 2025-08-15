# ChatGPT Shell

A small companion shell for launching the official ChatGPT site on Linux Mint (Ubuntu based) with handy local utilities.

## Features
- System tray with quick actions (open ChatGPT, quick prompt, screenshot, quit)
- Global shortcuts
- Snippet search and clipboard helpers
- Screenshot capture to `~/Pictures/ChatGPT-Shots`
- No webview or automation; opens your default browser in app mode

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
npm run dev
```

## Build
```bash
npm run tauri build
```

## Create a Debian package
Build assets and produce an installable `.deb`:

```bash
# compile TypeScript
npm run build

# compile the Rust backend
cargo build --release

# install cargo-deb if needed
cargo install cargo-deb

# package without rebuilding
cargo deb --no-build

# install the generated package
sudo dpkg -i target/debian/chatgpt-shell_*.deb
```

The `.deb` package will appear under `target/debian`.

See [privacy.md](src/privacy.md) for terms.
