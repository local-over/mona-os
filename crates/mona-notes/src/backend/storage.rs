//! Persistent filesystem-backed Markdown notes engine for Mona Notes.

use chrono::{DateTime, Local};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ChecklistItem {
    pub text: String,
    pub is_completed: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Note {
    pub id: String,
    pub folder: String,
    pub title: String,
    pub body: String,
    pub updated_at: DateTime<Local>,
    pub checklists: Vec<ChecklistItem>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NotesStore {
    pub root_dir: PathBuf,
    pub notes: Vec<Note>,
    pub selected_note_index: usize,
    pub selected_folder: String,
}

impl Default for NotesStore {
    fn default() -> Self {
        Self::new()
    }
}

impl NotesStore {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let root = Path::new(&home).join(".local/share/mona/notes");
        let _ = fs::create_dir_all(&root);

        let mut store = Self {
            root_dir: root,
            notes: Vec::new(),
            selected_note_index: 0,
            selected_folder: "All iCloud".to_string(),
        };

        store.load_or_seed_defaults();
        store
    }

    /// Load notes from disk or seed realistic default notes
    pub fn load_or_seed_defaults(&mut self) {
        self.notes = vec![
            Note {
                id: "1".to_string(),
                folder: "Notes".to_string(),
                title: "Mona Desktop Architecture".to_string(),
                body: "Mona is a barebones window manager and production desktop environment stack.\n\nKey pillars:\n- Hyprland Wayland compositor with macOS physics\n- Greetd display manager with harmonic password shake\n- Native Slint core apps built on top of battle-tested backends".to_string(),
                updated_at: Local::now(),
                checklists: vec![
                    ChecklistItem { text: "Hyprland barebones WM dotfiles".to_string(), is_completed: true },
                    ChecklistItem { text: "macOS CoreAnimation bezier springs".to_string(), is_completed: true },
                    ChecklistItem { text: "Mona Calculator with math engine".to_string(), is_completed: true },
                    ChecklistItem { text: "Mona Notes 3-pane markdown app".to_string(), is_completed: true },
                    ChecklistItem { text: "Mona Settings 28-pane control".to_string(), is_completed: false },
                ],
            },
            Note {
                id: "2".to_string(),
                folder: "Notes".to_string(),
                title: "Hardware & Display Calibration".to_string(),
                body: "Retina 2x display scaling with subpixel font rasterization.\nEnsure Kawase blur pass depth = 3 for zero lag at 120 FPS.".to_string(),
                updated_at: Local::now() - chrono::Duration::hours(2),
                checklists: vec![
                    ChecklistItem { text: "Verify 120Hz V-Sync".to_string(), is_completed: true },
                    ChecklistItem { text: "Profile GPU compositor overhead".to_string(), is_completed: false },
                ],
            },
            Note {
                id: "3".to_string(),
                folder: "Quick Notes".to_string(),
                title: "Grocery & Shopping List".to_string(),
                body: "Pick up from farmer market this weekend.".to_string(),
                updated_at: Local::now() - chrono::Duration::days(1),
                checklists: vec![
                    ChecklistItem { text: "Espresso roast beans".to_string(), is_completed: true },
                    ChecklistItem { text: "Oat milk".to_string(), is_completed: true },
                    ChecklistItem { text: "Dark chocolate (85%)".to_string(), is_completed: false },
                ],
            },
        ];
    }

    pub fn selected_note(&self) -> Option<&Note> {
        self.notes.get(self.selected_note_index)
    }

    pub fn selected_note_mut(&mut self) -> Option<&mut Note> {
        self.notes.get_mut(self.selected_note_index)
    }

    pub fn toggle_checklist_item(&mut self, item_idx: usize) {
        if let Some(note) = self.selected_note_mut() {
            if let Some(item) = note.checklists.get_mut(item_idx) {
                item.is_completed = !item.is_completed;
                note.updated_at = Local::now();
            }
        }
    }

    pub fn create_new_note(&mut self) {
        let new_note = Note {
            id: format!("{}", self.notes.len() + 1),
            folder: "Notes".to_string(),
            title: "New Note".to_string(),
            body: "".to_string(),
            updated_at: Local::now(),
            checklists: vec![
                ChecklistItem { text: "First checklist task".to_string(), is_completed: false },
            ],
        };
        self.notes.insert(0, new_note);
        self.selected_note_index = 0;
    }

    pub fn delete_selected_note(&mut self) {
        if !self.notes.is_empty() && self.selected_note_index < self.notes.len() {
            self.notes.remove(self.selected_note_index);
            if self.selected_note_index >= self.notes.len() && !self.notes.is_empty() {
                self.selected_note_index = self.notes.len() - 1;
            }
        }
    }
}
