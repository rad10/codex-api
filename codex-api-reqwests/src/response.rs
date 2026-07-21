//! This module handles the Response object that can be created

use http::{HeaderMap, StatusCode};
#[cfg(feature = "sync")]
use reqwest::blocking;
use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::ParsingError;


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

    /// Deserialized the response object into the desired model
    /// 
    /// This function will ignore the response status, so failure is high. If
    /// you want the response status to be taken accounted for, then you
    /// should use [`Self::deserialize_if_ok`]
    pub fn deserialize_data<U: DeserializeOwned>(self) -> Result<U, serde_json::Error> {
        serde_json::from_value(self.data)
    }

    /// Deserialized the response object into the desired model if the expected
    /// status code is triggered
    /// 
    /// This function will assume that the data will not deserialize properly
    /// is the status code is invalid. If you do not care about the response
    /// status, use [`Self::deserialize_data`] instead
    pub fn deserialize_if_ok<U: DeserializeOwned>(self, expected: StatusCode) -> Result<U, ParsingError> {
        if self.status_code != expected {
            Err(ParsingError::InvalidStatus(self.status_code))
        } else {
            serde_json::from_value(self.data).map_err(Into::into)
        }
    }
}

#[cfg(feature = "sync")]
impl TryFrom<blocking::Response> for ApiResponse {
    type Error = Error;

    fn try_from(value: blocking::Response) -> Result<Self, Self::Error> {
        Ok(Self { status_code: value.status(), headers: value.headers().clone(), data: value.json()? })
    }
}