#!/usr/bin/env bash

# config
CLI_NAME="git-find"
REPO="edenian-prince/rust-secrets"
INSTALL_DIR="$HOME/.local/bin"

info() { echo -e "\033[1;34m[INFO]\033[0m $*"; }
ok() { echo -e "\033[1;32m[OK]\033[0m $*"; }
warn() { echo -e "\033[1;33m[WARN]\033[0m $*"; }
error() { echo -e "\033[1;31m[ERROR]\033[0m $*" >&2; exit 1; }

# === Detect OS ===
info "Detecting platform..."
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$OS" in
  linux*)
    ASSET="git-find-linux-musl-x86_64"
    ;;
  darwin*)
    ASSET="git-find-macos-x86_64"   # Only works if you build this later
    ;;
  msys* | cygwin* | mingw*)
    ASSET="git-find-windows-x86_64.exe"
    ;;
  *)
    error "Unsupported OS: $OS"
    ;;
esac

info "Fetching latest release asset..."
RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | grep '"browser_download_url":' \
  | grep "$ASSET" \
  | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$RELEASE_URL" ]; then
  error "Could not find asset matching: $ASSET"
fi

info "Downloading $ASSET ..."
mkdir -p "$INSTALL_DIR"

curl -L -o "$INSTALL_DIR/$CLI_NAME" "$RELEASE_URL" || error "Download failed"

chmod +x "$INSTALL_DIR/$CLI_NAME"
ok "Installed $CLI_NAME to $INSTALL_DIR"

# === PATH check ===
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  warn "Adding $INSTALL_DIR to PATH"
  echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.bashrc"
fi

ok "✅ Installation complete!"
