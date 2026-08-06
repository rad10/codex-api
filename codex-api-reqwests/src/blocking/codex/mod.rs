use std::{
    io::{BufRead, BufReader},
    iter,
};

pub use codex_api_lib::codex::sync::{Codex, models, responses};
use codex_api_lib::{
    ApiCommon,
    codex::{ENDPOINT_MODELS, ENDPOINT_RESPONSES, MODULE_CODEX, ResponsesOptions},
};
use http::StatusCode;
use reqwest::{IntoUrl, blocking::Request};

use crate::{
    client::{
        blocking::CodexClient,
        traits::{CodexAccountId, CodexAuthorization},
    },
    codex::{CODEX_VERSION, response_stream, subagent_header},
    error::ParsingError,
    response::BlockingApiResponse,
};
pub use codex_api_types::codex::{ModelsResponse, ResponseEvent, ResponsesApiRequest};

pub mod analytics_events;

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
            .get(api_url)
            .bearer_auth(&self.authorization)
            .headers(headers)
            .json(&request)
            .build()
            .map_err(Into::into)
    }
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl + Clone> Codex
    for CodexClient<Auth, Acc, U>
{
    fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<ModelsResponse>,
    {
        // Creating API call
        let request_data = self.codex_models_request()?;

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
        // Creating API call
        let request_data = self.codex_responses_request(&request, options)?;

        // Calling API request
        self.client
            .execute(request_data)
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl TryFrom<BlockingApiResponse> for ModelsResponse {
    type Error = ParsingError;

    fn try_from(value: BlockingApiResponse) -> Result<Self, Self::Error> {
        value.deserialize_if_ok(StatusCode::OK)
    }
}

impl TryFrom<BlockingApiResponse> for Vec<ResponseEvent> {
    type Error = response_stream::ResponsesError;

    fn try_from(value: BlockingApiResponse) -> Result<Self, Self::Error> {
        // Split the full response into double lines

        let mut response_lines = BufReader::new(reqwest::blocking::Response::from(value)).lines();
        let reader = iter::from_fn(|| {
            let mut line_data = Vec::new();

            while let Some(line) = response_lines.next() {
                match line {
                    Ok(data) if data.is_empty() => break,
                    Ok(data) => line_data.push(data),
                    Err(e) => return Some(Err(e)),
                }
            }
            (!line_data.is_empty()).then(|| Ok(line_data.concat()))
        });

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
