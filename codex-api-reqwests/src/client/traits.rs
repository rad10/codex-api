use std::fmt::Display;

use http::{HeaderMap, HeaderValue, header::AUTHORIZATION};
use uuid::Uuid;

/// Describes that the object can be utilized as an authorization header for
/// the Codex Client
pub trait CodexAuthorization: Display {
    /// Displays the authorization string that will be used in the client API
    /// call
    fn authorization(&self) -> String {
        format!("Bearer {self}")
    }

    /// Provides a function to directly convert into a header value
    /// 
    /// Doesn't need to be implemented often, but is helpful when its possible
    /// to have a more optimized approach
    fn as_header(&self) -> Option<HeaderValue> {
        let mut header: Option<HeaderValue> = self.authorization().parse().ok();
        if let Some(header_data) = &mut header {
            header_data.set_sensitive(true)
        }
        header
    }

    /// Adds the auth token to headers
    fn add_authorization_header(&self, headers: &mut HeaderMap) {
        // Ensure that authorization begins with "bearer"
        debug_assert!(
            self.authorization().starts_with("Bearer "),
            "The \"authorization\" function needs to include bearer at the beginning in order to satisfy API requirements"
        );
        // Adding auth string to header
        if let Some(auth_header) = self.as_header() {
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

    /// Provides a function to directly convert into a header value
    /// 
    /// Doesn't need to be implemented often, but is helpful when its possible
    /// to have a more optimized approach
    fn as_header(&self) -> Option<HeaderValue> {
        self.account_id().parse().ok()
    }

    /// Adds the account ID to headers
    fn add_account_header(&self, headers: &mut HeaderMap) {
        if let Some(account_header) = self.as_header() {
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
