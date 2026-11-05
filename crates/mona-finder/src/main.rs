//! Mona Finder — Authentic macOS File Manager with Real Filesystem Integration.

mod backend;

use backend::fs::FinderState;
use slint::{ModelRc, VecModel};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

slint::include_modules!();

fn update_ui(ui: &FinderWindow, state: &FinderState) {
    ui.set_current_path_str(state.current_path.to_string_lossy().to_string().into());

    let mut model_items = Vec::new();
    for it in &state.items {
        model_items.push(FinderItem {
            name: it.name.clone().into(),
            is_dir: it.is_dir,
            size_str: it.size_formatted.clone().into(),
        });
    }
    ui.set_items(ModelRc::new(VecModel::from(model_items)));
    ui.set_status_text(format!("{} items", state.items.len()).into());
}

fn main() -> anyhow::Result<()> {
    let ui = FinderWindow::new()?;
    let state = Rc::new(RefCell::new(FinderState::new()));

    update_ui(&ui, &state.borrow());

    // Item clicked (if directory, navigate into it)
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_item_clicked(move |idx| {
            let mut st = state.borrow_mut();
            if let Some(item) = st.items.get(idx as usize) {
                if item.is_dir {
                    let target_path = item.path.clone();
                    st.navigate_to(target_path);
                    if let Some(ui) = ui_handle.upgrade() {
                        update_ui(&ui, &st);
                    }
                }
            }
        });
    }

    // Back button
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_back_clicked(move || {
            let mut st = state.borrow_mut();
            st.go_back();
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Forward button
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_forward_clicked(move || {
            let mut st = state.borrow_mut();
            st.go_forward();
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Sidebar shortcuts
    {
        let state = state.clone();
        let ui_handle = ui.as_weak();
        ui.on_sidebar_clicked(move |shortcut| {
            let mut st = state.borrow_mut();
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let target = match shortcut.as_str() {
                "home" => PathBuf::from(&home),
                "desktop" => PathBuf::from(&home).join("Desktop"),
                "documents" => PathBuf::from(&home).join("Documents"),
                "downloads" => PathBuf::from(&home).join("Downloads"),
                _ => PathBuf::from(&home),
            };
            st.navigate_to(target);
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
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
