#!/usr/bin/env bash

# === CONFIGURATION ===
CLI_NAME="git-find"
REPO="edenian-prince/rust-secrets"
INSTALL_DIR="$HOME/.local/bin"
HOOKS_TEMPLATE="$HOME/.git-template/hooks"
GLOBAL_HOOKS_PATH="$HOOKS_TEMPLATE" # using as global hooks path

# === FUNCTIONS ===
info() { echo -e "\033[1;34m[INFO]\033[0m $*"; }
ok() { echo -e "\033[1;32m[OK]\033[0m $*"; }
warn() { echo -e "\033[1;33m[WARN]\033[0m $*"; }
error() {
  echo -e "\033[1;31m[ERROR]\033[0m $*" >&2
  exit 1
}

# === STEP 1: Detect OS and download correct binary ===
info "Detecting platform..."
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$OS" in
linux*)
  BINARY="$CLI_NAME"
  ;;
darwin*)
  BINARY="$CLI_NAME-x86_64-apple-darwin"
  ;;
msys* | cygwin* | mingw*)
  BINARY="$CLI_NAME-x86_64-pc-windows-msvc.exe"
  ;;
*)
  error "Unsupported OS: $OS"
  ;;
esac

info "Fetching latest release URL..."
# Get the latest release JSON and extract the browser_download_url for our binary
RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" |
  grep '"browser_download_url":' |
  grep "$BINARY" |
  sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$RELEASE_URL" ]; then
  error "Could not find download URL for $BINARY in the latest release."
fi

info "Downloading $BINARY from latest release..."
mkdir -p "$INSTALL_DIR"
curl -L -o "$INSTALL_DIR/$CLI_NAME" "$RELEASE_URL" || error "Download failed"

chmod +x "$INSTALL_DIR/$CLI_NAME"
ok "Installed $CLI_NAME to $INSTALL_DIR"

# === STEP 2: Ensure CLI in PATH ===
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  warn "Adding $INSTALL_DIR to PATH (add this to your shell profile manually)"
  echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >>"$HOME/.bashrc"
fi

ok "✅ Installation complete!"
echo "$CLI_NAME CLI is now installed."
