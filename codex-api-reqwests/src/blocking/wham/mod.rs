pub use codex_api_lib::wham::sync::{Wham, rate_limit_reset_credits, usage};
use reqwest::IntoUrl;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

pub mod profiles;

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
