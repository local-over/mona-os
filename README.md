#  Mona OS

> **A Pure-Rust, macOS Sequoia-Inspired Desktop Environment & Display Manager for Linux**

Mona OS is an ultra-modern, lightweight Linux desktop environment built from scratch in **Rust** using the **Slint UI** framework and backed by the **Hyprland** Wayland compositor. It delivers a pixel-perfect macOS Sequoia experience on Linux with native performance, high-DPI scaling, frosted glass blur, and unified typography.

---

## ✨ Features

- **Translucent Menu Bar (`mona-shell`)**:
  - Global `` Apple menu (About Mona, System Settings, Lock Screen, Restart, Shutdown).
  - Dynamic active application title and menu controls.
  - Live status indicators: Network, Bluetooth, Audio, Battery, and Clock.
- **Frosted Glass Dock**:
  - macOS Sequoia-style floating translucent dock with subtle reflections and rounded pill borders.
  - Running app dot indicators and smooth hover expansion.
  - Pinned core apps: Finder, Terminal, Settings, Calculator, Notes, and Browser.
- **Control Center HUD**:
  - Quick toggles for Wi-Fi, Bluetooth, Dark/Light Mode, Do Not Disturb, and Night Shift.
  - Interactive sliders for Volume and Display Brightness.
- **Spotlight Search (`⌘ + Space`)**:
  - Instant fuzzy search across system applications, files, and quick math calculations.
- **Native Pure-Rust Core Applications**:
  - **Mona Terminal**: High-performance Wayland terminal with macOS Sequoia colors and tabs.
  - **Mona Finder**: Native file explorer with sidebar bookmarks, file grid/list views, and quick look preview.
  - **Mona Settings**: macOS-style categorized settings panel (Appearance, Displays, Network, Sound, About).
  - **Mona Calculator**: Clean macOS-styled calculator with standard and scientific functions.
  - **Mona Notes**: Quick note-taking app with markdown preview and persistence.
- **Hyprland Wayland Compositor Base**:
  - Hardware-accelerated GPU rendering (VirtIO-GPU, Mesa Gallium, Intel, AMD, NVIDIA).
  - True macOS window shadows (`color: rgba(00000055)`, `range: 35`, `render_power: 4`).
  - Smooth workspace transitions and floating window snapping.

---

## 🚀 Quick Start

### 1. Requirements

- **Rust toolchain** (1.75+ or newer: `cargo`, `rustc`)
- **Wayland compositor** (`hyprland`)
- **System development libraries**:
  - `pkgconf` / `pkg-config`
  - `fontconfig-dev` / `libfontconfig1-dev`
  - `wayland-dev` / `libwayland-dev`
  - `libxkbcommon-dev`
  - `mesa-dev` / `libegl1-mesa-dev`

### 2. Building from Source

```bash
# Clone the repository
git clone https://github.com/local-over/mona-os.git
cd mona-os

# Build the complete desktop workspace
cargo build --release --workspace

# Install binaries to system path
sudo cp target/release/mona-* /usr/local/bin/

# Install Hyprland dotfiles
mkdir -p ~/.config/hypr
cp config/hypr/hyprland.conf ~/.config/hypr/hyprland.conf
```

### 3. Launching Mona OS

```bash
# Start Mona desktop directly
Hyprland --config ~/.config/hypr/hyprland.conf
```

---

## 🌐 Running in Cloud / Google Cloud Shell (Web Browser VNC)

You can run Mona OS inside **Google Cloud Shell**, **GitHub Codespaces**, or any cloud Linux VM and access it directly inside your web browser via **noVNC**:

```bash
# In your Cloud Shell terminal:
git clone https://github.com/local-over/mona-os.git
cd mona-os
chmod +x run-cloud.sh
./run-cloud.sh
```

Then click the **Web Preview** button in Google Cloud Shell on port **6080** to view your running Mona OS desktop!

---

## ⌨️ Default Keybindings

| Shortcut | Action |
| :--- | :--- |
| <kbd>⌘</kbd> + <kbd>Space</kbd> | Open Spotlight Search HUD |
| <kbd>⌘</kbd> + <kbd>Return</kbd> | Launch Mona Terminal |
| <kbd>⌘</kbd> + <kbd>F</kbd> | Launch Mona Finder |
| <kbd>⌘</kbd> + <kbd>S</kbd> | Open System Settings |
| <kbd>⌘</kbd> + <kbd>K</kbd> | Open Calculator |
| <kbd>⌘</kbd> + <kbd>N</kbd> | Open Notes |
| <kbd>⌘</kbd> + <kbd>Q</kbd> | Close Active Window |
| <kbd>⌘</kbd> + <kbd>M</kbd> | Exit / Logout |

---

## 📄 License

MIT License © 2026 Hassan Elkady (LocalOver).
