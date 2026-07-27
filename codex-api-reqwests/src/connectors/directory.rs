#[cfg(feature = "async")]
use codex_api_lib::AsyncTryInto;
use codex_api_lib::connectors::directory;
use reqwest::IntoUrl;

#[cfg(feature = "async")]
use crate::client::CodexClient;
#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::traits::{CodexAccountId, CodexAuthorization};

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    directory::r#async::Directory for CodexClient<Auth, Acc, U>
{
    async fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }

    async fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    directory::r#async::Directory for CodexMiddleware<Auth, Acc, U>
{
    async fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }

    async fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> directory::sync::Directory
    for blocking::CodexClient<Auth, Acc, U>
{
    fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }

    fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
