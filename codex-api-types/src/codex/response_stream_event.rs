#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "ts")]
use ts_rs::TS;
#[cfg(feature = "js")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::response_event::{
    ModelVerification, SafetyBuffering, TokenUsage, TurnModerationMetadataEvent,
};

const X_CODEX_TURN_STATE_HEADER: &str = "x-codex-turn-state";
const TRUSTED_ACCESS_FOR_CYBER_VERIFICATION: &str = "trusted_access_for_cyber";

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct ResponsesStreamEvent {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: String,
    pub headers: Option<Value>,
    pub metadata: Option<Value>,
    pub response: Option<Value>,
    pub item: Option<Value>,
    pub item_id: Option<String>,
    pub call_id: Option<String>,
    pub delta: Option<String>,
    pub text: Option<String>,
    pub summary_index: Option<i64>,
    pub content_index: Option<i64>,
    pub safety_buffering: Option<Value>,
}

impl ResponsesStreamEvent {
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the effective model reported by the server, if present.
    ///
    /// Precedence:
    /// 1. `response.headers` for standard Responses stream events.
    /// 2. top-level `headers` for websocket metadata events.
    pub fn response_model(&self) -> Option<String> {
        let response_headers_model = self
            .response
            .as_ref()
            .and_then(|response| response.get("headers"))
            .and_then(header_openai_model_value_from_json);

        match response_headers_model {
            Some(model) => Some(model),
            None => self
                .headers
                .as_ref()
                .and_then(header_openai_model_value_from_json),
        }
    }

    pub fn turn_state(&self) -> Option<String> {
        if self.kind() != "response.metadata" {
            return None;
        }

        self.headers
            .as_ref()
            .and_then(header_turn_state_value_from_json)
    }

    pub fn model_verifications(&self) -> Option<Vec<ModelVerification>> {
        if self.kind() != "response.metadata" {
            return None;
        }

        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.get("openai_verification_recommendation"))
            .and_then(model_verifications_from_json_value)
    }

    pub fn turn_moderation_metadata(&self) -> Option<TurnModerationMetadataEvent> {
        if self.kind() != "response.metadata" {
            return None;
        }

        self.metadata
            .as_ref()
            .and_then(|metadata| metadata.get("openai_chatgpt_moderation_metadata"))
            .cloned()
            .map(|metadata| TurnModerationMetadataEvent { metadata })
    }

    pub fn safety_buffering(
        &self,
        treatment: &SafetyBufferingTreatment,
    ) -> Option<SafetyBuffering> {
        let value = self.safety_buffering.as_ref()?;
        let retry_model_present = value.as_object()?.contains_key("retry_model");
        let mut buffering: SafetyBuffering = serde_json::from_value(value.clone()).ok()?;
        buffering.show_buffering_ui = true;
        if !retry_model_present {
            buffering.faster_model.clone_from(&treatment.faster_model);
        }
        Some(buffering)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct SafetyBufferingTreatment {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub faster_model: Option<String>,
}

fn header_openai_model_value_from_json(value: &Value) -> Option<String> {
    let headers = value.as_object()?;
    headers.iter().find_map(|(name, value)| {
        if name.eq_ignore_ascii_case("openai-model") || name.eq_ignore_ascii_case("x-openai-model")
        {
            json_value_as_string(value)
        } else {
            None
        }
    })
}

fn header_turn_state_value_from_json(value: &Value) -> Option<String> {
    let headers = value.as_object()?;
    headers.iter().find_map(|(name, value)| {
        if name.eq_ignore_ascii_case(X_CODEX_TURN_STATE_HEADER) {
            json_value_as_string(value)
        } else {
            None
        }
    })
}

fn model_verifications_from_json_value(value: &Value) -> Option<Vec<ModelVerification>> {
    let verifications = value
        .as_array()
        .map(|items| {
            let mut verifications = Vec::new();
            for verification in items
                .iter()
                .filter_map(Value::as_str)
                .filter_map(parse_model_verification)
            {
                if !verifications.contains(&verification) {
                    verifications.push(verification);
                }
            }
            verifications
        })
        .unwrap_or_default();

    if verifications.is_empty() {
        None
    } else {
        Some(verifications)
    }
}

fn parse_model_verification(value: &str) -> Option<ModelVerification> {
    match value {
        TRUSTED_ACCESS_FOR_CYBER_VERIFICATION => Some(ModelVerification::TrustedAccessForCyber),
        _ => None,
    }
}

fn json_value_as_string(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Array(items) => items.first().and_then(json_value_as_string),
        _ => None,
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct Error {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub r#type: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub code: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub message: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub plan_type: Option<String>,
    pub resets_at: Option<i64>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ResponseCompleted {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usage: Option<ResponseCompletedUsage>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub end_turn: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ResponseCompletedUsage {
    pub input_tokens: i64,
    pub input_tokens_details: Option<ResponseCompletedInputTokensDetails>,
    pub output_tokens: i64,
    pub output_tokens_details: Option<ResponseCompletedOutputTokensDetails>,
    pub total_tokens: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ResponseCompletedInputTokensDetails {
    pub cached_tokens: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ResponseCompletedOutputTokensDetails {
    pub reasoning_tokens: i64,
}

impl From<ResponseCompletedUsage> for TokenUsage {
    fn from(val: ResponseCompletedUsage) -> Self {
        TokenUsage {
            input_tokens: val.input_tokens,
            cached_input_tokens: val
                .input_tokens_details
                .map(|d| d.cached_tokens)
                .unwrap_or(0),
            output_tokens: val.output_tokens,
            reasoning_output_tokens: val
                .output_tokens_details
                .map(|d| d.reasoning_tokens)
                .unwrap_or(0),
            total_tokens: val.total_tokens,
        }
    }
}

#[cfg_attr(feature = "js", wasm_bindgen)]
impl Error {
    pub fn is_context_window_error(&self) -> bool {
        self.code.as_deref() == Some("context_length_exceeded")
    }

    pub fn is_quota_exceeded_error(&self) -> bool {
        self.code.as_deref() == Some("insufficient_quota")
    }

    pub fn is_usage_not_included(&self) -> bool {
        self.code.as_deref() == Some("usage_not_included")
    }

    pub fn is_cyber_policy_error(&self) -> bool {
        self.code.as_deref() == Some("cyber_policy")
    }

    pub fn is_server_overloaded_error(&self) -> bool {
        self.code.as_deref() == Some("server_is_overloaded")
            || self.code.as_deref() == Some("slow_down")
    }

    pub fn cyber_policy_message(&self) -> String {
        self.message
            .clone()
            .filter(|message| !message.trim().is_empty())
            .unwrap_or_else(cyber_policy_fallback_message)
    }
}

#[cfg_attr(feature = "js", wasm_bindgen)]
pub fn cyber_policy_fallback_message() -> String {
    "This request has been flagged for possible cybersecurity risk.".to_string()
}
