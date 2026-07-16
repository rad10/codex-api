use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

#[cfg(feature = "async")]
use crate::FutureNotSend;
use crate::{ApiCommon, connectors::Connectors};

pub const MODULE_DIRECTORY: &str = "directory";
pub const ENDPOINT_LIST: &str = "list";
pub const ENDPOINT_LIST_WORKSPACE: &str = "list_workspace";

impl<'a, C> Connectors<'a, C> {
    pub fn directory(self) -> Directory<'a, C> {
        Directory { inner: self }
    }
}

/// Runs all Codex API calls
pub struct Directory<'a, C> {
    inner: Connectors<'a, C>,
}

impl<'a, C> AsRef<C> for Directory<'a, C> {
    fn as_ref(&self) -> &C {
        self.inner.as_ref()
    }
}

impl<'a, C> Borrow<C> for Directory<'a, C> {
    fn borrow(&self) -> &C {
        self.inner.borrow()
    }
}

#[cfg(feature = "sync")]
pub trait DirectorySync: ApiCommon {
    fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: DirectorySync> Directory<'a, C> {
    fn list(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String> {
        C::connectors_directory_list(self.borrow())
    }

    fn list_workspace(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String> {
        C::connectors_directory_list_workspace(self.borrow())
    }
}

#[cfg(feature = "async")]
pub trait DirectoryAsync: ApiCommon {
    fn connectors_directory_list(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;

    fn connectors_directory_list_workspace(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: DirectoryAsync> Directory<'a, C> {
    async fn list(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String> {
        C::connectors_directory_list(self.borrow()).await
    }

    async fn list_workspace(&self) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String> {
        C::connectors_directory_list_workspace(self.borrow()).await
    }
}

#[cfg(feature = "boxed")]
#[async_trait]
pub trait DirectoryAsyncBoxed: ApiCommon {
    async fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;

    async fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(feature = "boxed")]
#[async_trait]
impl<C: DirectoryAsync + WasmNotSync> DirectoryAsyncBoxed for C {
    async fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>
    {
        <C as DirectoryAsync>::connectors_directory_list(&self).await
    }

    async fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>
    {
        <C as DirectoryAsync>::connectors_directory_list_workspace(&self).await
    }
}