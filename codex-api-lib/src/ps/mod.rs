use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

#[cfg(feature = "async")]
use crate::ps::plugins::PluginsAsync;
#[cfg(feature = "boxed")]
use crate::ps::plugins::PluginsAsyncBoxed;
#[cfg(feature = "sync")]
use crate::ps::plugins::PluginsSync;
use crate::{ApiCommon, FutureNotSend};

pub mod plugins;

// Table of endpoint constants
pub const MODULE_PS: &str = "ps";
pub const ENDPOINT_MCP: &str = "mcp";

pub trait PsSub: Sized {
    fn ps<'a>(&'a self) -> Ps<'a, Self> {
        Ps { client: self }
    }
}

/// Runs all Codex API calls
pub struct Ps<'a, C> {
    client: &'a C,
}

impl<'a, C> AsRef<C> for Ps<'a, C> {
    fn as_ref(&self) -> &C {
        &self.client
    }
}

impl<'a, C> Borrow<C> for Ps<'a, C> {
    fn borrow(&self) -> &C {
        &self.client
    }
}

#[cfg(feature = "sync")]
pub trait PsSync: ApiCommon + PluginsSync {
    fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: PsSync> Ps<'a, C> {
    fn mcp(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::ps_mcp(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait PsAsync: ApiCommon + PluginsAsync {
    fn ps_mcp(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: PsAsync> Ps<'a, C> {
    async fn mcp(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::ps_mcp(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[async_trait]
pub trait PsAsyncBoxed: ApiCommon + PluginsAsyncBoxed {
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(feature = "boxed")]
#[async_trait]
impl<C: PsAsync + WasmNotSync> PsAsyncBoxed for C {
    async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>,
    {
        <C as PsAsync>::ps_mcp(&self).await
    }
}
