#[cfg(feature = "async")]
use codex_api_lib::AsyncTryInto;
use codex_api_lib::wham::profiles;
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
    profiles::r#async::Profiles for CodexClient<Auth, Acc, U>
{
    async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
    profiles::r#async::Profiles for CodexMiddleware<Auth, Acc, U>
{
    async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> profiles::sync::Profiles
    for blocking::CodexClient<Auth, Acc, U>
{
    fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
