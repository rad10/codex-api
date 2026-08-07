#[cfg(feature = "sync")]
pub mod blocking;

pub mod accounts;
pub mod client;
pub mod codex;
pub mod connectors;
pub mod error;
pub mod plugins;
pub mod ps;
pub mod response;
pub mod wham;

pub use reqwest::IntoUrl;

#[cfg(feature = "async")]
pub use codex_api_lib::r#async::CodexApi as AsyncCodexApi;

#[cfg(feature = "threaded")]
pub use codex_api_lib::r#async::{
    thread_safe::CodexApi as CodexApiThreadSafe, wasm_safe::CodexApi as CodexApiWasmSafe,
};

#[cfg(test)]
pub(crate) mod test {
    use bytes::Bytes;
    use httpmock::HttpMockResponse;

    #[track_caller]
    #[cfg(feature = "async")]
    pub(crate) fn mock_data_to_async_response(
        mock_data: HttpMockResponse,
    ) -> crate::response::ApiResponse {
        let http_response: http::Response<Bytes> = mock_data
            .try_into()
            .expect("Should convert into a response");
        reqwest::Response::from(http_response).into()
    }

    #[track_caller]
    #[cfg(feature = "sync")]
    pub(crate) fn mock_data_to_blocking_response(
        mock_data: HttpMockResponse,
    ) -> crate::response::BlockingApiResponse {
        let http_response: http::Response<Bytes> = mock_data
            .try_into()
            .expect("Should convert into a response");
        reqwest::blocking::Response::from(http_response).into()
    }
}
