use std::{str::FromStr, time::Duration};

use chrono::{DateTime, Utc};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

use crate::response_item::ResponseItem;

#[derive(Debug)]
pub enum ResponseEvent {
    Created,
    SafetyBuffering(SafetyBuffering),
    OutputItemDone(ResponseItem),
    OutputItemAdded(ResponseItem),
    /// Emitted when the server includes `OpenAI-Model` on the stream response.
    /// This can differ from the requested model when backend safety routing applies.
    ServerModel(String),
    /// Emitted when the server recommends additional account verification.
    ModelVerifications(Vec<ModelVerification>),
    /// Emitted when the server includes moderation metadata for first-party turn presentation.
    TurnModerationMetadata(TurnModerationMetadataEvent),
    /// Emitted when `X-Reasoning-Included: true` is present on the response,
    /// meaning the server already accounted for past reasoning tokens and the
    /// client should not re-estimate them.
    ServerReasoningIncluded(bool),
    Completed {
        response_id: String,
        token_usage: Option<TokenUsage>,
        /// Did the model affirmatively end its turn? Some providers do not set this,
        /// so we rely on fallback logic when this is `None`.
        end_turn: Option<bool>,
    },
    OutputTextDelta(String),
    ToolCallInputDelta {
        item_id: String,
        call_id: Option<String>,
        delta: String,
    },
    ReasoningSummaryDelta {
        delta: String,
        summary_index: i64,
    },
    ReasoningSummaryDone {
        item_id: String,
        text: String,
        summary_index: i64,
    },
    ReasoningContentDelta {
        delta: String,
        content_index: i64,
    },
    ReasoningSummaryPartAdded {
        summary_index: i64,
    },
    RateLimits(RateLimitSnapshot),
    ModelsEtag(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "ts", ts(rename_all = "snake_case"))]
