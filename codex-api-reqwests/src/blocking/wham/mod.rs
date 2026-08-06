pub use codex_api_lib::wham::sync::{Wham, rate_limit_reset_credits, usage};
use reqwest::{IntoUrl, blocking::Request};

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

pub mod profiles;

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait WhamRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn wham_rate_limit_reset_credits_request(&self) -> Result<Request, Self::BuildError>;

    fn wham_usage_request(&self) -> Result<Request, Self::BuildError>;
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Wham for CodexClient<Auth, Acc, U> {
    fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }

    fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
