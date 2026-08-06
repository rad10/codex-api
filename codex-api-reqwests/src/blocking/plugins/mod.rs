use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};
pub use codex_api_lib::plugins::sync::{Plugins, featured};
use reqwest::{IntoUrl, blocking::Request};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait PluginsRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn plugins_featured_request(&self) -> Result<Request, Self::BuildError>;
}

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
