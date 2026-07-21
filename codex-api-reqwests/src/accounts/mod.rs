#[cfg(feature = "async")]
use codex_api_lib::accounts::AccountsAsync;
use codex_api_lib::accounts::AccountsSub;
#[cfg(feature = "sync")]
use codex_api_lib::accounts::AccountsSync;
use reqwest::IntoUrl;
#[cfg(feature = "sync")]
use uuid::Uuid;

use crate::client::{CodexClient, CodexMiddleware, blocking, traits::{CodexAccountId, CodexAuthorization}};

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AccountsSub for CodexClient<Auth, Acc, U> {}
#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AccountsSub for CodexMiddleware<Auth, Acc, U> {}
#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AccountsSub for blocking::CodexClient<Auth, Acc, U> {}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> AccountsAsync for CodexClient<Auth, Acc, U> {
    async fn account_settings(
        &self,
        user_id: Uuid,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AccountsAsync for CodexMiddleware<Auth, Acc, U> {
    async fn account_settings(
        &self,
        user_id: Uuid,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AccountsSync for blocking::CodexClient<Auth, Acc, U> {
    fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}
