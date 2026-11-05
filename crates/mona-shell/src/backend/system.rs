//! System and process controller for Mona Shell.

use chrono::Local;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ShellState {
    pub active_app: String,
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
    pub dark_mode: bool,
    pub control_center_open: bool,
    pub apple_menu_open: bool,
    pub spotlight_open: bool,
    pub volume_percent: i32,
    pub brightness_percent: i32,
}

impl Default for ShellState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellState {
    pub fn new() -> Self {
        Self {
            active_app: "Finder".to_string(),
            wifi_enabled: true,
            bluetooth_enabled: true,
            dark_mode: true,
            control_center_open: false,
            apple_menu_open: false,
            spotlight_open: false,
            volume_percent: 65,
            brightness_percent: 85,
        }
    }

    pub fn formatted_time(&self) -> String {
        Local::now().format("%a %b %-d  %I:%M %p").to_string()
    }

    pub fn launch_app(&self, app_name: &str) {
        println!("==> Launching app: {}", app_name);
        let bin_name = match app_name {
            "Calculator" => "mona-calculator",
            "Notes" => "mona-notes",
            "Terminal" => "mona-terminal",
            "Settings" => "mona-settings",
            "Finder" => "mona-finder",
            _ => return,
        };

        // Try local target directory first, then PATH
        let local_bin = format!("./target/release/{}", bin_name);
        let cmd = if std::path::Path::new(&local_bin).exists() {
            local_bin
        } else {
            bin_name.to_string()
        };

        let _ = Command::new(cmd).spawn();
    }
}
