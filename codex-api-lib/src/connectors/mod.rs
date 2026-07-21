use std::borrow::Borrow;

use crate::ApiCommon;
#[cfg(feature = "async")]
use crate::connectors::directory::DirectoryAsync;
#[cfg(feature = "boxed")]
use crate::connectors::directory::DirectoryAsyncBoxed;
#[cfg(feature = "sync")]
use crate::connectors::directory::DirectorySync;

pub mod directory;

// Table of endpoint constants
pub const MODULE_CONNECTORS: &str = "connectors";

pub trait ConnectorsSub: Sized {
    fn connectors<'a>(&'a self) -> Connectors<'a, Self> {
        Connectors { client: self }
    }
}

/// Runs all Codex API calls
pub struct Connectors<'a, C> {
    client: &'a C,
}

impl<'a, C> AsRef<C> for Connectors<'a, C> {
    fn as_ref(&self) -> &C {
        &self.client
    }
}

impl<'a, C> Borrow<C> for Connectors<'a, C> {
    fn borrow(&self) -> &C {
        &self.client
    }
}

#[cfg(feature = "sync")]
pub trait ConnectorsSync: ApiCommon + DirectorySync {}

#[cfg(feature = "sync")]
impl<C: DirectorySync> ConnectorsSync for C {}

#[cfg(feature = "async")]
pub trait ConnectorsAsync: ApiCommon + DirectoryAsync {}

#[cfg(feature = "async")]
impl<C: DirectoryAsync> ConnectorsAsync for C {}

#[cfg(feature = "boxed")]
pub trait ConnectorsAsyncBoxed: ApiCommon + DirectoryAsyncBoxed {}

#[cfg(feature = "boxed")]
impl<C: DirectoryAsyncBoxed> ConnectorsAsyncBoxed for C {}
