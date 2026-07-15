use wasm_not_send_sync::WasmNotSend;

pub mod accounts;
pub mod codex;
pub mod connectors;
pub mod plugins;
pub mod ps;
pub mod wham;

/// A master trait containing all modules that are available
pub trait CodexApi {}

// blanket implement of api trait on all models that implement all traits
impl<T> CodexApi for T {}

// Creating a type for async functions
trait FutureNotSend: Future + WasmNotSend {}
impl<T: Future + WasmNotSend> FutureNotSend for T {}

/// Functions and types that will be common amongst all API calls
pub trait ApiCommon {
    /// The response type that all responses will be based on
    type Response;

    /// The error type that will occur on an API error
    type ApiError;
}