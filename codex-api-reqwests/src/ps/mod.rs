#[cfg(feature = "async")]
use codex_api_lib::AsyncTryInto;
use codex_api_lib::ps;
use reqwest::IntoUrl;

#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::{
    CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

pub mod plugins;

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> ps::r#async::Ps
    for CodexClient<Auth, Acc, U>
{
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> ps::r#async::Ps
    for CodexMiddleware<Auth, Acc, U>
{
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ps::sync::Ps
    for blocking::CodexClient<Auth, Acc, U>
{
    fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
