use codex_api_lib::accounts::sync::Accounts;
use reqwest::IntoUrl;
use uuid::Uuid;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

pub use codex_api_lib::accounts::sync::settings;

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
