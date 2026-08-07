use std::env;

#[cfg(feature = "async")]
use codex_api_reqwests::client::CodexClient;
#[cfg(feature = "sync")]
use codex_api_reqwests::client::blocking::CodexClient as BlockingClient;
use uuid::Uuid;

pub mod accounts;
#[cfg(feature = "sync")]
pub mod blocking;
pub mod codex;
pub mod connectors;
pub mod plugins;
pub mod ps;
pub mod wham;

pub const CODEX_AUTHORIZATION_TOKEN: Option<&'static str> =
    option_env!("CODEX_AUTHORIZATION_TOKEN");

pub fn codex_authorization_token() -> Option<String> {
    env::var("CODEX_AUTHORIZATION_KEY").ok()
}

pub fn codex_account_id() -> Option<Uuid> {
    env::var("CODEX_ACCOUNT_ID")
        .ok()
        .and_then(|s| s.parse().ok())
}

#[cfg(feature = "async")]
pub fn create_client() -> CodexClient<String, Uuid, &'static str> {
    let client = CodexClient::new(
        codex_authorization_token().expect("authorization key should be described"),
    )
    .unwrap();
    if let Some(account_id) = codex_account_id() {
        client.with_account(account_id)
    } else {
        client
    }
}

#[cfg(feature = "sync")]
pub fn create_blocking_client() -> BlockingClient<String, Uuid, &'static str> {
    let client = BlockingClient::new(
        codex_authorization_token().expect("authorization key should be described"),
    )
    .unwrap();
    if let Some(account_id) = codex_account_id() {
        client.with_account(account_id)
    } else {
        client
    }
}
