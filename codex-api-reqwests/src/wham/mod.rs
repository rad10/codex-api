use codex_api_lib::wham::WhamSub;
#[cfg(feature = "sync")]
use codex_api_lib::wham::WhamSync;
#[cfg(feature = "async")]
use codex_api_lib::{AsyncTryInto, wham::WhamAsync};
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

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> WhamSub
    for CodexClient<Auth, Acc, U>
{
}
#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> WhamSub
    for CodexMiddleware<Auth, Acc, U>
{
}
#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> WhamSub
    for blocking::CodexClient<Auth, Acc, U>
{
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> WhamAsync
    for CodexClient<Auth, Acc, U>
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
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> WhamAsync
    for CodexMiddleware<Auth, Acc, U>
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
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> WhamSync
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
