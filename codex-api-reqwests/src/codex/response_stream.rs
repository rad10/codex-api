use std::{fmt::Display, str::FromStr, time::Duration};

use codex_api_types::{
    codex::{
        ResponseEvent,
        response_stream_event::{Error, ResponseCompleted, ResponsesStreamEvent},
    },
    response_item::ResponseItem,
};
use serde_json::Value;

pub(super) struct StreamEvent {
    /// Contains the contents in "event"
    event: String,
    /// Contains the contents in "data"
    pub(super) data: ResponsesStreamEvent,
}

impl FromStr for StreamEvent {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let event = lines
            .next()
            .and_then(|line| line.strip_prefix("event: "))
            .ok_or(ApiError::InvalidResponseStream)?
            .trim()
            .to_string();
        let data = lines
            .next()
            .and_then(|line| line.strip_prefix("data: "))
            .and_then(|value| serde_json::from_str(value).ok())
            .ok_or(ApiError::InvalidResponseStream)?;
        Ok(Self { event, data })
    }
}

#[derive(Debug)]
pub enum ApiError {
    IO(std::io::Error),
    ReqwestError(reqwest::Error),
    InvalidResponseStream,
    Stream(String),
    ContextWindowExceeded,
    QuotaExceeded,
    UsageNotIncluded,
    Retryable {
        message: String,
        delay: Option<Duration>,
    },
    RateLimit(String),
    InvalidRequest {
        message: String,
    },
    CyberPolicy {
        message: String,
    },
    ServerOverloaded,
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Stream(s) => write!(f, "stream error: {s}"),
            ApiError::ContextWindowExceeded => f.write_str("context window exceeded"),
            ApiError::QuotaExceeded => f.write_str("quota exceeded"),
            ApiError::UsageNotIncluded => f.write_str("usage not included"),
            ApiError::Retryable { message, delay: _ } => write!(f, "retryable error: {message}"),
            ApiError::RateLimit(s) => write!(f, "rate limit: {s}"),
            ApiError::InvalidRequest { message } => write!(f, "invalid request: {message}"),
            ApiError::CyberPolicy { message } => write!(f, "cyber policy: {message}"),
            ApiError::ServerOverloaded => f.write_str("server overloaded"),
            ApiError::ReqwestError(error) => error.fmt(f),
            ApiError::InvalidResponseStream => f.write_str("an invalid response was given that failed to parse"),
            ApiError::IO(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReqwestError(error) => Some(error),
            Self::IO(error) => Some(error),
            _ => None,
        }
    }
}

