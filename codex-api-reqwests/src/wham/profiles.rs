#[cfg(feature = "async")]
use codex_api_lib::wham::profiles::ProfilesAsync;
#[cfg(feature = "sync")]
use codex_api_lib::wham::profiles::ProfilesSync;
use reqwest::IntoUrl;

#[cfg(feature = "async")]
use crate::client::CodexClient;
#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::traits::{CodexAuthorization, CodexAccountId};

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> ProfilesAsync
    for CodexClient<Auth, Acc, U>
{
    async fn wham_profiles_me(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> ProfilesAsync
    for CodexMiddleware<Auth, Acc, U>
{
    async fn wham_profiles_me(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ProfilesSync
    for blocking::CodexClient<Auth, Acc, U>
{
    fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}
