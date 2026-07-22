//! This module handles the Response object that can be created

use std::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

use http::StatusCode;
#[cfg(feature = "sync")]
use reqwest::blocking;
use reqwest::{Error, Response};
use serde::de::DeserializeOwned;

use crate::error::ParsingError;

pub struct ApiResponse(Response);

pub struct BlockingApiResponse(blocking::Response);

impl ApiResponse {
    /// Deserialized the response object into the desired model
    ///
    /// This function will ignore the response status, so failure is high. If
    /// you want the response status to be taken accounted for, then you
    /// should use [`Self::deserialize_if_ok`]
    pub async fn deserialize_data<U: DeserializeOwned>(self) -> Result<U, Error> {
        self.0.json().await
    }

    /// Deserialized the response object into the desired model if the expected
    /// status code is triggered
    ///
    /// This function will assume that the data will not deserialize properly
    /// is the status code is invalid. If you do not care about the response
    /// status, use [`Self::deserialize_data`] instead
    pub async fn deserialize_if_ok<U: DeserializeOwned>(
        self,
        expected: StatusCode,
    ) -> Result<U, ParsingError> {
        if self.status() != expected {
            Err(ParsingError::InvalidStatus(self.status()))
        } else {
            self.deserialize_data().await.map_err(Into::into)
        }
    }
}

impl BlockingApiResponse {
    /// Deserialized the response object into the desired model
    ///
    /// This function will ignore the response status, so failure is high. If
    /// you want the response status to be taken accounted for, then you
    /// should use [`Self::deserialize_if_ok`]
    pub fn deserialize_data<U: DeserializeOwned>(self) -> Result<U, Error> {
        self.0.json()
    }

    /// Deserialized the response object into the desired model if the expected
    /// status code is triggered
    ///
    /// This function will assume that the data will not deserialize properly
    /// is the status code is invalid. If you do not care about the response
    /// status, use [`Self::deserialize_data`] instead
    pub fn deserialize_if_ok<U: DeserializeOwned>(
        self,
        expected: StatusCode,
    ) -> Result<U, ParsingError> {
        if self.status() != expected {
            Err(ParsingError::InvalidStatus(self.status()))
        } else {
            self.deserialize_data().map_err(Into::into)
        }
    }
}

impl From<Response> for ApiResponse {
    fn from(value: Response) -> Self {
        Self(value)
    }
}
impl From<ApiResponse> for Response {
    fn from(value: ApiResponse) -> Self {
        value.0
    }
}
impl Deref for ApiResponse {
    type Target = Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ApiResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<blocking::Response> for BlockingApiResponse {
    fn from(value: blocking::Response) -> Self {
        Self(value)
    }
}
impl From<BlockingApiResponse> for blocking::Response {
    fn from(value: BlockingApiResponse) -> Self {
        value.0
    }
}
impl Deref for BlockingApiResponse {
    type Target = blocking::Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BlockingApiResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Response> for ApiResponse {
    fn as_ref(&self) -> &Response {
        &self.0
    }
}

impl AsMut<Response> for ApiResponse {
    fn as_mut(&mut self) -> &mut Response {
        &mut self.0
    }
}

impl Borrow<Response> for ApiResponse {
    fn borrow(&self) -> &Response {
        &self.0
    }
}

impl BorrowMut<Response> for ApiResponse {
    fn borrow_mut(&mut self) -> &mut Response {
        &mut self.0
    }
}

impl AsRef<blocking::Response> for BlockingApiResponse {
    fn as_ref(&self) -> &blocking::Response {
        &self.0
    }
}

impl AsMut<blocking::Response> for BlockingApiResponse {
    fn as_mut(&mut self) -> &mut blocking::Response {
        &mut self.0
    }
}

impl Borrow<blocking::Response> for BlockingApiResponse {
    fn borrow(&self) -> &blocking::Response {
        &self.0
    }
}

impl BorrowMut<blocking::Response> for BlockingApiResponse {
    fn borrow_mut(&mut self) -> &mut blocking::Response {
        &mut self.0
    }
}