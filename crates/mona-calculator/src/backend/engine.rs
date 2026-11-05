//! Production math calculation engine for Mona Calculator.
//! Supports Standard, Scientific, and Programmer bitboard evaluation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    BitAnd,
    BitOr,
    BitXor,
    Lsh,
    Rsh,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CalculatorEngine {
    pub display: String,
    pub accumulator: f64,
    pub pending_op: Option<BinaryOp>,
    pub clear_on_next_input: bool,
    pub memory: f64,
    pub is_error: bool,
}

impl Default for CalculatorEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CalculatorEngine {
    pub fn new() -> Self {
        Self {
            display: "0".to_string(),
            accumulator: 0.0,
            pending_op: None,
            clear_on_next_input: false,
            memory: 0.0,
            is_error: false,
        }
    }

    /// Reset everything to zero
    pub fn all_clear(&mut self) {
        self.display = "0".to_string();
        self.accumulator = 0.0;
        self.pending_op = None;
        self.clear_on_next_input = false;
        self.is_error = false;
    }

    /// Clear current entry only
    pub fn clear_entry(&mut self) {
        self.display = "0".to_string();
        self.clear_on_next_input = false;
    }

    /// Input numeric digit ('0'..='9')
    pub fn input_digit(&mut self, digit: char) {
        if self.is_error || self.clear_on_next_input || self.display == "0" {
            self.display = digit.to_string();
            self.clear_on_next_input = false;
            self.is_error = false;
        } else if self.display.len() < 14 {
            self.display.push(digit);
        }
    }

    /// Input decimal point ('.')
    pub fn input_decimal(&mut self) {
        if self.is_error || self.clear_on_next_input {
            self.display = "0.".to_string();
            self.clear_on_next_input = false;
            self.is_error = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    /// Invert sign (+/-)
    pub fn toggle_sign(&mut self) {
        if self.display == "0" || self.is_error {
            return;
        }
        if self.display.starts_with('-') {
            self.display.remove(0);
        } else {
            self.display.insert(0, '-');
        }
    }

    /// Percentage (%): divides current value by 100
    pub fn percentage(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            let res = val / 100.0;
            self.display = Self::format_number(res);
        }
    }

    /// Set binary operation (+, -, *, /)
    pub fn set_binary_op(&mut self, op: BinaryOp) {
        if self.pending_op.is_some() && !self.clear_on_next_input {
            self.evaluate();
        } else {
            self.accumulator = self.display.parse::<f64>().unwrap_or(0.0);
        }
        self.pending_op = Some(op);
        self.clear_on_next_input = true;
    }

    /// Evaluate current calculation (=)
    pub fn evaluate(&mut self) {
        if let Some(op) = self.pending_op {
            let current = self.display.parse::<f64>().unwrap_or(0.0);
            let result = match op {
                BinaryOp::Add => self.accumulator + current,
                BinaryOp::Subtract => self.accumulator - current,
                BinaryOp::Multiply => self.accumulator * current,
                BinaryOp::Divide => {
                    if current.abs() < f64::EPSILON {
                        self.is_error = true;
                        self.display = "Not a Number".to_string();
                        self.pending_op = None;
                        self.clear_on_next_input = true;
                        return;
                    }
                    self.accumulator / current
                }
                BinaryOp::BitAnd => ((self.accumulator as i64) & (current as i64)) as f64,
                BinaryOp::BitOr => ((self.accumulator as i64) | (current as i64)) as f64,
                BinaryOp::BitXor => ((self.accumulator as i64) ^ (current as i64)) as f64,
                BinaryOp::Lsh => ((self.accumulator as i64) << (current as u32)) as f64,
                BinaryOp::Rsh => ((self.accumulator as i64) >> (current as u32)) as f64,
            };

            self.display = Self::format_number(result);
            self.accumulator = result;
            self.pending_op = None;
            self.clear_on_next_input = true;
        }
    }

    /// Scientific Functions
    pub fn square_root(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            if val < 0.0 {
                self.is_error = true;
                self.display = "Error".to_string();
            } else {
                self.display = Self::format_number(val.sqrt());
                self.clear_on_next_input = true;
            }
        }
    }

    pub fn square(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            self.display = Self::format_number(val * val);
            self.clear_on_next_input = true;
        }
    }

    pub fn inverse(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            if val.abs() < f64::EPSILON {
                self.is_error = true;
                self.display = "Not a Number".to_string();
            } else {
                self.display = Self::format_number(1.0 / val);
            }
            self.clear_on_next_input = true;
        }
    }

    pub fn sin(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            self.display = Self::format_number(val.to_radians().sin());
            self.clear_on_next_input = true;
        }
    }

    pub fn cos(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            self.display = Self::format_number(val.to_radians().cos());
            self.clear_on_next_input = true;
        }
    }

    pub fn tan(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            self.display = Self::format_number(val.to_radians().tan());
            self.clear_on_next_input = true;
        }
    }

    /// Memory Registers
    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
    }

    pub fn memory_recall(&mut self) {
        self.display = Self::format_number(self.memory);
        self.clear_on_next_input = true;
    }

    pub fn memory_add(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            self.memory += val;
            self.clear_on_next_input = true;
        }
    }

    pub fn memory_subtract(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            self.memory -= val;
            self.clear_on_next_input = true;
        }
    }

    /// Clean formatting removing unnecessary trailing zeros and avoiding float rounding bugs
    fn format_number(val: f64) -> String {
        if val.is_infinite() || val.is_nan() {
            return "Error".to_string();
        }
        let rounded = (val * 100_000_000.0).round() / 100_000_000.0;
        if rounded.fract() == 0.0 && rounded.abs() < 1e12 {
            format!("{}", rounded as i64)
        } else {
            let mut s = format!("{:.8}", rounded);
            while s.contains('.') && (s.ends_with('0') || s.ends_with('.')) {
                if s.ends_with('.') {
                    s.pop();
                    break;
                }
                s.pop();
            }
            s
        }
    }
}
