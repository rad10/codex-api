pub mod accounts;
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

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use crate::{
            accounts::r#async::thread_safe::Accounts, codex::r#async::thread_safe::Codex,
            connectors::r#async::thread_safe::Connectors, plugins::r#async::thread_safe::Plugins,
            ps::r#async::thread_safe::Ps, wham::r#async::thread_safe::Wham,
        };

        /// A master trait containing all modules that are available
        pub trait CodexApi: Accounts + Codex + Connectors + Plugins + Ps + Wham {}

        impl<T: Accounts + Codex + Connectors + Plugins + Ps + Wham> CodexApi for T {}
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use crate::{
            accounts::r#async::wasm_safe::Accounts, codex::r#async::wasm_safe::Codex,
            connectors::r#async::wasm_safe::Connectors, plugins::r#async::wasm_safe::Plugins,
            ps::r#async::wasm_safe::Ps, wham::r#async::wasm_safe::Wham,
        };

        /// A master trait containing all modules that are available
        pub trait CodexApi: Accounts + Codex + Connectors + Plugins + Ps + Wham {}

        impl<T: Accounts + Codex + Connectors + Plugins + Ps + Wham> CodexApi for T {}
    }
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
#[cfg(feature = "threaded")]
use wasm_not_send_sync::WasmNotSend;

// Creating a type for async functions
#[cfg(feature = "threaded")]
pub trait FutureNotSend: Future + WasmNotSend {}
#[cfg(feature = "threaded")]
impl<T: Future + WasmNotSend> FutureNotSend for T {}

/// Functions and types that will be common amongst all API calls
pub trait ApiCommon {
    /// The response type that all responses will be based on
    type Response;

    /// The error type that will occur on an API error
    type ApiError;
}
