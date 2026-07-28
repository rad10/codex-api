use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};
use codex_api_lib::plugins::sync::Plugins;
pub use codex_api_lib::plugins::sync::featured;
use reqwest::IntoUrl;

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Plugins
    for CodexClient<Auth, Acc, U>
{
    fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
