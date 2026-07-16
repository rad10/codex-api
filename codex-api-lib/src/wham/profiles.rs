use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

#[cfg(feature = "async")]
use crate::FutureNotSend;
use crate::{ApiCommon, wham::Wham};

pub const MODULE_PROFILES: &str = "profiles";
pub const ENDPOINT_ME: &str = "me";

impl<'a, C> Wham<'a, C> {
    pub fn analytics_events(self) -> Profiles<'a, C> {
        Profiles { inner: self }
    }
}

/// Runs all Codex API calls
pub struct Profiles<'a, C> {
    inner: Wham<'a, C>,
}

impl<'a, C> AsRef<C> for Profiles<'a, C> {
    fn as_ref(&self) -> &C {
        self.inner.as_ref()
    }
}

impl<'a, C> Borrow<C> for Profiles<'a, C> {
    fn borrow(&self) -> &C {
        self.inner.borrow()
    }
}

#[cfg(feature = "sync")]
pub trait ProfilesSync: ApiCommon {
    fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: ProfilesSync> Profiles<'a, C> {
    fn me(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String> {
        C::wham_profiles_me(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait ProfilesAsync: ApiCommon {
    fn wham_profiles_me(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: ProfilesAsync> Profiles<'a, C> {
    /// Collects models from Codex's library
    async fn me(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String> {
        C::wham_profiles_me(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[async_trait]
pub trait ProfilesAsyncBoxed: ApiCommon {
    async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(feature = "boxed")]
#[async_trait]
impl<C: ProfilesAsync + WasmNotSync> ProfilesAsyncBoxed for C {
    async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>
    {
        <C as ProfilesAsync>::wham_profiles_me(&self).await
    }
}