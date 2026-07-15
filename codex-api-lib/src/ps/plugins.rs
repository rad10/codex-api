use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;

use crate::{ApiCommon, ps::Ps};

pub const MODULE_PLUGINS: &str = "plugins";
pub const ENDPOINT_INSTALLED: &str = "installed";
pub const ENDPOINT_LIST: &str = "list";
pub const ENDPOINT_SUGGESTED: &str = "suggested";

impl<'a, C> Ps<'a, C> {
    pub fn plugins(self) -> Plugins<'a, C> {
        Plugins { inner: self }
    }
}

/// Runs all Codex API calls
pub struct Plugins<'a, C> {
    inner: Ps<'a, C>,
}

impl<'a, C> AsRef<C> for Plugins<'a, C> {
    fn as_ref(&self) -> &C {
        self.inner.as_ref()
    }
}

impl<'a, C> Borrow<C> for Plugins<'a, C> {
    fn borrow(&self) -> &C {
        self.inner.borrow()
    }
}

#[cfg(feature = "sync")]
pub trait PluginsSync: ApiCommon {
    fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

//#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: PluginsSync> Plugins<'a, C> {
    fn installed(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String> {
        C::ps_plugins_installed(self.borrow())
    }

    fn list(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String> {
        C::ps_plugins_list(self.borrow())
    }

    fn suggested(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String> {
        C::ps_plugins_suggested(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait PluginsAsync: ApiCommon {
    fn ps_plugins_installed(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;

    fn ps_plugins_list(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;

    fn ps_plugins_suggested(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;
}

//#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: PluginsAsync> Plugins<'a, C> {
    async fn installed(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String> {
        C::ps_plugins_installed(self.borrow()).await
    }

    async fn list(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String> {
        C::ps_plugins_list(self.borrow()).await
    }

    async fn suggested(&self) -> Result<C::Response, C::ApiError>
    where
        Self::Response: TryInto<String> {
        C::ps_plugins_suggested(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[async_trait]
pub trait PluginsAsyncBoxed: ApiCommon {
    async fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    async fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    async fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(feature = "boxed")]
#[async_trait]
impl<C: PluginsAsync> PluginsAsyncBoxed for C {
    async fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>
    {
        <C as PluginsAsync>::ps_plugins_installed(&self).await
    }

    async fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>
    {
        <C as PluginsAsync>::ps_plugins_list(&self).await
    }

    async fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>
    {
        <C as PluginsAsync>::ps_plugins_suggested(&self).await
    }
}