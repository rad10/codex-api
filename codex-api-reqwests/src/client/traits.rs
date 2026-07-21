use std::fmt::Display;

use http::{HeaderMap, header::AUTHORIZATION};
use uuid::Uuid;

/// Describes that the object can be utilized as an authorization header for
/// the Codex Client
pub trait CodexAuthorization: Display {
    /// Displays the authorization string that will be used in the client API
    /// call
    fn authorization(&self) -> String {
        format!("Bearer {self}")
    }

    /// Adds the auth token to headers
    fn add_authorization_header(&self, headers: &mut HeaderMap) {
        // Ensure that authorization begins with "bearer"
        debug_assert!(
            self.authorization().starts_with("Bearer "),
            "The \"authorization\" function needs to include bearer at the beginning in order to satisfy API requirements"
        );
        // Adding auth string to header
        if let Ok(auth_header) = self.authorization().parse() {
            headers.insert(AUTHORIZATION, auth_header);
        }
    }
}

/// Describes that the object can be utilized as the account ID for the codex
/// client
pub trait CodexAccountId {
    /// Displays the authorization string that will be used in the client API
    /// call
    fn account_id(&self) -> String;

    /// Adds the account ID to headers
    fn add_account_header(&self, headers: &mut HeaderMap) {
        if let Ok(account_header) = self.account_id().parse() {
            headers.insert("ChatGPT-Account-ID", account_header);
        }
    }
}

/// A blank unit to use when no account ID is used
#[derive(Debug, Clone, Copy)]
pub struct NoAccountId;

impl Default for NoAccountId {
    fn default() -> Self {
        Self
    }
}

// Blanket implementations for allowing generic usage
impl CodexAuthorization for String {
    fn authorization(&self) -> String {
        self.clone()
    }
}

impl CodexAccountId for String {
    fn account_id(&self) -> String {
        self.clone()
    }
}

impl CodexAccountId for Uuid {
    fn account_id(&self) -> String {
        self.as_hyphenated().to_string()
    }
}

impl CodexAccountId for NoAccountId {
    fn account_id(&self) -> String {
        String::new()
    }

    fn add_account_header(&self, _headers: &mut HeaderMap) {}
}
