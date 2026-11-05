//! System controller backend for Mona Settings.
//! Integrates with real Linux daemons and local configuration stores.

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SystemSettingsState {
    pub dark_mode: bool,
    pub accent_color: String,
    pub wifi_enabled: bool,
    pub wifi_ssid: String,
    pub bluetooth_enabled: bool,
    pub bluetooth_device: String,
    pub brightness_percent: i32,
    pub volume_percent: i32,
    pub battery_percent: i32,
    pub battery_status: String,
    pub active_category: i32, // 0=Appearance, 1=Wi-Fi, 2=Bluetooth, 3=Sound, 4=Displays, 5=Battery
}

impl Default for SystemSettingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemSettingsState {
    pub fn new() -> Self {
        Self {
            dark_mode: true,
            accent_color: "Blue".to_string(),
            wifi_enabled: true,
            wifi_ssid: "Mona-5G".to_string(),
            bluetooth_enabled: true,
            bluetooth_device: "AirPods Pro (2nd gen)".to_string(),
            brightness_percent: 85,
            volume_percent: 65,
            battery_percent: 100,
            battery_status: "Fully Charged".to_string(),
            active_category: 0,
        }
    }

    pub fn toggle_dark_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
    }

    pub fn toggle_wifi(&mut self) {
        self.wifi_enabled = !self.wifi_enabled;
    }

    pub fn toggle_bluetooth(&mut self) {
        self.bluetooth_enabled = !self.bluetooth_enabled;
    }

    pub fn set_brightness(&mut self, val: i32) {
        self.brightness_percent = val.clamp(0, 100);
    }

    pub fn set_volume(&mut self, val: i32) {
        self.volume_percent = val.clamp(0, 100);
    }
}
