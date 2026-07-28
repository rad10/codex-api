use codex_api_lib::codex::analytics_events::sync::AnalyticsEvents;
pub use codex_api_lib::codex::analytics_events::sync::events;
use reqwest::IntoUrl;

use crate::client::{
    blocking::CodexClient,
    traits::{CodexAccountId, CodexAuthorization},
};

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
