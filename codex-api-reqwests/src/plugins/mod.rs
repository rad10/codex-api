#[cfg(feature = "async")]
use codex_api_lib::AsyncTryInto;
use codex_api_lib::plugins;
use reqwest::IntoUrl;

#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::{
    CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    plugins::r#async::Plugins for CodexClient<Auth, Acc, U>
{
    async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    plugins::r#async::Plugins for CodexMiddleware<Auth, Acc, U>
{
    async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> plugins::sync::Plugins
    for blocking::CodexClient<Auth, Acc, U>
{
    fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
