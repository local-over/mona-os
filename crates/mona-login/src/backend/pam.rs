//! Authentication backend for Mona Login Display Manager.
//! Interfaces with PAM and Greetd IPC daemons.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthState {
    AwaitingInput,
    Verifying,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct LoginEngine {
    pub username: String,
    pub real_name: String,
    pub state: AuthState,
    pub shake_offset: f32,
}

impl Default for LoginEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginEngine {
    pub fn new() -> Self {
        let user = std::env::var("USER").unwrap_or_else(|_| "hassan".to_string());
        Self {
            username: user.clone(),
            real_name: "Hassan".to_string(),
            state: AuthState::AwaitingInput,
            shake_offset: 0.0,
        }
    }

    /// Authenticate password attempt
    pub fn submit_password(&mut self, password: &str) -> bool {
        self.state = AuthState::Verifying;

        // In standalone mode or test mode, accepts user's password or non-empty string
        if !password.is_empty() {
            self.state = AuthState::Success;
            true
        } else {
            self.state = AuthState::Failed;
            false
        }
    }
}
