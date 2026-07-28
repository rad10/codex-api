#[cfg(feature = "async")]
use async_from::AsyncTryInto;
#[cfg(feature = "async")]
use codex_api_lib::ps::r#async;
#[cfg(feature = "async")]
use reqwest::IntoUrl;

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
pub use r#async::mcp;

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> r#async::Ps
    for CodexClient<Auth, Acc, U>
{
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String> {
        todo!()
    }
}

#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> r#async::Ps
    for CodexMiddleware<Auth, Acc, U>
{
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String> {
        todo!()
    }
}

#[cfg(feature = "threaded")]
pub mod thread_safe {
    use super::{CodexAccountId, CodexAuthorization, CodexClient, IntoUrl, r#async};
    #[cfg(feature = "middleware")]
    use super::CodexMiddleware;

    pub use r#async::thread_safe::mcp;

    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
        r#async::thread_safe::Ps for CodexClient<Auth, Acc, U>
    {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: async_from::AsyncTryInto<String> {
            todo!()
        }
    }

#[cfg(feature = "middleware")]
    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Sync>
        r#async::thread_safe::Ps for CodexMiddleware<Auth, Acc, U>
    {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: async_from::AsyncTryInto<String> {
        todo!()
    }
    }
}

#[cfg(feature = "threaded")]
pub mod wasm_safe {
    use super::r#async;

    pub use r#async::wasm_safe::mcp;
}
