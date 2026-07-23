use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

use crate::{ApiCommon, codex::Codex};
#[cfg(feature = "async")]
use crate::{AsyncTryInto, FutureNotSend};

pub const MODULE_ANALYTICS_EVENTS: &str = "analytics-events";
pub const ENDPOINT_EVENTS: &str = "events";

impl<'a, C> Codex<'a, C> {
    pub fn analytics_events(self) -> AnalyticsEvents<'a, C> {
        AnalyticsEvents { inner: self }
    }
}

/// Runs all Codex API calls
pub struct AnalyticsEvents<'a, C> {
    inner: Codex<'a, C>,
}

impl<'a, C> AsRef<C> for AnalyticsEvents<'a, C> {
    fn as_ref(&self) -> &C {
        self.inner.as_ref()
    }
}

impl<'a, C> Borrow<C> for AnalyticsEvents<'a, C> {
    fn borrow(&self) -> &C {
        self.inner.borrow()
    }
}

#[cfg(feature = "sync")]
pub trait AnalyticsEventsSync: ApiCommon {
    fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: AnalyticsEventsSync> AnalyticsEvents<'a, C> {
    pub fn events(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::codex_analytics_events_events(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait AnalyticsEventsAsync: ApiCommon {
    fn codex_analytics_events_events(
        &self,
    ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: AsyncTryInto<String>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: AnalyticsEventsAsync> AnalyticsEvents<'a, C> {
    /// Collects models from Codex's library
    pub async fn events(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: AsyncTryInto<String>,
    {
        C::codex_analytics_events_events(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AnalyticsEventsAsyncBoxed: ApiCommon {
    async fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>;
}

#[cfg(feature = "boxed")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<C: AnalyticsEventsAsync + WasmNotSync> AnalyticsEventsAsyncBoxed for C {
    async fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        <C as AnalyticsEventsAsync>::codex_analytics_events_events(&self).await
    }
}
