//! Real filesystem controller for Mona Finder.

use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size_formatted: String,
}

#[derive(Debug, Clone)]
pub struct FinderState {
    pub current_path: PathBuf,
    pub history: Vec<PathBuf>,
    pub history_index: usize,
    pub items: Vec<FileEntry>,
}

impl Default for FinderState {
    fn default() -> Self {
        Self::new()
    }
}

impl FinderState {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let initial_path = PathBuf::from(home);

        let mut state = Self {
            current_path: initial_path.clone(),
            history: vec![initial_path],
            history_index: 0,
            items: Vec::new(),
        };

        state.refresh();
        state
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        if path.is_dir() {
            self.history.truncate(self.history_index + 1);
            self.history.push(path.clone());
            self.history_index += 1;
            self.current_path = path;
            self.refresh();
        }
    }

    pub fn go_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
            self.refresh();
        }
    }

    pub fn go_forward(&mut self) {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
            self.refresh();
        }
    }

    pub fn refresh(&mut self) {
        self.items.clear();

        if let Ok(entries) = fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue; // Skip hidden dotfiles
                }

                let path = entry.path();
                let is_dir = path.is_dir();
                let size_formatted = if is_dir {
                    "--".to_string()
                } else if let Ok(meta) = entry.metadata() {
                    let bytes = meta.len();
                    if bytes < 1024 {
                        format!("{} B", bytes)
                    } else if bytes < 1024 * 1024 {
                        format!("{:.1} KB", bytes as f64 / 1024.0)
                    } else {
                        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
                    }
                } else {
                    "--".to_string()
                };

                self.items.push(FileEntry {
                    name,
                    path,
                    is_dir,
                    size_formatted,
                });
            }
        }

        // Sort folders first, then files alphabetically
        self.items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
    }
}
