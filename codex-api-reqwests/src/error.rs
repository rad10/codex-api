use std::{error::Error, fmt::Display};

use http::StatusCode;

/// Describes all Codex Errors that can occur on the API side
#[derive(Debug)]
pub enum ApiError {
    /// URL failed to parse properly
    Endpoint(url::ParseError),
    /// Reqwest had an internal error
    Reqwest(reqwest::Error),
}

/// Describes all Codex Errors that can occur on the API side
#[cfg(feature = "middleware")]
#[derive(Debug)]
pub enum MiddlewareError {
    /// URL failed to parse properly
    Endpoint(url::ParseError),
    /// Reqwest had an internal error
    Middleware(reqwest_middleware::Error),
}

/// Provides an error that is used for parsing model values from responses
#[derive(Debug)]
pub enum ParsingError {
    /// The response gave an unexpected status code
    InvalidStatus(StatusCode),
    /// The body gave invalid data and did not deserialize properly
    FailedDeserialization(reqwest::Error),
}

impl From<url::ParseError> for ApiError {
    fn from(value: url::ParseError) -> Self {
        Self::Endpoint(value)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Endpoint(parse_error) => {
                write!(f, "failed to create endpoint url: {parse_error}")
            }
            ApiError::Reqwest(error) => error.fmt(f),
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApiError::Endpoint(parse_error) => Some(parse_error),
            ApiError::Reqwest(error) => Some(error),
        }
    }
}

#[cfg(feature = "middleware")]
impl From<url::ParseError> for MiddlewareError {
    fn from(value: url::ParseError) -> Self {
        Self::Endpoint(value)
    }
}

#[cfg(feature = "middleware")]
impl From<reqwest::Error> for MiddlewareError {
    fn from(value: reqwest::Error) -> Self {
        Self::Middleware(reqwest_middleware::Error::Reqwest(value))
    }
}

#[cfg(feature = "middleware")]
impl From<reqwest_middleware::Error> for MiddlewareError {
    fn from(value: reqwest_middleware::Error) -> Self {
        Self::Middleware(value)
    }
}

#[cfg(feature = "middleware")]
impl Display for MiddlewareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiddlewareError::Endpoint(parse_error) => {
                write!(f, "failed to create endpoint url: {parse_error}")
            }
            MiddlewareError::Middleware(error) => error.fmt(f),
        }
    }
}

#[cfg(feature = "middleware")]
impl Error for MiddlewareError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MiddlewareError::Endpoint(parse_error) => Some(parse_error),
            MiddlewareError::Middleware(error) => Some(error),
        }
    }
}

impl From<StatusCode> for ParsingError {
    fn from(value: StatusCode) -> Self {
        Self::InvalidStatus(value)
    }
}

impl From<reqwest::Error> for ParsingError {
    fn from(value: reqwest::Error) -> Self {
        Self::FailedDeserialization(value)
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidStatus(status_code) => {
                write!(f, "received unexpected status code {status_code}")
            }
            ParsingError::FailedDeserialization(error) => {
                write!(f, "failed to deserialize object: {error}")
            }
        }
    }
}

impl Error for ParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParsingError::InvalidStatus(_status_code) => None,
            ParsingError::FailedDeserialization(error) => Some(error),
        }
    }
}
