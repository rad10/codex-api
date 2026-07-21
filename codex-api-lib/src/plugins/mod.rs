use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

use crate::ApiCommon;
#[cfg(feature = "async")]
use crate::{AsyncTryInto, FutureNotSend};

// Table of endpoint constants
pub const MODULE_PLUGINS: &str = "plugins";
pub const ENDPOINT_FEATURED: &str = "featured";

pub trait PluginsSub: Sized {
    fn accounts<'a>(&'a self) -> Plugins<'a, Self> {
        Plugins { client: self }
    }
}

/// Runs all Codex API calls
pub struct Plugins<'a, C> {
    client: &'a C,
}

impl<'a, C> AsRef<C> for Plugins<'a, C> {
    fn as_ref(&self) -> &C {
        &self.client
    }
}

impl<'a, C> Borrow<C> for Plugins<'a, C> {
    fn borrow(&self) -> &C {
        &self.client
    }
}

#[cfg(feature = "sync")]
pub trait PluginsSync: ApiCommon {
    /// Gets the settings for the given user's account
    fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: PluginsSync> Plugins<'a, C> {
    /// Gets the settings for the given user's account
    pub fn featured(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::plugins_featured(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait PluginsAsync: ApiCommon {
    /// Gets the settings for the given user's account
    fn plugins_featured(
        &self,
    ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: AsyncTryInto<String>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: PluginsAsync> Plugins<'a, C> {
    /// Gets the settings for the given user's account
    pub async fn featured(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: AsyncTryInto<String>,
    {
        C::plugins_featured(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait PluginsAsyncBoxed: ApiCommon {
    /// Gets the settings for the given user's account
    async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>;
}

#[cfg(feature = "boxed")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<C: PluginsAsync + WasmNotSync> PluginsAsyncBoxed for C {
    async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        <C as PluginsAsync>::plugins_featured(&self).await
    }
}
