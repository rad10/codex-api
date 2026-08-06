pub use codex_api_lib::codex::analytics_events::sync::{AnalyticsEvents, events};
use reqwest::{IntoUrl, blocking::Request};

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait AnalyticsEventsRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn codex_analytics_events_events_request(&self) -> Result<Request, Self::BuildError>;
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AnalyticsEvents
    for CodexClient<Auth, Acc, U>
{
    fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        todo!()
    }
}
