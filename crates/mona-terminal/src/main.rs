//! Mona Terminal — Authentic macOS Terminal with Live PTY Execution.

mod backend;

use backend::pty::TerminalSession;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let ui = TerminalWindow::new()?;
    let session = Rc::new(RefCell::new(TerminalSession::new()));

    // Set initial text
    ui.set_terminal_output(session.borrow().formatted_terminal_text().into());
    ui.set_current_prompt(session.borrow().prompt_prefix().into());
    ui.set_window_title(format!("{} — -zsh — 80×24", session.borrow().username).into());

    // Command submission
    {
        let session = session.clone();
        let ui_handle = ui.as_weak();
        ui.on_command_submitted(move |cmd| {
            let mut sess = session.borrow_mut();
            sess.execute_command(cmd.as_str());
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_terminal_output(sess.formatted_terminal_text().into());
                ui.set_current_prompt(sess.prompt_prefix().into());
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
