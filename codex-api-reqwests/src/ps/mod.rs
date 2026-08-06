#[cfg(feature = "async")]
use async_from::AsyncTryInto;
#[cfg(feature = "async")]
use codex_api_lib::ps::r#async;
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
pub mod plugins;

#[cfg(feature = "async")]
pub use r#async::{Ps, mcp};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait PsRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn ps_mcp_request(&self) -> Result<Request, Self::BuildError>;
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Ps for CodexClient<Auth, Acc, U> {
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
    }
}

#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Ps
    for CodexMiddleware<Auth, Acc, U>
{
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
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

    pub use r#async::thread_safe::{Ps, mcp};

    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> Ps
        for CodexClient<Auth, Acc, U>
    {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: async_from::AsyncTryInto<String>,
        {
            todo!()
        }
    }

    #[cfg(feature = "middleware")]
    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync> Ps
        for CodexMiddleware<Auth, Acc, U>
    {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
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

    pub use r#async::wasm_safe::{Ps, mcp};
}
