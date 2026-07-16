use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

#[cfg(feature = "async")]
use crate::wham::profiles::ProfilesAsync;
#[cfg(feature = "boxed")]
use crate::wham::profiles::ProfilesAsyncBoxed;
#[cfg(feature = "sync")]
use crate::wham::profiles::ProfilesSync;
use crate::{ApiCommon, FutureNotSend};

pub mod profiles;

// Table of endpoint constants
pub const MODULE_WHAM: &str = "wham";
pub const ENDPOINT_RATE_LIMIT_RESET_CREDITS: &str = "rate-limit-reset-credits";
pub const ENDPOINT_USAGE: &str = "usage";

pub trait WhamSub: Sized {
    fn ps<'a>(&'a self) -> Wham<'a, Self> {
        Wham { client: self }
    }
}

/// Runs all Codex API calls
pub struct Wham<'a, C> {
    client: &'a C,
}

impl<'a, C> AsRef<C> for Wham<'a, C> {
    fn as_ref(&self) -> &C {
        &self.client
    }
}

impl<'a, C> Borrow<C> for Wham<'a, C> {
    fn borrow(&self) -> &C {
        &self.client
    }
}

#[cfg(feature = "sync")]
pub trait WhamSync: ApiCommon + ProfilesSync {
    fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: WhamSync> Wham<'a, C> {
    fn rate_limit_reset_credits(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::wham_rate_limit_reset_credits(self.borrow())
    }

    fn usage(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::wham_usage(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait WhamAsync: ApiCommon + ProfilesAsync {
    fn wham_rate_limit_reset_credits(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;

    fn wham_usage(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: WhamAsync> Wham<'a, C> {
    async fn rate_limit_reset_credits(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::wham_rate_limit_reset_credits(self.borrow()).await
    }

    async fn usage(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::wham_usage(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[async_trait]
pub trait WhamAsyncBoxed: ApiCommon + ProfilesAsyncBoxed {
    async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(feature = "boxed")]
#[async_trait]
impl<C: WhamAsync + WasmNotSync> WhamAsyncBoxed for C {
    async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        <C as WhamAsync>::wham_rate_limit_reset_credits(&self).await
    }

    async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>
    {
        <C as WhamAsync>::wham_usage(&self).await
    }
}
