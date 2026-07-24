use std::io::{BufRead, BufReader};

#[cfg(feature = "sync")]
use codex_api_lib::codex::CodexSync;
use codex_api_lib::codex::{CodexSub, ENDPOINT_MODELS, ENDPOINT_RESPONSES, MODULE_CODEX};
#[cfg(feature = "async")]
use codex_api_lib::{AsyncTryFrom, AsyncTryInto, codex::CodexAsync};
use codex_api_types::codex::{SessionSource, SubAgentSource};
use http::{HeaderValue, StatusCode};
use reqwest::IntoUrl;

#[cfg(feature = "middleware")]
use crate::client::CodexMiddleware;
#[cfg(feature = "async")]
use crate::response::ApiResponse;
#[cfg(feature = "sync")]
use crate::{client::blocking, response::BlockingApiResponse};
use crate::{
    client::{
        CodexClient,
        traits::{CodexAccountId, CodexAuthorization},
    },
    error::ParsingError,
};

pub use codex_api_lib::codex::ResponsesOptions;
pub use codex_api_types::codex::{ModelsResponse, ResponseEvent, ResponsesApiRequest};

pub mod analytics_events;
pub mod response_stream;

const CODEX_VERSION: &'static str = "0.144.6";

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> CodexSub
    for CodexClient<Auth, Acc, U>
{
}
#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> CodexSub
    for CodexMiddleware<Auth, Acc, U>
{
}
#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> CodexSub
    for blocking::CodexClient<Auth, Acc, U>
{
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Clone + Sync>
    CodexAsync for CodexClient<Auth, Acc, U>
{
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<ModelsResponse>,
    {
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
        let request_data = self
            .client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .query(&[("client_version", CODEX_VERSION)])
            .build()?;

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
        let request_data = self
            .client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .json(&request)
            .build()?;

        // Calling API request
        self.client
            .execute(request_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Clone + Sync>
    CodexAsync for CodexMiddleware<Auth, Acc, U>
{
    async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<ModelsResponse>,
    {
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
        let request_data = self
            .client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .query(&[("client_version", CODEX_VERSION)])
            .build()?;

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
        let request_data = self
            .client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .json(&request)
            .build()?;

        // Calling API request
        self.client
            .execute(request_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization + Sync, Acc: CodexAccountId + Sync, U: IntoUrl + Clone + Sync>
    CodexSync for blocking::CodexClient<Auth, Acc, U>
{
    fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<ModelsResponse>,
    {
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
        let request_data = self
            .client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .query(&[("client_version", CODEX_VERSION)])
            .build()?;

        // Calling API request
        self.client
            .execute(request_data)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn codex_responses(
        &self,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<Vec<ResponseEvent>>,
    {
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
        let request_data = self
            .client
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .json(&request)
            .build()?;

        // Calling API request
        self.client
            .execute(request_data)
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

#[cfg(feature = "sync")]
impl TryFrom<BlockingApiResponse> for ModelsResponse {
    type Error = ParsingError;

    fn try_from(value: BlockingApiResponse) -> Result<Self, Self::Error> {
        value.deserialize_if_ok(StatusCode::OK)
    }
}

#[cfg(feature = "async")]
impl AsyncTryFrom<ApiResponse> for Vec<ResponseEvent> {
    type Error = response_stream::ResponsesError;

    async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
        CombineLines {
            inner: reqwest::Response::from(value)
                .text()
                .await?
                .lines()
                .map(|line| line.to_owned()),
            func: |line_iter| {
                let mut lines = Vec::new();

                while let Some(line) = line_iter.next().filter(|s| !s.is_empty()) {
                    lines.push(line);
                }
                (!lines.is_empty()).then(|| lines.concat())
            },
        }
        .map(|event| {
            event
                .parse()
                .map(|data: response_stream::StreamEvent| data.data)
                .and_then(response_stream::process_responses_event)
                .and_then(|processing| {
                    processing.ok_or(response_stream::ResponsesError::InvalidResponseStream)
                })
        })
        .collect::<Result<Vec<_>, _>>()
    }
}

#[cfg(feature = "sync")]
impl TryFrom<BlockingApiResponse> for Vec<ResponseEvent> {
    type Error = response_stream::ResponsesError;

    fn try_from(value: BlockingApiResponse) -> Result<Self, Self::Error> {
        // Split the full response into double lines

        let reader = CombineLines {
            inner: BufReader::new(reqwest::blocking::Response::from(value)).lines(),
            func: |lines: &mut std::io::Lines<BufReader<reqwest::blocking::Response>>| {
                let mut line_data = Vec::new();

                while let Some(line) = lines.next() {
                    match line {
                        Ok(data) if data.is_empty() => break,
                        Ok(data) => line_data.push(data),
                        Err(e) => return Some(Err(e)),
                    }
                }
                (!line_data.is_empty()).then(|| Ok(line_data.concat()))
            },
        };

        reader
            .map(|event| match event {
                Ok(event) => event.parse(),
                Err(err) => Err(response_stream::ResponsesError::IO(err)),
            })
            .map(|event_data| {
                event_data
                    .map(|event: response_stream::StreamEvent| event.data)
                    .and_then(response_stream::process_responses_event)
                    .and_then(|processing| {
                        processing.ok_or(response_stream::ResponsesError::InvalidResponseStream)
                    })
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

struct CombineLines<I, U, F: FnMut(&mut I) -> Option<U>> {
    inner: I,
    func: F,
}

impl<I, U, F: FnMut(&mut I) -> Option<U>> Iterator for CombineLines<I, U, F> {
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        (self.func)(&mut self.inner)
    }
}

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
