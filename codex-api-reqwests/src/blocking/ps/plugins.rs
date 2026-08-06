use reqwest::{IntoUrl, blocking::Request};

pub use codex_api_lib::ps::plugins::sync::{Plugins, installed, list, suggested};

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait PluginsRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn ps_plugins_installed_request(&self) -> Result<Request, Self::BuildError>;

    fn ps_plugins_list_request(&self) -> Result<Request, Self::BuildError>;

    fn ps_plugins_suggested_request(&self) -> Result<Request, Self::BuildError>;
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> Plugins
    for CodexClient<Auth, Acc, U>
{
    fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }

    fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }

    fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
