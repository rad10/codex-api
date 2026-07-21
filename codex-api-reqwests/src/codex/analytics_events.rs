#[cfg(feature = "async")]
use codex_api_lib::codex::analytics_events::AnalyticsEventsAsync;
#[cfg(feature = "sync")]
use codex_api_lib::codex::analytics_events::AnalyticsEventsSync;
use reqwest::IntoUrl;

#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::{CodexClient, traits::{CodexAccountId, CodexAuthorization}};

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AnalyticsEventsAsync for CodexClient<Auth, Acc, U> {
    async fn codex_analytics_events_events(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AnalyticsEventsAsync for CodexMiddleware<Auth, Acc, U> {
    async fn codex_analytics_events_events(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> AnalyticsEventsSync for blocking::CodexClient<Auth, Acc, U> {
    fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}
