//! This module handles the Response object that can be created

use http::{HeaderMap, StatusCode};
use reqwest::{Error, Response, blocking};
use serde_json::Value;


/// The generic default response all API calls will make
#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub data: Value,
}
    
impl ApiResponse {
    /// Converts a Reqwest response into an API response
    pub async fn from_response(response: Response) -> Result<Self, Error> {
        Ok(Self { status_code: response.status(), headers: response.headers().clone(), data: response.json().await? })
    }
    /// Converts a Reqwest response into an API response
    /// 
    /// Fails if the response status code is an error
    pub async fn from_response_with_fail_on_error(response: Response) -> Result<Self, Error> {
        let response = response.error_for_status()?;
        Self::from_response(response).await
    }

    /// Returns if the response was an error
    pub fn is_error_response(&self) -> bool {
        self.status_code.is_client_error() || self.status_code.is_server_error()
    }
}

#[cfg(feature = "sync")]
impl TryFrom<blocking::Response> for ApiResponse {
    type Error = Error;

    fn try_from(value: blocking::Response) -> Result<Self, Self::Error> {
        Ok(Self { status_code: value.status(), headers: value.headers().clone(), data: value.json()? })
    }
}