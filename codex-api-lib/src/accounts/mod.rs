use std::borrow::Borrow;

#[cfg(feature = "boxed")]
use async_trait::async_trait;
use uuid::Uuid;
#[cfg(feature = "boxed")]
use wasm_not_send_sync::WasmNotSync;

use crate::ApiCommon;
#[cfg(feature = "async")]
use crate::{AsyncTryInto, FutureNotSend};

// Table of endpoint constants
pub const MODULE_ACCOUNTS: &str = "accounts";
pub const ENDPOINT_SETTINGS: &str = "settings";

pub trait AccountsSub: Sized {
    fn accounts<'a>(&'a self) -> Account<'a, Self> {
        Account { client: self }
    }
}

/// Runs all Codex API calls
pub struct Account<'a, C> {
    client: &'a C,
}

impl<'a, C> AsRef<C> for Account<'a, C> {
    fn as_ref(&self) -> &C {
        &self.client
    }
}

impl<'a, C> Borrow<C> for Account<'a, C> {
    fn borrow(&self) -> &C {
        &self.client
    }
}

#[cfg(feature = "sync")]
pub trait AccountsSync: ApiCommon {
    /// Gets the settings for the given user's account
    fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String>;
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl<'a, C: AccountsSync> Account<'a, C> {
    /// Gets the settings for the given user's account
    pub fn settings(&self, user_id: Uuid) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        C::account_settings(self.borrow(), user_id)
    }
}

#[cfg(feature = "async")]
pub trait AccountsAsync: ApiCommon {
    /// Gets the settings for the given user's account
    fn account_settings(
        &self,
        user_id: Uuid,
    ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: AsyncTryInto<String>;
}

#[cfg(all(feature = "async", not(feature = "sync")))]
impl<'a, C: AccountsAsync> Account<'a, C> {
    /// Gets the settings for the given user's account
    pub async fn settings(&self, user_id: Uuid) -> Result<C::Response, C::ApiError>
    where
        C::Response: AsyncTryInto<String>,
    {
        C::account_settings(self.borrow(), user_id).await
    }
}

#[cfg(feature = "boxed")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AccountsAsyncBoxed: ApiCommon {
    /// Gets the settings for the given user's account
    async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>;
}

#[cfg(feature = "boxed")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<C: AccountsAsync + WasmNotSync> AccountsAsyncBoxed for C {
    async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<String>,
    {
        <C as AccountsAsync>::account_settings(&self, user_id).await
    }
}
