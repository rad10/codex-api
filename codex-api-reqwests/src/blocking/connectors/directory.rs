pub use codex_api_lib::connectors::directory::sync::{Directory, list, list_workspace};
use reqwest::{IntoUrl, blocking::Request};

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait DirectoryRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn connectors_directory_list_request(&self) -> Result<Request, Self::BuildError>;

    fn connectors_directory_list_workspace_request(&self) -> Result<Request, Self::BuildError>;
}

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
