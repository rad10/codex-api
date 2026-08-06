#[cfg(feature = "async")]
use async_from::AsyncTryInto;
#[cfg(feature = "async")]
use codex_api_lib::wham::r#async;
#[cfg(feature = "async")]
use reqwest::IntoUrl;
use reqwest::Request;

#[cfg(feature = "middleware")]
use crate::client::CodexMiddleware;
#[cfg(feature = "async")]
use crate::client::{
    CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

#[cfg(feature = "async")]
pub mod profiles;

#[cfg(feature = "async")]
pub use r#async::{Wham, rate_limit_reset_credits, usage};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait WhamRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn wham_rate_limit_reset_credits_request(&self) -> Result<Request, Self::BuildError>;

    fn wham_usage_request(&self) -> Result<Request, Self::BuildError>;
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Wham for CodexClient<Auth, Acc, U> {
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

#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Wham
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

#[cfg(feature = "threaded")]
pub mod thread_safe {
    #[cfg(feature = "middleware")]
    use super::CodexMiddleware;
    use super::{CodexAccountId, CodexAuthorization, CodexClient, IntoUrl, r#async};

    pub use r#async::thread_safe::{Wham, rate_limit_reset_credits, usage};

    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> Wham
        for CodexClient<Auth, Acc, U>
    {
        async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: async_from::AsyncTryInto<String>,
        {
            todo!()
        }

        async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: async_from::AsyncTryInto<String>,
        {
            todo!()
        }
    }

    #[cfg(feature = "middleware")]
    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> Wham
        for CodexMiddleware<Auth, Acc, U>
    {
        async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: async_from::AsyncTryInto<String>,
        {
            todo!()
        }

        async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
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

    pub use r#async::wasm_safe::{Wham, rate_limit_reset_credits, usage};
}
