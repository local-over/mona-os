//! Mona System Settings — Authentic 28-Pane Apple System Settings.

mod backend;

use backend::system::SystemSettingsState;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn update_ui(ui: &SettingsWindow, state: &SystemSettingsState) {
    ui.set_dark_mode(state.dark_mode);
    ui.set_wifi_enabled(state.wifi_enabled);
    ui.set_wifi_ssid(state.wifi_ssid.clone().into());
    ui.set_bluetooth_enabled(state.bluetooth_enabled);
    ui.set_bluetooth_device(state.bluetooth_device.clone().into());
    ui.set_volume_val(state.volume_percent);
    ui.set_brightness_val(state.brightness_percent);
    ui.set_battery_percent(state.battery_percent);
    ui.set_active_cat(state.active_category);
}

fn main() -> anyhow::Result<()> {
    let ui = SettingsWindow::new()?;
    let state = Rc::new(RefCell::new(SystemSettingsState::new()));

    update_ui(&ui, &state.borrow());

    // Category select
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_select_category(move |cat| {
            let mut st = state.borrow_mut();
            st.active_category = cat;
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Toggle dark mode
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_dark_mode(move || {
            let mut st = state.borrow_mut();
            st.toggle_dark_mode();
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Toggle wifi
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_wifi(move || {
            let mut st = state.borrow_mut();
            st.toggle_wifi();
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Toggle bluetooth
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_bluetooth(move || {
            let mut st = state.borrow_mut();
            st.toggle_bluetooth();
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Volume changed
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_volume_changed(move |val| {
            let mut st = state.borrow_mut();
            st.set_volume(val as i32);
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_volume_val(st.volume_percent);
            }
        });
    }

    // Window close
    {
        let ui_handle = ui.as_weak();
        ui.on_close_window(move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.hide().unwrap();
            }
            std::process::exit(0);
        });
    }

    ui.run()?;
    Ok(())
}
