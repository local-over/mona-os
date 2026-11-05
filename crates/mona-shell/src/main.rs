//! Mona Shell — 24pt Translucent Menu Bar, Control Center, Spotlight HUD & Frosted Glass Dock.

mod backend;

use backend::system::ShellState;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn update_ui(ui: &ShellWindow, state: &ShellState) {
    ui.set_active_app_title(state.active_app.clone().into());
    ui.set_time_str(state.formatted_time().into());
    ui.set_apple_menu_open(state.apple_menu_open);
    ui.set_control_center_open(state.control_center_open);
    ui.set_spotlight_open(state.spotlight_open);
    ui.set_wifi_enabled(state.wifi_enabled);
    ui.set_bluetooth_enabled(state.bluetooth_enabled);
    ui.set_dark_mode(state.dark_mode);
    ui.set_volume_val(state.volume_percent);
    ui.set_brightness_val(state.brightness_percent);
}

fn main() -> anyhow::Result<()> {
    let ui = ShellWindow::new()?;
    let state = Rc::new(RefCell::new(ShellState::new()));

    update_ui(&ui, &state.borrow());

    // Toggle Apple Menu
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_apple_menu(move || {
            let mut st = state.borrow_mut();
            st.apple_menu_open = !st.apple_menu_open;
            st.control_center_open = false;
            st.spotlight_open = false;
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Toggle Control Center
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_control_center(move || {
            let mut st = state.borrow_mut();
            st.control_center_open = !st.control_center_open;
            st.apple_menu_open = false;
            st.spotlight_open = false;
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Toggle Spotlight
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_spotlight(move || {
            let mut st = state.borrow_mut();
            st.spotlight_open = !st.spotlight_open;
            st.apple_menu_open = false;
            st.control_center_open = false;
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Toggle Dark Mode
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_dark_mode(move || {
            let mut st = state.borrow_mut();
            st.dark_mode = !st.dark_mode;
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Launch Application from Dock
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_launch_app(move |app_name| {
            let mut st = state.borrow_mut();
            st.active_app = app_name.to_string();
            st.launch_app(app_name.as_str());
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    ui.run()?;
    Ok(())
}
