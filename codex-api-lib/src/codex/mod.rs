use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;

#[cfg(feature = "async")]
use crate::codex::analytics_events::AnalyticsEventsAsync;
#[cfg(feature = "boxed")]
use crate::codex::analytics_events::AnalyticsEventsAsyncBoxed;
#[cfg(feature = "sync")]
use crate::codex::analytics_events::AnalyticsEventsSync;
use crate::{ApiCommon, FutureNotSend};

pub mod analytics_events;

// Table of endpoint constants
pub const MODULE_CODEX: &str = "codex";
pub const ENDPOINT_MODELS: &str = "models";
pub const ENDPOINT_RESPONSES: &str = "responses";

pub trait CodexSub {
    fn codex<'a>(&'a self) -> Codex<'a, Self> {
        Codex { client: self }
    }
}

/// Runs all Codex API calls
pub struct Codex<'a, C> {
    client: &'a C,
}

impl<'a, C> AsRef<C> for Codex<'a, C> {
    fn as_ref(&self) -> &C {
        &self.client
    }
}

impl<'a, C> Borrow<C> for Codex<'a, C> {
    fn borrow(&self) -> &C {
        &self.client
    }
}

#[cfg(feature = "sync")]
pub trait CodexSync: ApiCommon + AnalyticsEventsSync {
    /// Collects models from Codex's library
    fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    /// Collects a response from ChatGPT's API
    fn codex_responses(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

//#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: CodexSync> Codex<'a, C> {
    fn models(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        C::codex_models(self.borrow())
    }

    fn responses(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        C::codex_responses(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait CodexAsync: ApiCommon + AnalyticsEventsAsync {
    /// Collects models from Codex's library
    fn codex_models(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;

    /// Collects a response from ChatGPT's API
    fn codex_responses(
        &self,
    ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;
}

//#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: CodexAsync> Codex<'a, C> {
    /// Collects models from Codex's library
    async fn models(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        C::codex_models(self.borrow()).await
    }

    /// Collects a response from ChatGPT's API
    async fn responses(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        C::codex_responses(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[async_trait]
pub trait CodexAsyncBoxed: ApiCommon + AnalyticsEventsAsyncBoxed {
    /// Collects models from Codex's library
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    /// Collects a response from ChatGPT's API
    async fn codex_responses(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(feature = "boxed")]
#[async_trait]
impl<C: CodexAsync> CodexAsyncBoxed for C {
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        <C as CodexAsync>::codex_models(&self).await
    }

    async fn codex_responses(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        <C as CodexAsync>::codex_responses(&self).await
    }
}
