use reqwest::IntoUrl;

pub mod plugins;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};
pub use codex_api_lib::ps::sync::{Ps, mcp};

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Ps for CodexClient<Auth, Acc, U> {
    fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
