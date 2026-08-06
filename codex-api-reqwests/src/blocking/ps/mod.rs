use reqwest::{IntoUrl, blocking::Request};

pub mod plugins;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};
pub use codex_api_lib::ps::sync::{Ps, mcp};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait PsRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn ps_mcp_request(&self) -> Result<Request, Self::BuildError>;
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Ps for CodexClient<Auth, Acc, U> {
    fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
