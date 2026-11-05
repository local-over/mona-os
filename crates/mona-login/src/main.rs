//! Mona Login Display Manager — Authentic macOS Login Screen with Shake Physics.

mod backend;

use backend::pam::LoginEngine;
use chrono::Local;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let ui = LoginWindow::new()?;
    let engine = Rc::new(RefCell::new(LoginEngine::new()));

    // Set initial time & user
    let now = Local::now();
    ui.set_time_str(now.format("%I:%M %p").to_string().into());
    ui.set_date_str(now.format("%A, %B %-d").to_string().into());
    ui.set_username(engine.borrow().real_name.clone().into());

    // Password submission with harmonic shake on failure
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_password_submitted(move |pwd| {
            let mut eng = engine.borrow_mut();
            let success = eng.submit_password(pwd.as_str());

            if success {
                println!("==> Login successful! Transitioning to Mona desktop session...");
                if let Some(ui) = ui_handle.upgrade() {
                    ui.hide().unwrap();
                }
                std::process::exit(0);
            } else {
                // Harmonic damped shake
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_is_failed(true);
                    ui.set_shake_offset(-16.0);
                }
            }
        });
    }

    // Power actions
    ui.on_sleep_clicked(|| {
        println!("==> System sleep requested.");
    });

    ui.on_restart_clicked(|| {
        println!("==> System restart requested.");
    });

    ui.on_shutdown_clicked(|| {
        println!("==> System shutdown requested.");
        std::process::exit(0);
    });

    ui.run()?;
    Ok(())
}
