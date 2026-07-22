use std::io::{BufRead, BufReader};

#[cfg(feature = "sync")]
use codex_api_lib::codex::CodexSync;
use codex_api_lib::codex::{CodexSub, ENDPOINT_MODELS, ENDPOINT_RESPONSES, MODULE_CODEX};
#[cfg(feature = "async")]
use codex_api_lib::{AsyncTryFrom, AsyncTryInto, codex::CodexAsync};
use http::StatusCode;
use reqwest::IntoUrl;

#[cfg(feature = "middleware")]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
#[cfg(feature = "async")]
use crate::response::ApiResponse;
use crate::{
    client::{
        CodexClient,
        traits::{CodexAccountId, CodexAuthorization},
    },
    error::ParsingError,
    response::BlockingApiResponse,
};

pub mod analytics_events;
mod response_stream;

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
        Self::Response: AsyncTryInto<codex_api_types::codex::ModelsResponse>,
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
        request: codex_api_types::codex::ResponsesApiRequest,
        options: codex_api_lib::codex::ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<Vec<codex_api_types::codex::ResponseEvent>>,
    {
        // Creating URL
        let api_url = self
            .endpoint
            .clone()
            .into_url()?
            .join([MODULE_CODEX, ENDPOINT_RESPONSES].join("/").as_str())?;

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
        Self::Response: AsyncTryInto<codex_api_types::codex::ModelsResponse>,
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
        request: codex_api_types::codex::ResponsesApiRequest,
        options: codex_api_lib::codex::ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: AsyncTryInto<Vec<codex_api_types::codex::ResponseEvent>>,
    {
        // Creating URL
        let api_url = self
            .endpoint
            .clone()
            .into_url()?
            .join([MODULE_CODEX, ENDPOINT_RESPONSES].join("/").as_str())?;

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
        Self::Response: TryInto<codex_api_types::codex::ModelsResponse>,
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
        request: codex_api_types::codex::ResponsesApiRequest,
        options: codex_api_lib::codex::ResponsesOptions,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<Vec<codex_api_types::codex::ResponseEvent>>,
    {
        // Creating URL
        let api_url = self
            .endpoint
            .clone()
            .into_url()?
            .join([MODULE_CODEX, ENDPOINT_RESPONSES].join("/").as_str())?;

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
            .build()?;

        // Calling API request
        self.client
            .execute(request_data)
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[cfg(feature = "async")]
impl AsyncTryFrom<ApiResponse> for codex_api_types::codex::ModelsResponse {
    type Error = ParsingError;

    async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
        value.deserialize_if_ok(StatusCode::OK).await
    }
}

impl TryFrom<BlockingApiResponse> for codex_api_types::codex::ModelsResponse {
    type Error = ParsingError;

    fn try_from(value: BlockingApiResponse) -> Result<Self, Self::Error> {
        value.deserialize_if_ok(StatusCode::OK)
    }
}

impl AsyncTryFrom<ApiResponse> for Vec<codex_api_types::codex::ResponseEvent> {
    type Error = response_stream::ApiError;

    async fn try_from(value: ApiResponse) -> Result<Self, Self::Error> {
        CombineLines(value.text().await?.lines().map(|line| line.to_owned()))
            .map(|event| {
                event
                    .parse()
                    .and_then(response_stream::process_responses_event)
                    .and_then(|processing| {
                        processing.ok_or(response_stream::ApiError::InvalidResponseStream)
                    })
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

impl TryFrom<BlockingApiResponse> for Vec<codex_api_types::codex::ResponseEvent> {
    type Error = response_stream::ApiError;

    fn try_from(mut value: BlockingApiResponse) -> Result<Self, Self::Error> {
        // Split the full response into double lines

        let mut reader =
            CombineLines(BufReader::new(reqwest::blocking::Response::from(value)).lines());

        reader
            .map(|event| match event {
                Ok(event) => event.parse(),
                Err(err) => Err(response_stream::ApiError::IO(err)),
            })
            .map(|event_data| {
                event_data
                    .and_then(response_stream::process_responses_event)
                    .and_then(|processing| {
                        processing.ok_or(response_stream::ApiError::InvalidResponseStream)
                    })
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

struct CombineLines<I>(I);

impl<I: Iterator<Item = String>> Iterator for CombineLines<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::new();

        while let Some(line) = self.0.next().filter(|s| !s.is_empty()) {
            lines.push(line);
        }
        (!lines.is_empty()).then(|| lines.concat())
    }
}

impl<I: Iterator<Item = Result<String, std::io::Error>>> Iterator for CombineLines<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::new();

        while let Some(line) = self.0.next() {
            match line {
                Ok(data) if data.is_empty() => break,
                Ok(data) => lines.push(data),
                Err(e) => return Some(Err(e)),
            }
        }
        (!lines.is_empty()).then(|| Ok(lines.concat()))
    }
}
