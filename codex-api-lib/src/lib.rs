use wasm_not_send_sync::WasmNotSend;

pub mod accounts;
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

pub trait CodexApiSync:
    accounts::AccountsSync
    + codex::CodexSync
    + connectors::ConnectorsSync
    + plugins::PluginsSync
    + ps::PsSync
    + wham::WhamSync
{
}

pub trait CodexApiAsync:
    accounts::AccountsAsync
    + codex::CodexAsync
    + connectors::ConnectorsAsync
    + plugins::PluginsAsync
    + ps::PsAsync
    + wham::WhamAsync
{
}

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

// Creating a type for async functions
pub trait FutureNotSend: Future + WasmNotSend {}
impl<T: Future + WasmNotSend> FutureNotSend for T {}

/// Functions and types that will be common amongst all API calls
pub trait ApiCommon {
    /// The response type that all responses will be based on
    type Response;

    /// The error type that will occur on an API error
    type ApiError;
}
