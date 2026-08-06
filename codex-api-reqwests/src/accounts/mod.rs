#[cfg(feature = "async")]
use async_from::AsyncTryInto;
#[cfg(feature = "async")]
use codex_api_lib::accounts::r#async;
#[cfg(feature = "async")]
use reqwest::IntoUrl;
use reqwest::Request;
#[cfg(feature = "async")]
use uuid::Uuid;

#[cfg(feature = "middleware")]
use crate::client::CodexMiddleware;
#[cfg(feature = "async")]
use crate::client::{
    CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

#[cfg(feature = "async")]
pub use r#async::{Accounts, settings};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait AccountsRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn account_settings_request(&self, user_id: Uuid) -> Result<Request, Self::BuildError>;
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Accounts
    for CodexClient<Auth, Acc, U>
{
    async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Accounts
    for CodexMiddleware<Auth, Acc, U>
{
    async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "threaded")]
pub mod thread_safe {
    #[cfg(feature = "middleware")]
    use super::CodexMiddleware;
    use super::{CodexAccountId, CodexAuthorization, CodexClient, IntoUrl, Uuid, r#async};

    pub use r#async::thread_safe::{Accounts, settings};

    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> Accounts
        for CodexClient<Auth, Acc, U>
    {
        async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: async_from::AsyncTryInto<String>,
        {
            todo!()
        }
    }

    #[cfg(feature = "middleware")]
    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> Accounts
        for CodexMiddleware<Auth, Acc, U>
    {
        async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: async_from::AsyncTryInto<String>,
        {
            todo!()
        }
    }
}

#[cfg(feature = "threaded")]
pub mod wasm_safe {
    use super::r#async;

    pub use r#async::wasm_safe::{Accounts, settings};
}
