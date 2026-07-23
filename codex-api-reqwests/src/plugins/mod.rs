use codex_api_lib::plugins::PluginsSub;
#[cfg(feature = "sync")]
use codex_api_lib::plugins::PluginsSync;
#[cfg(feature = "async")]
use codex_api_lib::{AsyncTryInto, plugins::PluginsAsync};
use reqwest::IntoUrl;

#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::{
    CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PluginsSub
    for CodexClient<Auth, Acc, U>
{
}
#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PluginsSub
    for CodexMiddleware<Auth, Acc, U>
{
}
#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PluginsSub
    for blocking::CodexClient<Auth, Acc, U>
{
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> PluginsAsync
    for CodexClient<Auth, Acc, U>
{
    async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> PluginsAsync
    for CodexMiddleware<Auth, Acc, U>
{
    async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PluginsSync
    for blocking::CodexClient<Auth, Acc, U>
{
    fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
