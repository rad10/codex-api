pub use codex_api_lib::connectors::directory::sync::{Directory, list, list_workspace};
use reqwest::IntoUrl;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Directory
    for CodexClient<Auth, Acc, U>
{
    fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }

    fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
