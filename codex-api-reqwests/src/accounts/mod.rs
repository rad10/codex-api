#[cfg(feature = "async")]
use codex_api_lib::AsyncTryInto;
use codex_api_lib::accounts;
use reqwest::IntoUrl;
use uuid::Uuid;

#[cfg(feature = "middleware")]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::{
    CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    accounts::r#async::Accounts for CodexClient<Auth, Acc, U>
{
    async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    accounts::r#async::Accounts for CodexMiddleware<Auth, Acc, U>
{
    async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> accounts::sync::Accounts
    for blocking::CodexClient<Auth, Acc, U>
{
    fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
