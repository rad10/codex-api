pub use codex_api_lib::wham::profiles::sync::{Profiles, me};
use reqwest::{IntoUrl, blocking::Request};

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait ProfilesRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn wham_profiles_me_request(&self) -> Result<Request, Self::BuildError>;
}

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
