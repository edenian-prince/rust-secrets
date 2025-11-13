#!/usr/bin/env bash
set -euo pipefail

# === CONFIGURATION ===
CLI_NAME="git-find"
RELEASE_URL="https://github.com/edenian-prince/rust-secrets/releases/download/v0.1.1"
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
  # BINARY="$CLI_NAME-x86_64-unknown-linux-gnu"
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

info "Downloading binary for $OS..."
mkdir -p "$INSTALL_DIR"
curl -L -o "$INSTALL_DIR/$CLI_NAME" "$RELEASE_URL/$BINARY" || error "Download failed"

# Windows binaries need .exe extension
if [[ "$OS" == *"mingw"* || "$OS" == *"msys"* || "$OS" == *"cygwin"* ]]; then
  mv "$INSTALL_DIR/$CLI_NAME" "$INSTALL_DIR/$CLI_NAME.exe"
fi

chmod +x "$INSTALL_DIR/$CLI_NAME"
ok "Installed $CLI_NAME to $INSTALL_DIR"

# === STEP 2: Ensure CLI in PATH ===
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  warn "Adding $INSTALL_DIR to PATH (add this to your shell profile manually)"
  echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >>"$HOME/.bashrc"
fi

# === STEP 3: Set up global hooks ===
info "Setting up global Git hooks..."
mkdir -p "$HOOKS_TEMPLATE"

# Download or copy your pre-commit hook
# (replace this with your repo's pre-commit logic if you want to bundle it)
cat >"$HOOKS_TEMPLATE/pre-commit" <<'EOF'
#!/usr/bin/env bash
# Global pre-commit hook using git-find
if ! command -v git-find &>/dev/null; then
  echo "git-find not found in PATH, skipping secret scan."
  exit 0
fi

echo "Running git-find hook..."
git find hook
EOF

chmod +x "$HOOKS_TEMPLATE/pre-commit"

# Configure Git globally to use it
git config --global core.hooksPath "$GLOBAL_HOOKS_PATH"
ok "Configured global Git hooks path → $GLOBAL_HOOKS_PATH"

# === STEP 4: Confirm setup ===
info "Verifying setup..."
git config --global --get core.hooksPath
"$INSTALL_DIR/$CLI_NAME" --version || echo "(CLI verification skipped)"

ok "✅ Installation complete!"
echo "Your global pre-commit hook is now active for all repos."
