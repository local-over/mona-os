//! Mona Notes — Authentic 3-Pane Apple Notes with Markdown & Interactive Checklists.

mod backend;

use backend::storage::NotesStore;
use slint::{ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn update_ui(ui: &NotesWindow, store: &NotesStore) {
    // 1. Notes List entries
    let mut entries = Vec::new();
    for (i, note) in store.notes.iter().enumerate() {
        let snippet = note.body.lines().next().unwrap_or("No additional text").to_string();
        let date_str = note.updated_at.format("%b %-d").to_string();
        entries.push(NoteEntry {
            id: note.id.clone().into(),
            title: note.title.clone().into(),
            snippet: snippet.into(),
            date_str: date_str.into(),
            is_selected: i == store.selected_note_index,
        });
    }
    ui.set_notes_list(ModelRc::new(VecModel::from(entries)));

    // 2. Active Note Content
    if let Some(note) = store.selected_note() {
        ui.set_active_title(note.title.clone().into());
        ui.set_active_body(note.body.clone().into());
        let date_full = note.updated_at.format("%B %-d, %Y at %I:%M %p").to_string();
        ui.set_active_date(date_full.into());

        // Checklists
        let mut checks = Vec::new();
        for (i, item) in note.checklists.iter().enumerate() {
            checks.push(ChecklistData {
                index: i as i32,
                text: item.text.clone().into(),
                is_completed: item.is_completed,
            });
        }
        ui.set_active_checklists(ModelRc::new(VecModel::from(checks)));
    }
}

fn main() -> anyhow::Result<()> {
    let ui = NotesWindow::new()?;
    let store = Rc::new(RefCell::new(NotesStore::new()));

    // Initial state populate
    update_ui(&ui, &store.borrow());

    // Select note callback
    {
        let store = store.clone();
        let ui_handle = ui.as_weak();
        ui.on_select_note(move |idx| {
            let mut st = store.borrow_mut();
            if (idx as usize) < st.notes.len() {
                st.selected_note_index = idx as usize;
            }
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Toggle checklist checkbox
    {
        let store = store.clone();
        let ui_handle = ui.as_weak();
        ui.on_toggle_checklist(move |idx| {
            let mut st = store.borrow_mut();
            st.toggle_checklist_item(idx as usize);
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // New Note
    {
        let store = store.clone();
        let ui_handle = ui.as_weak();
        ui.on_new_note_clicked(move || {
            let mut st = store.borrow_mut();
            st.create_new_note();
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Delete Note
    {
        let store = store.clone();
        let ui_handle = ui.as_weak();
        ui.on_delete_note_clicked(move || {
            let mut st = store.borrow_mut();
            st.delete_selected_note();
            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, &st);
            }
        });
    }

    // Window Close
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
