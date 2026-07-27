#[cfg(feature = "async")]
use codex_api_lib::AsyncTryInto;
use codex_api_lib::wham;
use reqwest::IntoUrl;

#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::{
    CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

pub mod profiles;

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    wham::r#async::Wham for CodexClient<Auth, Acc, U>
{
    async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }

    async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    wham::r#async::Wham for CodexMiddleware<Auth, Acc, U>
{
    async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }

    async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> wham::sync::Wham
    for blocking::CodexClient<Auth, Acc, U>
{
    fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }

    fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
