#[cfg(feature = "async")]
use wasm_not_send_sync::WasmNotSend;

pub mod accounts;
#[cfg(feature = "async")]
mod async_from;
pub mod codex;
pub mod connectors;
pub mod plugins;
pub mod ps;
pub mod wham;

pub const STANDARD_ENDPOINT: &'static str = "https://chatgpt.com/backend-ui/";

/// A master trait containing all modules that are available
pub trait CodexApi:
    accounts::AccountsSub
    + codex::CodexSub
    + connectors::ConnectorsSub
    + plugins::PluginsSub
    + ps::PsSub
    + wham::WhamSub
{
}

#[cfg(feature = "sync")]
pub trait CodexApiSync:
    accounts::AccountsSync
    + codex::CodexSync
    + connectors::ConnectorsSync
    + plugins::PluginsSync
    + ps::PsSync
    + wham::WhamSync
{
}

#[cfg(feature = "async")]
pub trait CodexApiAsync:
    accounts::AccountsAsync
    + codex::CodexAsync
    + connectors::ConnectorsAsync
    + plugins::PluginsAsync
    + ps::PsAsync
    + wham::WhamAsync
{
}

#[cfg(feature = "boxed")]
pub trait CodexApiBoxed:
    accounts::AccountsAsyncBoxed
    + codex::CodexAsyncBoxed
    + connectors::ConnectorsAsyncBoxed
    + plugins::PluginsAsyncBoxed
    + ps::PsAsyncBoxed
    + wham::WhamAsyncBoxed
{
}

// blanket implement of api trait on all models that implement all traits
impl<
    T: accounts::AccountsSub
        + codex::CodexSub
        + connectors::ConnectorsSub
        + plugins::PluginsSub
        + ps::PsSub
        + wham::WhamSub,
> CodexApi for T
{
}
#[cfg(feature = "sync")]
impl<
    T: accounts::AccountsSync
        + codex::CodexSync
        + connectors::ConnectorsSync
        + plugins::PluginsSync
        + ps::PsSync
        + wham::WhamSync,
> CodexApiSync for T
{
}
#[cfg(feature = "async")]
impl<
    T: accounts::AccountsAsync
        + codex::CodexAsync
        + connectors::ConnectorsAsync
        + plugins::PluginsAsync
        + ps::PsAsync
        + wham::WhamAsync,
> CodexApiAsync for T
{
}
#[cfg(feature = "boxed")]
impl<
    T: accounts::AccountsAsyncBoxed
        + codex::CodexAsyncBoxed
        + connectors::ConnectorsAsyncBoxed
        + plugins::PluginsAsyncBoxed
        + ps::PsAsyncBoxed
        + wham::WhamAsyncBoxed,
> CodexApiBoxed for T
{
}

#[cfg(feature = "async")]
pub use async_from::{AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto};

// Creating a type for async functions
#[cfg(feature = "async")]
pub trait FutureNotSend: Future + WasmNotSend {}
#[cfg(feature = "async")]
impl<T: Future + WasmNotSend> FutureNotSend for T {}

/// Functions and types that will be common amongst all API calls
pub trait ApiCommon {
    /// The response type that all responses will be based on
    type Response;

    /// The error type that will occur on an API error
    type ApiError;
}
