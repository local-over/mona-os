#!/usr/bin/env bash
# ==============================================================================
# MONA OS — AUTOMATED INSTALLER & TURNKEY DEPLOYMENT SCRIPT
# Deploys barebones Hyprland WM + Greetd + Mona macOS Apps & Shell
# ==============================================================================

set -euo pipefail

echo "===================================================================="
echo "          MONA OS — MACOS-INSPIRED LINUX ENVIRONMENT INSTALLER      "
echo "===================================================================="

# 1. Detect Package Manager & Distro
if command -v pacman &>/dev/null; then
    DISTRO="arch"
    PKG_INSTALL="sudo pacman -S --needed --noconfirm"
    DEPS="hyprland hyprpaper greetd pipewire wireplumber brightnessctl networkmanager bluez grim slurp wl-clipboard"
elif command -v dnf &>/dev/null; then
    DISTRO="fedora"
    PKG_INSTALL="sudo dnf install -y"
    DEPS="hyprland hyprpaper greetd pipewire wireplumber brightnessctl NetworkManager bluez grim slurp wl-clipboard"
elif command -v apt-get &>/dev/null; then
    DISTRO="debian"
    PKG_INSTALL="sudo apt-get install -y"
    DEPS="hyprland greetd pipewire wireplumber brightnessctl network-manager bluez grim slurp wl-clipboard"
elif command -v apk &>/dev/null; then
    DISTRO="alpine"
    PKG_INSTALL="apk add"
    DEPS="hyprland greetd pipewire wireplumber networkmanager bluez grim slurp wl-clipboard rust cargo build-base mesa-dev wayland-dev libxkbcommon-dev fontconfig-dev"
else
    DISTRO="generic"
    echo "Notice: Non-standard distribution detected. Please ensure Hyprland and Greetd are installed."
fi

echo "==> Detected environment: $DISTRO"

# 2. Install Underlying Core Daemons
if [[ "$DISTRO" != "generic" ]]; then
    echo "==> Installing barebones window manager & system backends..."
    $PKG_INSTALL $DEPS || echo "Warning: Some packages could not be installed automatically."
fi

# 3. Build Mona Release Binaries
echo "==> Compiling Mona native apps and shell in Rust..."
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"
cargo build --release --workspace

# 4. Install Binaries to /usr/local/bin
echo "==> Installing Mona binaries to /usr/local/bin/..."
sudo install -m 755 target/release/mona-login /usr/local/bin/
sudo install -m 755 target/release/mona-shell /usr/local/bin/
sudo install -m 755 target/release/mona-calculator /usr/local/bin/
sudo install -m 755 target/release/mona-notes /usr/local/bin/
sudo install -m 755 target/release/mona-terminal /usr/local/bin/
sudo install -m 755 target/release/mona-settings /usr/local/bin/
sudo install -m 755 target/release/mona-finder /usr/local/bin/

# 5. Deploy Window Manager Dotfiles
echo "==> Deploying macOS-tuned Hyprland dotfiles..."
mkdir -p "$HOME/.config/hypr"
cp config/hypr/hyprland.conf "$HOME/.config/hypr/hyprland.conf"

# 6. Deploy Greetd Login Display Manager Config
echo "==> Configuring Greetd login manager..."
sudo mkdir -p /etc/greetd /etc/mona/hypr
sudo cp config/greetd/config.toml /etc/greetd/config.toml
sudo cp config/hypr/login.conf /etc/mona/hypr/login.conf

# 7. Register Wayland Session for System Display Managers
echo "==> Registering Mona Wayland session (/usr/share/wayland-sessions/mona.desktop)..."
sudo mkdir -p /usr/share/wayland-sessions
sudo tee /usr/share/wayland-sessions/mona.desktop > /dev/null << 'EOF'
[Desktop Entry]
Name=Mona (macOS)
Comment=Apple-Inspired Pure Rust Desktop Environment
Exec=Hyprland --config ~/.config/hypr/hyprland.conf
Type=Application
DesktopNames=Mona;Hyprland
EOF

echo "===================================================================="
echo "                 MONA OS INSTALLATION COMPLETE!                     "
echo "===================================================================="
echo "You can now:"
echo "  1. Test any app immediately:"
echo "     mona-calculator &"
echo "     mona-notes &"
echo "     mona-terminal &"
echo "     mona-settings &"
echo "     mona-finder &"
echo "     mona-shell &"
echo "  2. Or start the full desktop session:"
echo "     Hyprland --config ~/.config/hypr/hyprland.conf"
echo "===================================================================="
