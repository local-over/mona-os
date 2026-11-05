//! Mona Calculator — Authentic Apple Calculator with Rust Math Engine.

mod backend;

use backend::engine::{BinaryOp, CalculatorEngine};
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let ui = CalculatorWindow::new()?;
    let engine = Rc::new(RefCell::new(CalculatorEngine::new()));

    // Digit click
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_digit_clicked(move |digit| {
            if let Some(ch) = digit.chars().next() {
                let mut eng = engine.borrow_mut();
                eng.input_digit(ch);
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_display_text(eng.display.clone().into());
                }
            }
        });
    }

    // Decimal click
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_decimal_clicked(move || {
            let mut eng = engine.borrow_mut();
            eng.input_decimal();
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(eng.display.clone().into());
            }
        });
    }

    // Binary operations (+, -, *, /)
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_op_clicked(move |op_str| {
            let mut eng = engine.borrow_mut();
            let op = match op_str.as_str() {
                "+" => BinaryOp::Add,
                "−" | "-" => BinaryOp::Subtract,
                "×" | "*" => BinaryOp::Multiply,
                "÷" | "/" => BinaryOp::Divide,
                _ => return,
            };
            eng.set_binary_op(op);
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(eng.display.clone().into());
                ui.set_active_op(op_str);
            }
        });
    }

    // Equals (=)
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_equals_clicked(move || {
            let mut eng = engine.borrow_mut();
            eng.evaluate();
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(eng.display.clone().into());
                ui.set_active_op("".into());
            }
        });
    }

    // Clear (AC / C)
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_clear_clicked(move || {
            let mut eng = engine.borrow_mut();
            eng.all_clear();
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text("0".into());
                ui.set_active_op("".into());
            }
        });
    }

    // Toggle Sign (+/-)
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_sign_clicked(move || {
            let mut eng = engine.borrow_mut();
            eng.toggle_sign();
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(eng.display.clone().into());
            }
        });
    }

    // Percentage (%)
    {
        let engine = engine.clone();
        let ui_handle = ui.as_weak();
        ui.on_percent_clicked(move || {
            let mut eng = engine.borrow_mut();
            eng.percentage();
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(eng.display.clone().into());
            }
        });
    }

    // Window controls
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
