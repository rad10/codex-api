#[cfg(feature = "sync")]
use codex_api_lib::codex::CodexSync;
use codex_api_lib::{
    codex::{CodexSub, ENDPOINT_MODELS, ENDPOINT_RESPONSES, MODULE_CODEX},
};
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
        Self::Response: AsyncTryInto<String>,
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
        Self::Response: AsyncTryInto<String>,
    {
        todo!()
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
        Self::Response: TryInto<String>,
    {
        todo!()
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
