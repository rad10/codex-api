use std::{
    borrow::Borrow,
    sync::{Arc, OnceLock},
};

#[cfg(feature = "boxed")]
use async_trait::async_trait;
#[cfg(feature = "sync")]
use codex_api_types::codex::ResponsesApiRequest;
use codex_api_types::codex::{ModelsResponse, SessionSource};
use http::HeaderMap;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

use crate::ApiCommon;
#[cfg(feature = "boxed")]
use crate::codex::analytics_events::AnalyticsEventsAsyncBoxed;
#[cfg(feature = "sync")]
use crate::codex::analytics_events::AnalyticsEventsSync;
#[cfg(feature = "async")]
use crate::{FutureNotSend, codex::analytics_events::AnalyticsEventsAsync};

pub mod analytics_events;

// Table of endpoint constants
pub const MODULE_CODEX: &str = "codex";
pub const ENDPOINT_MODELS: &str = "models";
pub const ENDPOINT_RESPONSES: &str = "responses";

pub trait CodexSub: Sized {
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
        Self::Response: TryInto<ModelsResponse>;

    /// Collects a response from ChatGPT's API
    fn codex_responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: CodexSync> Codex<'a, C> {
    pub fn models(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<ModelsResponse>,
    {
        C::codex_models(self.borrow())
    }

    pub fn responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::codex_responses(self.borrow(), request, options)
    }
}

#[cfg(feature = "async")]
pub trait CodexAsync: ApiCommon + AnalyticsEventsAsync {
    /// Collects models from Codex's library
    fn codex_models(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<ModelsResponse>;

    /// Collects a response from ChatGPT's API
    fn codex_responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: CodexAsync> Codex<'a, C> {
    /// Collects models from Codex's library
    pub async fn models(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<ModelsResponse>,
    {
        C::codex_models(self.borrow()).await
    }

    /// Collects a response from ChatGPT's API
    pub async fn responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<C::Response, C::ApiError> {
        C::codex_responses(self.borrow(), request, options).await
    }
}

#[cfg(feature = "boxed")]
#[async_trait]
pub trait CodexAsyncBoxed: ApiCommon + AnalyticsEventsAsyncBoxed {
    /// Collects models from Codex's library
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<ModelsResponse>;

    /// Collects a response from ChatGPT's API
    async fn codex_responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>;
}

#[cfg(feature = "boxed")]
#[async_trait]
impl<C: CodexAsync + WasmNotSync> CodexAsyncBoxed for C {
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<ModelsResponse>,
    {
        <C as CodexAsync>::codex_models(&self).await
    }

    async fn codex_responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError> {
        <C as CodexAsync>::codex_responses(&self, request, options).await
    }
}

#[derive(Default)]
pub struct ResponsesOptions {
    pub session_id: Option<String>,
    pub thread_id: Option<String>,
    pub session_source: Option<SessionSource>,
    pub extra_headers: HeaderMap,
    pub turn_state: Option<Arc<OnceLock<String>>>,
}