pub enum ModelVerification {
    TrustedAccessForCyber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SafetyBuffering {
    pub use_cases: Vec<String>,
    pub reasons: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub show_buffering_ui: bool,
    #[cfg_attr(feature = "serde", serde(rename = "retry_model"))]
    pub faster_model: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct TurnModerationMetadataEvent {
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct TokenUsage {
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub input_tokens: i64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub cached_input_tokens: i64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub output_tokens: i64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub reasoning_output_tokens: i64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub total_tokens: i64,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct RateLimitSnapshot {
    pub limit_id: Option<String>,
    pub limit_name: Option<String>,
    pub primary: Option<RateLimitWindow>,
    pub secondary: Option<RateLimitWindow>,
    pub credits: Option<CreditsSnapshot>,
    pub individual_limit: Option<SpendControlLimitSnapshot>,
    /// Backend-reported spend-control state. `None` is unavailable, not a sparse-update recovery.
    pub spend_control_reached: Option<bool>,
    pub plan_type: Option<PlanType>,
    pub rate_limit_reached_type: Option<RateLimitReachedType>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "ts", ts(rename_all = "lowercase"))]
pub enum PlanType {
    #[default]
    Free,
    Go,
    Plus,
    Pro,
    ProLite,
    Team,
    #[cfg_attr(feature = "serde", serde(rename = "self_serve_business_usage_based"))]
    #[cfg_attr(feature = "ts", ts(rename = "self_serve_business_usage_based"))]
    SelfServeBusinessUsageBased,
    Business,
    #[cfg_attr(feature = "serde", serde(rename = "enterprise_cbp_usage_based"))]
    #[cfg_attr(feature = "ts", ts(rename = "enterprise_cbp_usage_based"))]
    EnterpriseCbpUsageBased,
    Enterprise,
    Edu,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "ts", ts(rename_all = "snake_case"))]
pub enum RateLimitReachedType {
    RateLimitReached,
    WorkspaceOwnerCreditsDepleted,
    WorkspaceMemberCreditsDepleted,
    WorkspaceOwnerUsageLimitReached,
    WorkspaceMemberUsageLimitReached,
}

impl FromStr for RateLimitReachedType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "rate_limit_reached" => Ok(Self::RateLimitReached),
            "workspace_owner_credits_depleted" => Ok(Self::WorkspaceOwnerCreditsDepleted),
            "workspace_member_credits_depleted" => Ok(Self::WorkspaceMemberCreditsDepleted),
            "workspace_owner_usage_limit_reached" => Ok(Self::WorkspaceOwnerUsageLimitReached),
            "workspace_member_usage_limit_reached" => Ok(Self::WorkspaceMemberUsageLimitReached),
            other => Err(format!("unknown rate limit reached type: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct RateLimitWindow {
    /// Percentage (0-100) of the window that has been consumed.
    pub used_percent: f64,
    /// Rolling window duration, in minutes.
    #[cfg_attr(feature = "ts", ts(type = "number | null"))]
    pub window_minutes: Option<i64>,
    /// Unix timestamp (seconds since epoch) when the window resets.
    #[cfg_attr(feature = "ts", ts(type = "number | null"))]
    pub resets_at: Option<i64>,
}

impl RateLimitWindow {
    /// Rolling window duration, in minutes.
    pub fn window_minutes(&self) -> Option<Duration> {
        self.window_minutes.clone().map(i64::unsigned_abs).map(Duration::from_mins)
    }

    /// Unix timestamp (seconds since epoch) when the window resets.
    pub fn resets_at(&self) -> Option<DateTime<Utc>> {
        self.resets_at.clone().and_then(DateTime::from_timestamp_secs)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct CreditsSnapshot {
    pub has_credits: bool,
    pub unlimited: bool,
    pub balance: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct SpendControlLimitSnapshot {
    pub limit: String,
    pub used: String,
    pub remaining_percent: i32,
    pub resets_at: i64,
}

// Includes prompts, tools and space to call compact.
const BASELINE_TOKENS: i64 = 12000;

impl TokenUsage {
    pub fn is_zero(&self) -> bool {
        self.total_tokens == 0
    }

    pub fn cached_input(&self) -> i64 {
        self.cached_input_tokens.max(0)
    }

    pub fn non_cached_input(&self) -> i64 {
        (self.input_tokens - self.cached_input()).max(0)
    }

    /// Primary count for display as a single absolute value: non-cached input + output.
    pub fn blended_total(&self) -> i64 {
        (self.non_cached_input() + self.output_tokens.max(0)).max(0)
    }

    pub fn tokens_in_context_window(&self) -> i64 {
        self.total_tokens
    }

    /// Estimate the remaining user-controllable percentage of the model's context window.
    ///
    /// `context_window` is the total size of the model's context window.
    /// `BASELINE_TOKENS` should capture tokens that are always present in
    /// the context (e.g., system prompt and fixed tool instructions) so that
    /// the percentage reflects the portion the user can influence.
    ///
    /// This normalizes both the numerator and denominator by subtracting the
    /// baseline, so immediately after the first prompt the UI shows 100% left
    /// and trends toward 0% as the user fills the effective window.
    pub fn percent_of_context_window_remaining(&self, context_window: i64) -> i64 {
        if context_window <= BASELINE_TOKENS {
            return 0;
        }

        let effective_window = context_window - BASELINE_TOKENS;
        let used = (self.tokens_in_context_window() - BASELINE_TOKENS).max(0);
        let remaining = (effective_window - used).max(0);
        ((remaining as f64 / effective_window as f64) * 100.0)
            .clamp(0.0, 100.0)
            .round() as i64
    }

    /// In-place element-wise sum of token counts.
    pub fn add_assign(&mut self, other: &TokenUsage) {
        self.input_tokens += other.input_tokens;
        self.cached_input_tokens += other.cached_input_tokens;
        self.output_tokens += other.output_tokens;
        self.reasoning_output_tokens += other.reasoning_output_tokens;
        self.total_tokens += other.total_tokens;
    }
}
