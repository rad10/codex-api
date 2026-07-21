#[cfg(feature = "async")]
use codex_api_lib::ps::PsAsync;
use codex_api_lib::ps::PsSub;
#[cfg(feature = "sync")]
use codex_api_lib::ps::PsSync;
use reqwest::IntoUrl;

#[cfg(feature = "async")]
use crate::client::CodexClient;
#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::traits::{CodexAuthorization, CodexAccountId};


pub mod plugins;

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PsSub for CodexClient<Auth, Acc, U> {}
#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PsSub for CodexMiddleware<Auth, Acc, U> {}
#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PsSub for blocking::CodexClient<Auth, Acc, U> {}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PsAsync for CodexClient<Auth, Acc, U> {
    fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PsAsync for CodexMiddleware<Auth, Acc, U> {
    fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PsSync for blocking::CodexClient<Auth, Acc, U> {
    fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}
