use codex_api_lib::ApiCommon;
use codex_api_lib::codex::ResponsesOptions;
#[cfg(feature = "async")]
use codex_api_lib::codex::{ENDPOINT_MODELS, ENDPOINT_RESPONSES, MODULE_CODEX, r#async};
#[cfg(feature = "async")]
use codex_api_lib::{AsyncTryFrom, AsyncTryInto};
use codex_api_types::codex::{ModelInfo, ResponsesApiRequest};
use codex_api_types::codex::{SessionSource, SubAgentSource};
#[cfg(feature = "async")]
use futures::{AsyncBufReadExt, StreamExt, TryStreamExt, stream::try_unfold};
use http::{HeaderValue, StatusCode};
#[cfg(feature = "async")]
use reqwest::IntoUrl;
use reqwest::Request;

#[cfg(feature = "middleware")]
use crate::client::CodexMiddleware;
#[cfg(feature = "async")]
use crate::response::ApiResponse;
#[cfg(feature = "async")]
use crate::{
    client::{
        CodexClient,
        traits::{CodexAccountId, CodexAuthorization},
    },
    error::ParsingError,
};

#[cfg(feature = "async")]
pub use codex_api_lib::codex::{
    r#async::{Codex, models, responses},
};
#[cfg(feature = "async")]
pub use codex_api_types::codex::{ModelsResponse, ResponseEvent};

pub mod analytics_events;
pub mod response_stream;

pub(crate) const CODEX_VERSION: &'static str = "0.146.1";

/// Provides the option to collect the request without sending it yet
///
/// This can be useful if you wish to alter or edit the request before sending it
pub trait CodexRequest {
    /// Contains the errors that can occur during build
    type BuildError;

    fn codex_models_request(&self) -> Result<Request, Self::BuildError>;

    fn codex_responses_request(
        &self,
        request: &ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Request, Self::BuildError>;
}

pub fn models_request<R: CodexRequest>(client: &R) -> Result<Request, R::BuildError> {
    client.codex_models_request()
}

pub fn responses_request<R: CodexRequest>(
    client: &R,
    request: &ResponsesApiRequest,
    options: ResponsesOptions,
) -> Result<Request, R::BuildError> {
    client.codex_responses_request(request, options)
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone> CodexRequest
    for CodexClient<Auth, Acc, U>
{
    type BuildError = <Self as ApiCommon>::ApiError;

    fn codex_models_request(&self) -> Result<Request, Self::BuildError> {
        // Creating URL
        let api_url = self
            .endpoint
            .clone()
            .into_url()?
            .join([MODULE_CODEX, ENDPOINT_MODELS].join("/").as_str())?;

        let mut headers = self.extra_headers.clone();
        if let Some(account_id) = self.account_id.as_ref() {
            account_id.add_account_header(&mut headers);
        }
        // Creating API call
        self.client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .query(&[("client_version", CODEX_VERSION)])
            .build()
            .map_err(Into::into)
    }

    fn codex_responses_request(
        &self,
        request: &ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Request, Self::BuildError> {
        // Creating URL
        let api_url = self
            .endpoint
            .clone()
            .into_url()?
            .join([MODULE_CODEX, ENDPOINT_RESPONSES].join("/").as_str())?;

        let mut headers = self.extra_headers.clone();
        headers.extend(options.extra_headers);
        if let Some(account_id) = self.account_id.as_ref() {
            account_id.add_account_header(&mut headers);
        }
        if let Some(thread_id) = options.thread_id.and_then(|thread| thread.parse().ok()) {
            headers.insert("x-client-request-id", thread_id);
        }
        if let Some(subagent) = options.session_source.and_then(subagent_header) {
            headers.insert("x-openai-subagent", subagent);
        }
        // Creating API call
        self.client
            .post(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .json(&request)
            .build()
            .map_err(Into::into)
    }
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone> Codex
    for CodexClient<Auth, Acc, U>
{
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<ModelsResponse>,
    {
        // Creating API call
        let request_data = self.codex_models_request()?;

        // Calling API request
        self.client
            .execute(request_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    async fn codex_responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<Vec<ResponseEvent>>,
    {
        // Creating API call
        let request_data = self.codex_responses_request(&request, options)?;

        // Calling API request
        self.client
            .execute(request_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone> CodexRequest
    for CodexMiddleware<Auth, Acc, U>
{
    type BuildError = <Self as ApiCommon>::ApiError;

    fn codex_models_request(&self) -> Result<Request, Self::BuildError> {
        // Creating URL
        let api_url = self
            .endpoint
            .clone()
            .into_url()?
            .join([MODULE_CODEX, ENDPOINT_MODELS].join("/").as_str())?;

        let mut headers = self.extra_headers.clone();
        if let Some(account_id) = self.account_id.as_ref() {
            account_id.add_account_header(&mut headers);
        }
        // Creating API call
        self.client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .query(&[("client_version", CODEX_VERSION)])
            .build()
            .map_err(Into::into)
    }

    fn codex_responses_request(
        &self,
        request: &ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Request, Self::BuildError> {
        // Creating URL
        let api_url = self
            .endpoint
            .clone()
            .into_url()?
            .join([MODULE_CODEX, ENDPOINT_RESPONSES].join("/").as_str())?;

        let mut headers = self.extra_headers.clone();
        headers.extend(options.extra_headers);
        if let Some(account_id) = self.account_id.as_ref() {
            account_id.add_account_header(&mut headers);
        }
        if let Some(thread_id) = options.thread_id.and_then(|thread| thread.parse().ok()) {
            headers.insert("x-client-request-id", thread_id);
        }
        if let Some(subagent) = options.session_source.and_then(subagent_header) {
            headers.insert("x-openai-subagent", subagent);
        }
        // Creating API call
        self.client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .json(&request)
            .build()
            .map_err(Into::into)
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone> Codex
    for CodexMiddleware<Auth, Acc, U>
{
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<ModelsResponse>,
    {
        // Creating API call
        let request_data = self.codex_models_request()?;

        // Calling API request
        self.client
            .execute(request_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    async fn codex_responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<Vec<ResponseEvent>>,
    {
        // Creating API call
        let request_data = self.codex_responses_request(&request, options)?;

        // Calling API request
        self.client
            .execute(request_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[cfg(feature = "async")]
impl AsyncTryFrom<ApiResponse> for ModelsResponse {
    type Error = ParsingError;

    async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
        value.deserialize_if_ok(StatusCode::OK).await
    }
}

#[cfg(feature = "async")]
impl AsyncTryFrom<ApiResponse> for Vec<ModelInfo> {
    type Error = ParsingError;

    async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
        ModelsResponse::async_try_from(value).await.map(Into::into)
    }
}

#[cfg(feature = "async")]
async fn api_response_to_response_event(
    value: ApiResponse,
) -> Result<Vec<ResponseEvent>, response_stream::ResponsesError> {
    try_unfold(
        reqwest::Response::from(value)
            .bytes_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            .into_async_read()
            .lines(),
        async |mut line_stream| {
            let mut event_string = Vec::new();

            while let Some(line) = line_stream.next().await {
                let clean_data = line?;
                if clean_data.is_empty() {
                    break;
                } else {
                    event_string.push(clean_data);
                }
            }
            Ok((!event_string.is_empty()).then(|| (event_string.concat(), line_stream)))
        },
    )
    .map_err(|e| response_stream::ResponsesError::IO(e))
    .and_then(async |s| s.parse())
    .map_ok(|data: response_stream::StreamEvent| data.data)
    .and_then(async |event| response_stream::process_responses_event(event))
    .and_then(async |processing| {
        processing.ok_or(response_stream::ResponsesError::InvalidResponseStream)
    })
    .try_collect()
    .await
}

#[cfg(feature = "async")]
impl AsyncTryFrom<ApiResponse> for Vec<ResponseEvent> {
    type Error = response_stream::ResponsesError;

    async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
        api_response_to_response_event(value).await
    }
}

pub(crate) fn subagent_header(source: SessionSource) -> Option<HeaderValue> {
    match source {
        SessionSource::SubAgent(SubAgentSource::Review) => Some(HeaderValue::from_static("review")),
        SessionSource::SubAgent(SubAgentSource::Compact) => {
            Some(HeaderValue::from_static("compact"))
        }
        SessionSource::SubAgent(SubAgentSource::MemoryConsolidation) => {
            Some(HeaderValue::from_static("memory_consolidation"))
        }
        SessionSource::SubAgent(SubAgentSource::ThreadSpawn { .. }) => {
            Some(HeaderValue::from_static("collab_spawn"))
        }
        SessionSource::SubAgent(SubAgentSource::Other(label)) => label.parse().ok(),
        _ => None,
    }
}

#[cfg(feature = "threaded")]
pub mod thread_safe {
    #[cfg(feature = "middleware")]
    use super::CodexMiddleware;
    use super::{
        ApiResponse, AsyncTryInto, CodexAccountId, CodexAuthorization, CodexClient, CodexRequest,
        IntoUrl, ModelsResponse, ParsingError, ResponseEvent, ResponsesApiRequest,
        ResponsesOptions, StatusCode, api_response_to_response_event, r#async, response_stream,
    };

    pub use r#async::thread_safe::{Codex, models, responses};
    use async_from::thread_safe::AsyncTryFromThreadSafe;

    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Clone + Sync>
        Codex for CodexClient<Auth, Acc, U>
    {
        async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<ModelsResponse>,
        {
            // Creating API call
            let request_data = self.codex_models_request()?;

            // Calling API request
            self.client
                .execute(request_data)
                .await
                .map(Into::into)
                .map_err(Into::into)
        }

        async fn codex_responses(
            &self,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<Vec<ResponseEvent>>,
        {
            // Creating API call
            let request_data = self.codex_responses_request(&request, options)?;

            // Calling API request
            self.client
                .execute(request_data)
                .await
                .map(Into::into)
                .map_err(Into::into)
        }
    }

    #[cfg(feature = "middleware")]
    impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Clone + Sync>
        Codex for CodexMiddleware<Auth, Acc, U>
    {
        async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<ModelsResponse>,
        {
            // Creating API call
            let request_data = self.codex_models_request()?;

            // Calling API request
            self.client
                .execute(request_data)
                .await
                .map(Into::into)
                .map_err(Into::into)
        }

        async fn codex_responses(
            &self,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<Vec<ResponseEvent>>,
        {
            // Creating API call
            let request_data = self.codex_responses_request(&request, options)?;

            // Calling API request
            self.client
                .execute(request_data)
                .await
                .map(Into::into)
                .map_err(Into::into)
        }
    }

    impl AsyncTryFromThreadSafe<ApiResponse> for ModelsResponse {
        type Error = ParsingError;

        async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
            value.deserialize_if_ok(StatusCode::OK).await
        }
    }

    impl AsyncTryFromThreadSafe<ApiResponse> for Vec<ResponseEvent> {
        type Error = response_stream::ResponsesError;

        async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
            api_response_to_response_event(value).await
        }
    }

    #[cfg(test)]
    #[cfg(not(target_arch = "wasm32"))]
    mod test {
        use crate::codex::test::build_model_async_response;

        use super::ModelsResponse;
        use async_from::thread_safe::AsyncTryFromThreadSafe;

        #[tokio::test]
        async fn test_threaded_async_conversions() {
            let api_response = build_model_async_response();

            // Testing async conversion
            let model_data = ModelsResponse::async_try_from_threaded(api_response)
                .await
                .expect("model should convert as expected");

            assert!(!model_data.models.is_empty());
        }
    }
}

#[cfg(feature = "threaded")]
pub mod wasm_safe {
    pub use super::r#async::wasm_safe::{Codex, models, responses};

    #[cfg(test)]
    mod test {
        use async_from::wasm_safe::AsyncTryFromWasmSafe;

        use super::super::{ModelsResponse, test::build_model_async_response};

        #[tokio::test]
        async fn test_threaded_async_conversions() {
            let api_response = build_model_async_response();

            // Testing async conversion
            let model_data = ModelsResponse::async_try_from_wasm(api_response)
                .await
                .expect("model should convert as expected");

            assert!(!model_data.models.is_empty());
        }
    }
}

#[cfg(test)]
#[cfg(feature = "async")]
mod test {
    use crate::test::mock_data_to_async_response;

    use super::{ApiResponse, AsyncTryFrom, ModelsResponse};
    use httpmock::HttpMockResponse;

    pub(super) fn build_model_async_response() -> ApiResponse {
        // Creating fake response to convert into
        let mock_response = HttpMockResponse::builder()
            .status(200)
            .body(
                "
        fasdfasd
        ",
            )
            .build();

        mock_data_to_async_response(mock_response)
    }

    #[tokio::test]
    async fn test_model_async_conversions() {
        let api_response = build_model_async_response();

        // Testing async conversion
        let model_data = ModelsResponse::async_try_from(api_response)
            .await
            .expect("model should convert as expected");

        assert!(!model_data.models.is_empty());
    }
}
