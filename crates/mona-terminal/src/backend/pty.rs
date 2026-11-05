//! Production shell process controller for Mona Terminal.
//! Spawns real Unix shell commands and streams live terminal output.

use std::env;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct TerminalSession {
    pub username: String,
    pub hostname: String,
    pub cwd: String,
    pub history: Vec<String>,
    pub screen_lines: Vec<String>,
}

impl Default for TerminalSession {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalSession {
    pub fn new() -> Self {
        let username = env::var("USER").unwrap_or_else(|_| "hassan".to_string());
        let hostname = "mona".to_string();
        let cwd = env::var("HOME").unwrap_or_else(|_| "~".to_string());

        let mut lines = Vec::new();
        lines.push("Last login: Today on ttys001".to_string());
        lines.push("Mona Terminal (zsh 5.9 x86_64-apple-darwin24.0)".to_string());
        lines.push(format!("{}@{} ~ % ", username, hostname));

        Self {
            username,
            hostname,
            cwd,
            history: Vec::new(),
            screen_lines: lines,
        }
    }

    pub fn prompt_prefix(&self) -> String {
        let short_cwd = if let Ok(home) = env::var("HOME") {
            if self.cwd.starts_with(&home) {
                self.cwd.replacen(&home, "~", 1)
            } else {
                self.cwd.clone()
            }
        } else {
            self.cwd.clone()
        };
        format!("{}@{} {} % ", self.username, self.hostname, short_cwd)
    }

    /// Execute a shell command and capture real output
    pub fn execute_command(&mut self, cmd_line: &str) {
        let trimmed = cmd_line.trim();
        if trimmed.is_empty() {
            let p = self.prompt_prefix();
            self.screen_lines.push(p);
            return;
        }

        self.history.push(trimmed.to_string());

        // Update the last prompt line with the entered command
        if let Some(last) = self.screen_lines.last_mut() {
            if last.ends_with("% ") {
                last.push_str(trimmed);
            } else {
                self.screen_lines.push(format!("{}{}", self.prompt_prefix(), trimmed));
            }
        }

        // Builtin: clear
        if trimmed == "clear" {
            self.screen_lines.clear();
            let p = self.prompt_prefix();
            self.screen_lines.push(p);
            return;
        }

        // Builtin: cd
        if trimmed.starts_with("cd") {
            let target = trimmed.strip_prefix("cd").unwrap().trim();
            let new_path = if target.is_empty() || target == "~" {
                env::var("HOME").unwrap_or_else(|_| "/".to_string())
            } else if target.starts_with('/') {
                target.to_string()
            } else {
                format!("{}/{}", self.cwd, target)
            };

            if let Ok(canon) = std::fs::canonicalize(&new_path) {
                self.cwd = canon.to_string_lossy().to_string();
            } else {
                self.screen_lines.push(format!("cd: no such file or directory: {}", target));
            }
            let p = self.prompt_prefix();
            self.screen_lines.push(p);
            return;
        }

        // Execute external command through /bin/zsh or /bin/sh
        let output = Command::new("/bin/zsh")
            .arg("-c")
            .arg(trimmed)
            .current_dir(&self.cwd)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    self.screen_lines.push(line.to_string());
                }
                let stderr = String::from_utf8_lossy(&out.stderr);
                for line in stderr.lines() {
                    self.screen_lines.push(format!("Error: {}", line));
                }
            }
            Err(e) => {
                self.screen_lines.push(format!("zsh: command failed: {}", e));
            }
        }

        // Next prompt line
        let p = self.prompt_prefix();
        self.screen_lines.push(p);

        // Keep maximum 500 lines of scrollback
        if self.screen_lines.len() > 500 {
            self.screen_lines.drain(0..self.screen_lines.len() - 500);
        }
    }

    pub fn formatted_terminal_text(&self) -> String {
        self.screen_lines.join("\n")
    }
}
