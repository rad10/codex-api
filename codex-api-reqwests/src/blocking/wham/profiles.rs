pub use codex_api_lib::wham::profiles::sync::{Profiles, me};
use reqwest::IntoUrl;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Profiles
    for CodexClient<Auth, Acc, U>
{
    fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
