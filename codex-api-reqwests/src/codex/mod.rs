use codex_api_lib::ApiCommon;
#[cfg(feature = "async")]
use codex_api_lib::codex::{ENDPOINT_MODELS, ENDPOINT_RESPONSES, MODULE_CODEX, r#async};
#[cfg(feature = "async")]
use codex_api_lib::{AsyncTryFrom, AsyncTryInto};
#[cfg(feature = "async")]
use codex_api_types::codex::{SessionSource, SubAgentSource};
#[cfg(feature = "async")]
use futures::{AsyncBufReadExt, StreamExt, TryStreamExt, stream::try_unfold};
#[cfg(feature = "async")]
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
    ResponsesOptions,
    r#async::{Codex, models, responses},
};
#[cfg(feature = "async")]
pub use codex_api_types::codex::{ModelsResponse, ResponseEvent, ResponsesApiRequest};

pub mod analytics_events;
pub mod response_stream;

pub(crate) const CODEX_VERSION: &'static str = "0.144.6";

#[cfg(feature = "async")]
fn models_params<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone>(
    client: &CodexClient<Auth, Acc, U>,
) -> Result<Request, <CodexClient<Auth, Acc, U> as ApiCommon>::ApiError> {
    // Creating URL
    let api_url = client
        .endpoint
        .clone()
        .into_url()?
        .join([MODULE_CODEX, ENDPOINT_MODELS].join("/").as_str())?;

    let mut headers = client.extra_headers.clone();
    if let Some(account_id) = client.account_id.as_ref() {
        account_id.add_account_header(&mut headers);
    }
    // Creating API call
    client
        .client
        .get(api_url)
        .bearer_auth(&client.authorization)
        .headers(headers)
        .query(&[("client_version", CODEX_VERSION)])
        .build()
        .map_err(Into::into)
}

#[cfg(feature = "async")]
fn responses_params<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone>(
    client: &CodexClient<Auth, Acc, U>,
    request: &ResponsesApiRequest,
    options: ResponsesOptions,
) -> Result<Request, <CodexClient<Auth, Acc, U> as ApiCommon>::ApiError> {
    // Creating URL
    let api_url = client
        .endpoint
        .clone()
        .into_url()?
        .join([MODULE_CODEX, ENDPOINT_RESPONSES].join("/").as_str())?;

    let mut headers = client.extra_headers.clone();
    headers.extend(options.extra_headers);
    if let Some(account_id) = client.account_id.as_ref() {
        account_id.add_account_header(&mut headers);
    }
    if let Some(thread_id) = options.thread_id.and_then(|thread| thread.parse().ok()) {
        headers.insert("x-client-request-id", thread_id);
    }
    if let Some(subagent) = options.session_source.and_then(subagent_header) {
        headers.insert("x-openai-subagent", subagent);
    }
    // Creating API call
    client
        .client
        .get(api_url)
        .bearer_auth(&client.authorization)
        .headers(headers)
        .json(&request)
        .build()
        .map_err(Into::into)
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
        let request_data = models_params(self)?;

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
        let request_data = responses_params(self, &request, options)?;

        // Calling API request
        self.client
            .execute(request_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
fn models_params_middleware<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone>(
    client: &CodexMiddleware<Auth, Acc, U>,
) -> Result<Request, <CodexMiddleware<Auth, Acc, U> as ApiCommon>::ApiError> {
    // Creating URL
    let api_url = client
        .endpoint
        .clone()
        .into_url()?
        .join([MODULE_CODEX, ENDPOINT_MODELS].join("/").as_str())?;

    let mut headers = client.extra_headers.clone();
    if let Some(account_id) = client.account_id.as_ref() {
        account_id.add_account_header(&mut headers);
    }
    // Creating API call
    client
        .client
        .get(api_url)
        .bearer_auth(&client.authorization)
        .headers(headers)
        .query(&[("client_version", CODEX_VERSION)])
        .build()
        .map_err(Into::into)
}

#[cfg(all(feature = "async", feature = "middleware"))]
fn responses_params_middleware<
    Auth: CodexAuthorization,
    Acc: CodexAccountId,
    U: IntoUrl + Clone,
>(
    client: &CodexMiddleware<Auth, Acc, U>,
    request: &ResponsesApiRequest,
    options: ResponsesOptions,
) -> Result<Request, <CodexMiddleware<Auth, Acc, U> as ApiCommon>::ApiError> {
    // Creating URL
    let api_url = client
        .endpoint
        .clone()
        .into_url()?
        .join([MODULE_CODEX, ENDPOINT_RESPONSES].join("/").as_str())?;

    let mut headers = client.extra_headers.clone();
    headers.extend(options.extra_headers);
    if let Some(account_id) = client.account_id.as_ref() {
        account_id.add_account_header(&mut headers);
    }
    if let Some(thread_id) = options.thread_id.and_then(|thread| thread.parse().ok()) {
        headers.insert("x-client-request-id", thread_id);
    }
    if let Some(subagent) = options.session_source.and_then(subagent_header) {
        headers.insert("x-openai-subagent", subagent);
    }
    // Creating API call
    client
        .client
        .get(api_url)
        .bearer_auth(&client.authorization)
        .headers(headers)
        .json(&request)
        .build()
        .map_err(Into::into)
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
        let request_data = models_params_middleware(self)?;

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
        let request_data = responses_params_middleware(self, &request, options)?;

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

#[cfg(feature = "async")]
fn subagent_header(source: SessionSource) -> Option<HeaderValue> {
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
    use super::{
        ApiResponse, AsyncTryInto, CodexAccountId, CodexAuthorization, CodexClient, IntoUrl,
        ModelsResponse, ParsingError, ResponseEvent, ResponsesApiRequest, ResponsesOptions,
        StatusCode, api_response_to_response_event, r#async, models_params, response_stream,
        responses_params,
    };
    #[cfg(feature = "middleware")]
    use super::{CodexMiddleware, models_params_middleware, responses_params_middleware};

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
            let request_data = models_params(self)?;

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
            let request_data = responses_params(self, &request, options)?;

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
            let request_data = models_params_middleware(self)?;

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
            let request_data = responses_params_middleware(self, &request, options)?;

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
}

#[cfg(feature = "threaded")]
pub mod wasm_safe {
    pub use super::r#async::wasm_safe::{Codex, models, responses};
}
