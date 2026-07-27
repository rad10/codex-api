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

#[cfg(feature = "sync")]
pub mod sync {
    use crate::{
        accounts::sync::Accounts, codex::sync::Codex, connectors::sync::Connectors,
        plugins::sync::Plugins, ps::sync::Ps, wham::sync::Wham,
    };

    /// A master trait containing all modules that are available
    pub trait CodexApi: Accounts + Codex + Connectors + Plugins + Ps + Wham {}

    impl<T: Accounts + Codex + Connectors + Plugins + Ps + Wham> CodexApi for T {}
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{
        accounts::r#async::Accounts, codex::r#async::Codex, connectors::r#async::Connectors,
        plugins::r#async::Plugins, ps::r#async::Ps, wham::r#async::Wham,
    };

    /// A master trait containing all modules that are available
    pub trait CodexApi: Accounts + Codex + Connectors + Plugins + Ps + Wham {}

    impl<T: Accounts + Codex + Connectors + Plugins + Ps + Wham> CodexApi for T {}
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use crate::{
        accounts::boxed::Accounts, codex::boxed::Codex, connectors::boxed::Connectors,
        plugins::boxed::Plugins, ps::boxed::Ps, wham::boxed::Wham,
    };

    /// A master trait containing all modules that are available
    pub trait CodexApi: Accounts + Codex + Connectors + Plugins + Ps + Wham {}

    impl<T: Accounts + Codex + Connectors + Plugins + Ps + Wham> CodexApi for T {}
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
