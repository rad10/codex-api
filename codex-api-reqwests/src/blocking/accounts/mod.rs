use reqwest::{IntoUrl, blocking::Request};
use uuid::Uuid;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

pub use codex_api_lib::accounts::sync::{Accounts, settings};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait AccountsRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn account_settings_request(&self, user_id: Uuid) -> Result<Request, Self::BuildError>;
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Accounts
    for CodexClient<Auth, Acc, U>
{
    fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