pub fn process_responses_event(
    event: ResponsesStreamEvent,
) -> std::result::Result<Option<ResponseEvent>, ApiError> {
    match event.kind.as_str() {
        "response.output_item.done" => {
            if let Some(item_val) = event.item {
                if let Ok(item) = serde_json::from_value::<ResponseItem>(item_val) {
                    return Ok(Some(ResponseEvent::OutputItemDone(item)));
                }
            }
        }
        "response.output_text.delta" => {
            if let Some(delta) = event.delta {
                return Ok(Some(ResponseEvent::OutputTextDelta(delta)));
            }
        }
        "response.custom_tool_call_input.delta" => {
            if let (Some(delta), Some(item_id)) =
                (event.delta, event.item_id.clone().or(event.call_id.clone()))
            {
                return Ok(Some(ResponseEvent::ToolCallInputDelta {
                    item_id,
                    call_id: event.call_id,
                    delta,
                }));
            }
        }
        "response.reasoning_summary_text.delta" => {
            if let (Some(delta), Some(summary_index)) = (event.delta, event.summary_index) {
                return Ok(Some(ResponseEvent::ReasoningSummaryDelta {
                    delta,
                    summary_index,
                }));
            }
        }
        "response.reasoning_summary_text.done" => {
            if let (Some(item_id), Some(text), Some(summary_index)) =
                (event.item_id, event.text, event.summary_index)
            {
                return Ok(Some(ResponseEvent::ReasoningSummaryDone {
                    item_id,
                    text,
                    summary_index,
                }));
            }
        }
        "response.reasoning_text.delta" => {
            if let (Some(delta), Some(content_index)) = (event.delta, event.content_index) {
                return Ok(Some(ResponseEvent::ReasoningContentDelta {
                    delta,
                    content_index,
                }));
            }
        }
        "response.created" => {
            if event.response.is_some() {
                return Ok(Some(ResponseEvent::Created {}));
            }
        }
        "response.failed" => {
            return Err(
                if let Some(resp_val) = event.response
                    && let Some(error) = resp_val.get("error")
                    && let Ok(error) = serde_json::from_value::<Error>(error.clone())
                {
                    if error.is_context_window_error() {
                        ApiError::ContextWindowExceeded
                    } else if error.is_quota_exceeded_error() {
                        ApiError::QuotaExceeded
                    } else if error.is_usage_not_included() {
                        ApiError::UsageNotIncluded
                    } else if error.is_cyber_policy_error() {
                        let message = error.cyber_policy_message();
                        ApiError::CyberPolicy { message }
                    } else if matches!(error.code.as_deref(), Some("invalid_prompt" | "bio_policy"))
                    {
                        let message = error
                            .message
                            .unwrap_or_else(|| "Invalid request.".to_string());
                        ApiError::InvalidRequest { message }
                    } else if error.is_server_overloaded_error() {
                        ApiError::ServerOverloaded
                    } else {
                        let delay = try_parse_retry_after(&error);
                        let message = error.message.unwrap_or_default();
                        ApiError::Retryable { message, delay }
                    }
                } else {
                    ApiError::Stream("response.failed event received".into())
                },
            );
        }
        "response.incomplete" => {
            let reason = event.response.as_ref().and_then(|response| {
                response
                    .get("incomplete_details")
                    .and_then(|details| details.get("reason"))
                    .and_then(Value::as_str)
            });
            let reason = reason.unwrap_or("unknown");
            let message = format!("Incomplete response returned, reason: {reason}");
            return Err(ApiError::Stream(message));
        }
        "response.completed" => {
            if let Some(resp_val) = event.response {
                match serde_json::from_value::<ResponseCompleted>(resp_val) {
                    Ok(resp) => {
                        return Ok(Some(ResponseEvent::Completed {
                            response_id: resp.id,
                            token_usage: resp.usage.map(Into::into),
                            end_turn: resp.end_turn,
                        }));
                    }
                    Err(err) => {
                        let error = format!("failed to parse ResponseCompleted: {err}");
                        return Err(ApiError::Stream(error));
                    }
                }
            }
        }
        "response.output_item.added" => {
            if let Some(item_val) = event.item {
                if let Ok(item) = serde_json::from_value::<ResponseItem>(item_val) {
                    return Ok(Some(ResponseEvent::OutputItemAdded(item)));
                }
            }
        }
        "response.reasoning_summary_part.added" => {
            if let Some(summary_index) = event.summary_index {
                return Ok(Some(ResponseEvent::ReasoningSummaryPartAdded {
                    summary_index,
                }));
            }
        }
        _ => {}
    }

    Ok(None)
}

fn try_parse_retry_after(err: &Error) -> Option<Duration> {
    if err.code.as_deref() != Some("rate_limit_exceeded") {
        return None;
    }

    let re = rate_limit_regex();
    if let Some(message) = &err.message
        && let Some(captures) = re.captures(message)
    {
        let seconds = captures.get(1);
        let unit = captures.get(2);

        if let (Some(value), Some(unit)) = (seconds, unit) {
            let value = value.as_str().parse::<f64>().ok()?;
            let unit = unit.as_str().to_ascii_lowercase();

            if unit == "s" || unit.starts_with("second") {
                return Some(Duration::from_secs_f64(value));
            } else if unit == "ms" {
                return Some(Duration::from_millis(value as u64));
            }
        }
    }
    None
}

fn rate_limit_regex() -> &'static regex_lite::Regex {
    static RE: std::sync::OnceLock<regex_lite::Regex> = std::sync::OnceLock::new();
    #[expect(clippy::unwrap_used)]
    RE.get_or_init(|| {
        regex_lite::Regex::new(r"(?i)try again in\s*(\d+(?:\.\d+)?)\s*(s|ms|seconds?)").unwrap()
    })
}
