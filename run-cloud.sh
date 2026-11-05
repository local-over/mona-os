#!/bin/bash
set -e

echo "=========================================================="
echo "           MONA OS — CLOUD / WEB VNC RUNNER              "
echo "=========================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "==> 1. Detecting package manager and installing dependencies..."
if command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get install -y git curl wget build-essential pkg-config \
        libfontconfig1-dev libwayland-dev libxkbcommon-dev libegl1-mesa-dev \
        xvfb x11vnc novnc websockify openbox
elif command -v apk >/dev/null 2>&1; then
    sudo apk update
    sudo apk add git curl wget build-base pkgconf fontconfig-dev \
        wayland-dev libxkbcommon-dev mesa-dev xvfb x11vnc novnc websockify openbox
elif command -v dnf >/dev/null 2>&1; then
    sudo dnf install -y git curl wget gcc gcc-c++ pkgconf fontconfig-devel \
        wayland-devel libxkbcommon-devel mesa-libEGL-devel xorg-x11-server-Xvfb x11vnc novnc openbox
fi

# Ensure Rust is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "==> Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "==> 2. Compiling Mona Desktop Shell & Core Apps..."
cargo build --release -p mona-shell
mkdir -p "$HOME/.local/bin"
cp target/release/mona-shell "$HOME/.local/bin/"
export PATH="$HOME/.local/bin:$PATH"

# Background compilation for the remaining apps
(cargo build --release --workspace && cp target/release/mona-* "$HOME/.local/bin/") &

echo "==> 3. Starting virtual display (Xvfb) on :99..."
Xvfb :99 -screen 0 1280x800x24 -listen tcp -ac &
export DISPLAY=:99
sleep 1

# Start basic window manager for floating dialogs
openbox &

echo "==> 4. Starting Mona Shell on virtual display..."
mona-shell &

echo "==> 5. Starting VNC Server on display :99..."
x11vnc -display :99 -nopw -listen 127.0.0.1 -xkb -ncache 10 -forever -shared &
sleep 1

echo "==> 6. Starting noVNC Web Browser gateway on port 6080..."
websockify --web /usr/share/novnc 6080 localhost:5900 &

echo ""
echo "=========================================================="
echo "           MONA OS IS RUNNING IN THE CLOUD!              "
echo "=========================================================="
echo "In Google Cloud Shell: Click 'Web Preview' -> 'Preview on port 6080'"
echo "In GitHub Codespaces: Open the forwarded port 6080"
echo "Direct URL: http://localhost:6080/vnc.html"
echo "=========================================================="

wait
