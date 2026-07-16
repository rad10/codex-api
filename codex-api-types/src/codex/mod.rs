//! Shared model metadata types exchanged between Codex services and clients.
//!
//! These types are serialized across core, TUI, app-server, and SDK boundaries, so field defaults
//! are used to preserve compatibility when older payloads omit newly introduced attributes.

use std::{collections::HashMap, fmt, str::FromStr};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
#[cfg(feature = "ts")]
use ts_rs::TS;
#[cfg(feature = "js")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::agent_path::AgentPath;
use crate::response_item::ResponseItem;
use crate::thread_id::ThreadId;

const PERSONALITY_PLACEHOLDER: &str = "{{ personality }}";
pub const SPEED_TIER_FAST: &str = "fast";

/// Request/config sentinel for explicit standard routing.
///
/// This is not a catalog service tier id. It means the user intentionally
/// selected no service tier, so model catalog defaults should not apply.
pub const SERVICE_TIER_DEFAULT_REQUEST_VALUE: &str = "default";

/// See https://platform.openai.com/docs/guides/reasoning?api-mode=responses#get-started-with-reasoning
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(type = "string"))]
#[cfg_attr(
    feature = "schemars",
    schemars(description = "A non-empty reasoning effort value advertised by the model.")
)]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    #[default]
    Medium,
    High,
    XHigh,
    Max,
    Ultra,
    /// A model-defined effort value that this client does not know yet.
    Custom(String),
}

impl ReasoningEffort {
    /// Returns the exact value used on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
            Self::Max => "max",
            Self::Ultra => "ultra",
            Self::Custom(effort) => effort,
        }
    }
}

impl fmt::Display for ReasoningEffort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for ReasoningEffort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ReasoningEffort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}

impl FromStr for ReasoningEffort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::XHigh),
            "max" => Ok(Self::Max),
            "ultra" => Ok(Self::Ultra),
            "" => Err("reasoning_effort must not be empty".to_string()),
            effort => Ok(Self::Custom(effort.to_string())),
        }
    }
}

/// Canonical user-input modality tags advertised by a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum InputModality {
    /// Plain text turns and tool payloads.
    Text,
    /// Image attachments included in user turns.
    Image,
}

/// Backward-compatible default when `input_modalities` is omitted on the wire.
///
/// Legacy payloads predate modality metadata, so we conservatively assume both text and images are
/// accepted unless a preset explicitly narrows support.
#[cfg_attr(feature = "js", wasm_bindgen)]
pub fn default_input_modalities() -> Vec<InputModality> {
    vec![InputModality::Text, InputModality::Image]
}

/// A reasoning effort option that can be surfaced for a model.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct ReasoningEffortPreset {
    /// Effort level that the model supports.
    pub effort: ReasoningEffort,
    /// Short human description shown next to the effort in UIs.
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ModelUpgrade {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub id: String,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub migration_config_key: String,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub model_link: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub upgrade_copy: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub migration_markdown: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ModelAvailabilityNux {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ModelServiceTier {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub id: String,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub name: String,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub description: String,
}

/// Metadata describing a Codex-supported model.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct ModelPreset {
    /// Stable identifier for the preset.
    pub id: String,
    /// Model slug (e.g., "gpt-5").
    pub model: String,
    /// Display name shown in UIs.
    pub display_name: String,
    /// Short human description shown in UIs.
    pub description: String,
    /// Reasoning effort applied when none is explicitly chosen.
    pub default_reasoning_effort: ReasoningEffort,
    /// Supported reasoning effort options.
    pub supported_reasoning_efforts: Vec<ReasoningEffortPreset>,
    /// Whether this model supports personality-specific instructions.
    #[cfg_attr(feature = "serde", serde(default))]
    pub supports_personality: bool,
    /// Deprecated: use `service_tiers` instead.
    #[cfg_attr(feature = "serde", serde(default))]
    pub additional_speed_tiers: Vec<String>,
    /// Service tiers this model can run with.
    #[cfg_attr(feature = "serde", serde(default))]
    pub service_tiers: Vec<ModelServiceTier>,
    /// Catalog default service tier id for this model.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub default_service_tier: Option<String>,
    /// Whether this is the default model for new users.
    pub is_default: bool,
    /// recommended upgrade model
    pub upgrade: Option<ModelUpgrade>,
    /// Whether this preset should appear in the picker UI.
    pub show_in_picker: bool,
    /// Multi-agent backend selected when this model starts a new thread.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing, skip_deserializing)
    )]
    #[cfg_attr(feature = "schemars", schemars(skip))]
    #[cfg_attr(feature = "ts", ts(skip))]
    pub multi_agent_version: Option<MultiAgentVersion>,
    /// Availability NUX shown when this preset becomes accessible to the user.
    pub availability_nux: Option<ModelAvailabilityNux>,
    /// whether this model is supported in the api
    pub supported_in_api: bool,
    /// Input modalities accepted when composing user turns for this preset.
    #[cfg_attr(feature = "serde", serde(default = "default_input_modalities"))]
    pub input_modalities: Vec<InputModality>,
}

/// Visibility of a model in the picker or APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ModelVisibility {
    List,
    Hide,
    None,
}

/// Shell execution capability for a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ConfigShellToolType {
    Default,
    Local,
    UnifiedExec,
    Disabled,
    ShellCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ApplyPatchToolType {
    Freeform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum WebSearchToolType {
    #[default]
    Text,
    TextAndImage,
}

/// Server-provided truncation policy metadata for a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum TruncationMode {
    Bytes,
    Tokens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ToolMode {
    Direct,
    CodeMode,
    CodeModeOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct TruncationPolicyConfig {
    pub mode: TruncationMode,
    pub limit: i64,
}

impl TruncationPolicyConfig {
    pub const fn bytes(limit: i64) -> Self {
        Self {
            mode: TruncationMode::Bytes,
            limit,
        }
    }

    pub const fn tokens(limit: i64) -> Self {
        Self {
            mode: TruncationMode::Tokens,
            limit,
        }
    }
}

/// Semantic version triple encoded as an array in JSON (e.g. [0, 62, 0]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ClientVersion(pub i32, pub i32, pub i32);

#[cfg(feature = "serde")]
const fn default_effective_context_window_percent() -> i64 {
    95
}

#[cfg(feature = "serde")]
const fn default_true() -> bool {
    true
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[cfg(feature = "serde")]
const fn is_true(value: &bool) -> bool {
    *value
}

/// Model metadata returned by the Codex backend `/models` endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct ModelInfo {
    pub slug: String,
    pub display_name: String,
    pub description: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub default_reasoning_level: Option<ReasoningEffort>,
    pub supported_reasoning_levels: Vec<ReasoningEffortPreset>,
    pub shell_type: ConfigShellToolType,
    pub visibility: ModelVisibility,
    pub supported_in_api: bool,
    pub priority: i32,
    #[cfg_attr(feature = "serde", serde(default))]
    pub additional_speed_tiers: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub service_tiers: Vec<ModelServiceTier>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub default_service_tier: Option<String>,
    pub availability_nux: Option<ModelAvailabilityNux>,
    pub upgrade: Option<ModelInfoUpgrade>,
    pub base_instructions: String,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub model_messages: Option<ModelMessages>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub include_skills_usage_instructions: bool,
    /// Whether the model accepts the Responses API `reasoning.summary` parameter.
    #[cfg_attr(
        feature = "serde",
        serde(default = "default_true", skip_serializing_if = "is_true")
    )]
    pub supports_reasoning_summary_parameter: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub default_reasoning_summary: ReasoningSummary,
    pub support_verbosity: bool,
    pub default_verbosity: Option<Verbosity>,
    pub apply_patch_tool_type: Option<ApplyPatchToolType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub web_search_tool_type: WebSearchToolType,
    pub truncation_policy: TruncationPolicyConfig,
    pub supports_parallel_tool_calls: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub supports_image_detail_original: bool,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub context_window: Option<i64>,
    /// Maximum context window allowed for config overrides.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub max_context_window: Option<i64>,
    /// Token threshold for automatic compaction. When omitted, core derives it
    /// from `context_window` (90%). When provided, core clamps it to 90% of the
    /// context window when available.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub auto_compact_token_limit: Option<i64>,
    /// Opaque identifier for compaction-compatible model configurations.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub comp_hash: Option<String>,
    /// Percentage of the context window considered usable for inputs, after
    /// reserving headroom for system prompts, tool overhead, and model output.
    #[cfg_attr(
        feature = "serde",
        serde(default = "default_effective_context_window_percent")
    )]
    pub effective_context_window_percent: i64,
    pub experimental_supported_tools: Vec<String>,
    /// Input modalities accepted by the backend for this model.
    #[cfg_attr(feature = "serde", serde(default = "default_input_modalities"))]
    pub input_modalities: Vec<InputModality>,
    /// Internal-only marker set by core when a model slug resolved to fallback metadata.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing, skip_deserializing)
    )]
    #[cfg_attr(feature = "schemars", schemars(skip))]
    #[cfg_attr(feature = "ts", ts(skip))]
    pub used_fallback_model_metadata: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub supports_search_tool: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub use_responses_lite: bool,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub auto_review_model_override: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub tool_mode: Option<ToolMode>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub multi_agent_version: Option<MultiAgentVersion>,
}

impl ModelInfo {
    pub fn resolved_context_window(&self) -> Option<i64> {
        self.context_window.or(self.max_context_window)
    }

    pub fn auto_compact_token_limit(&self) -> Option<i64> {
        let context_limit = self
            .resolved_context_window()
            .map(|context_window| (context_window * 9) / 10);
        let config_limit = self.auto_compact_token_limit;
        if let Some(context_limit) = context_limit {
            return Some(
                config_limit.map_or(context_limit, |limit| std::cmp::min(limit, context_limit)),
            );
        }
        config_limit
    }

    pub fn supports_personality(&self) -> bool {
        self.model_messages
            .as_ref()
            .is_some_and(ModelMessages::supports_personality)
    }

    pub fn get_model_instructions(&self, personality: Option<Personality>) -> String {
        if let Some(model_messages) = &self.model_messages
            && let Some(template) = &model_messages.instructions_template
        {
            // if we have a template, always use it
            let personality_message = model_messages
                .get_personality_message(personality)
                .unwrap_or_default();
            template.replace(PERSONALITY_PLACEHOLDER, personality_message.as_str())
        } else {
            self.base_instructions.clone()
        }
    }
}

/// A strongly-typed template for assembling model instructions and developer messages. If
/// instructions_* is populated and valid, it will override base_instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ModelMessages {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub instructions_template: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub instructions_variables: Option<ModelInstructionsVariables>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub approvals: Option<ApprovalMessages>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub auto_review: Option<AutoReviewMessages>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub permissions: Option<PermissionMessages>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ApprovalMessages {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub on_request: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub on_request_auto_review: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct AutoReviewMessages {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub policy: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub policy_template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct PermissionMessages {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub danger_full_access: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub workspace_write: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub read_only: Option<String>,
}

#[cfg_attr(feature = "js", wasm_bindgen)]
impl ModelMessages {
    fn has_personality_placeholder(&self) -> bool {
        self.instructions_template
            .as_ref()
            .map(|spec| spec.contains(PERSONALITY_PLACEHOLDER))
            .unwrap_or(false)
    }

    fn supports_personality(&self) -> bool {
        self.has_personality_placeholder()
            && self
                .instructions_variables
                .as_ref()
                .is_some_and(ModelInstructionsVariables::is_complete)
    }

    pub fn get_personality_message(&self, personality: Option<Personality>) -> Option<String> {
        self.instructions_variables
            .as_ref()
            .and_then(|variables| variables.get_personality_message(personality))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ModelInstructionsVariables {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub personality_default: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub personality_friendly: Option<String>,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub personality_pragmatic: Option<String>,
}

#[cfg_attr(feature = "js", wasm_bindgen)]
impl ModelInstructionsVariables {
    pub fn is_complete(&self) -> bool {
        self.personality_default.is_some()
            && self.personality_friendly.is_some()
            && self.personality_pragmatic.is_some()
    }

    pub fn get_personality_message(&self, personality: Option<Personality>) -> Option<String> {
        if let Some(personality) = personality {
            match personality {
                Personality::None => Some(String::new()),
                Personality::Friendly => self.personality_friendly.clone(),
                Personality::Pragmatic => self.personality_pragmatic.clone(),
            }
        } else {
            self.personality_default.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct ModelInfoUpgrade {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub model: String,
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub migration_markdown: String,
}

impl From<&ModelUpgrade> for ModelInfoUpgrade {
    fn from(upgrade: &ModelUpgrade) -> Self {
        ModelInfoUpgrade {
            model: upgrade.id.clone(),
            migration_markdown: upgrade.migration_markdown.clone().unwrap_or_default(),
        }
    }
}

/// Response wrapper for `/models`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct ModelsResponse {
    pub models: Vec<ModelInfo>,
}

// convert ModelInfo to ModelPreset
impl From<ModelInfo> for ModelPreset {
    fn from(info: ModelInfo) -> Self {
        let supports_personality = info.supports_personality();
        ModelPreset {
            id: info.slug.clone(),
            model: info.slug.clone(),
            display_name: info.display_name,
            description: info.description.unwrap_or_default(),
            default_reasoning_effort: info
                .default_reasoning_level
                .unwrap_or(ReasoningEffort::None),
            supported_reasoning_efforts: info.supported_reasoning_levels.clone(),
            supports_personality,
            additional_speed_tiers: info.additional_speed_tiers,
            service_tiers: info.service_tiers,
            default_service_tier: info.default_service_tier,
            is_default: false, // default is the highest priority available model
            upgrade: info.upgrade.as_ref().map(|upgrade| ModelUpgrade {
                id: upgrade.model.clone(),
                migration_config_key: info.slug.clone(),
                // todo(aibrahim): add the model link here.
                model_link: None,
                upgrade_copy: None,
                migration_markdown: Some(upgrade.migration_markdown.clone()),
            }),
            show_in_picker: info.visibility == ModelVisibility::List,
            multi_agent_version: info.multi_agent_version,
            availability_nux: info.availability_nux,
            supported_in_api: info.supported_in_api,
            input_modalities: info.input_modalities,
        }
    }
}

impl ModelPreset {
    pub fn supports_fast_mode(&self) -> bool {
        self.service_tiers
            .iter()
            .any(|tier| tier.id == ServiceTier::Fast.request_value())
            || self
                .additional_speed_tiers
                .iter()
                .any(|tier| tier == SPEED_TIER_FAST)
    }
}

impl ModelInfo {
    pub fn supports_service_tier(&self, service_tier: &str) -> bool {
        self.service_tiers
            .iter()
            .any(|tier| tier.id == service_tier)
    }

    pub fn service_tier_for_request(&self, service_tier: Option<String>) -> Option<String> {
        service_tier.filter(|service_tier| {
            service_tier != SERVICE_TIER_DEFAULT_REQUEST_VALUE
                && self.supports_service_tier(service_tier)
        })
    }
}

impl ModelPreset {
    /// Filter models based on authentication mode.
    ///
    /// In ChatGPT mode, all models are visible. Otherwise, only API-supported models are shown.
    pub fn filter_by_auth(models: Vec<ModelPreset>, chatgpt_mode: bool) -> Vec<ModelPreset> {
        models
            .into_iter()
            .filter(|model| chatgpt_mode || model.supported_in_api)
            .collect()
    }

    /// Recompute the single default preset using picker visibility.
    ///
    /// The first picker-visible model wins; if none are picker-visible, the first model wins.
    pub fn mark_default_by_picker_visibility(models: &mut [ModelPreset]) {
        for preset in models.iter_mut() {
            preset.is_default = false;
        }
        if let Some(default) = models.iter_mut().find(|preset| preset.show_in_picker) {
            default.is_default = true;
        } else if let Some(default) = models.first_mut() {
            default.is_default = true;
        }
    }
}

/// A summary of the reasoning performed by the model. This can be useful for
/// debugging and understanding the model's reasoning process.
/// See https://platform.openai.com/docs/guides/reasoning?api-mode=responses#reasoning-summaries
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ReasoningSummary {
    #[default]
    Auto,
    Concise,
    Detailed,
    /// Option to disable reasoning summaries.
    None,
}

/// Controls output length/detail on GPT-5 models via the Responses API.
/// Serialized with lowercase values to match the OpenAI API.
#[derive(Hash, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum Verbosity {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum Personality {
    None,
    Friendly,
    Pragmatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ServiceTier {
    Fast,
    Flex,
}

impl ServiceTier {
    pub const fn request_value(self) -> &'static str {
        match self {
            Self::Fast => "priority",
            Self::Flex => "flex",
        }
    }
}
#[cfg_attr(feature = "js", wasm_bindgen)]
impl ServiceTier {
    pub fn from_request_value(value: &str) -> Option<Self> {
        match value {
            "fast" | "priority" => Some(Self::Fast),
            "flex" => Some(Self::Flex),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "ts", ts(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum MultiAgentVersion {
    Disabled,
    V1,
    V2,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "ts", ts(rename_all = "lowercase"))]
pub enum SessionSource {
    Cli,
    #[default]
    VSCode,
    Exec,
    Mcp,
    Custom(String),
    Internal(InternalSessionSource),
    SubAgent(SubAgentSource),
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "ts", ts(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum InternalSessionSource {
    MemoryConsolidation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "ts", ts(rename_all = "snake_case"))]
pub enum SubAgentSource {
    Review,
    Compact,
    ThreadSpawn {
        parent_thread_id: ThreadId,
        depth: i32,
        #[cfg_attr(feature = "serde", serde(default))]
        agent_path: Option<AgentPath>,
        #[cfg_attr(feature = "serde", serde(default))]
        agent_nickname: Option<String>,
        #[cfg_attr(feature = "serde", serde(default, alias = "agent_type"))]
        agent_role: Option<String>,
    },
    MemoryConsolidation,
    Other(String),
}

impl fmt::Display for SessionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionSource::Cli => f.write_str("cli"),
            SessionSource::VSCode => f.write_str("vscode"),
            SessionSource::Exec => f.write_str("exec"),
            SessionSource::Mcp => f.write_str("mcp"),
            SessionSource::Custom(source) => f.write_str(source),
            SessionSource::Internal(source) => write!(f, "internal_{source}"),
            SessionSource::SubAgent(sub_source) => write!(f, "subagent_{sub_source}"),
            SessionSource::Unknown => f.write_str("unknown"),
        }
    }
}

impl SessionSource {
    pub fn from_startup_arg(value: &str) -> Result<Self, &'static str> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err("session source must not be empty");
        }

        let normalized = trimmed.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "cli" => SessionSource::Cli,
            "vscode" => SessionSource::VSCode,
            "exec" => SessionSource::Exec,
            "mcp" | "appserver" | "app-server" | "app_server" => SessionSource::Mcp,
            "unknown" => SessionSource::Unknown,
            _ => SessionSource::Custom(normalized),
        })
    }

    pub fn is_internal(&self) -> bool {
        matches!(self, SessionSource::Internal(_))
    }

    pub fn is_non_root_agent(&self) -> bool {
        matches!(
            self,
            SessionSource::Internal(_) | SessionSource::SubAgent(_)
        )
    }

    pub fn get_nickname(&self) -> Option<String> {
        match self {
            SessionSource::SubAgent(SubAgentSource::ThreadSpawn { agent_nickname, .. }) => {
                agent_nickname.clone()
            }
            _ => None,
        }
    }

    pub fn get_agent_role(&self) -> Option<String> {
        match self {
            SessionSource::SubAgent(SubAgentSource::ThreadSpawn { agent_role, .. }) => {
                agent_role.clone()
            }
            _ => None,
        }
    }

    pub fn get_agent_path(&self) -> Option<AgentPath> {
        match self {
            SessionSource::SubAgent(SubAgentSource::ThreadSpawn { agent_path, .. }) => {
                agent_path.clone()
            }
            _ => None,
        }
    }

    pub fn restriction_product(&self) -> Option<Product> {
        match self {
            SessionSource::Custom(source) => Product::from_session_source_name(source),
            SessionSource::Cli
            | SessionSource::VSCode
            | SessionSource::Exec
            | SessionSource::Mcp
            | SessionSource::Unknown => Some(Product::Codex),
            SessionSource::Internal(_) | SessionSource::SubAgent(_) => None,
        }
    }

    pub fn matches_product_restriction(&self, products: &[Product]) -> bool {
        products.is_empty()
            || self
                .restriction_product()
                .is_some_and(|product| product.matches_product_restriction(products))
    }

    pub fn parent_thread_id(&self) -> Option<ThreadId> {
        match self {
            SessionSource::SubAgent(subagent_source) => subagent_source.parent_thread_id(),
            SessionSource::Cli
            | SessionSource::VSCode
            | SessionSource::Exec
            | SessionSource::Mcp
            | SessionSource::Custom(_)
            | SessionSource::Internal(_)
            | SessionSource::Unknown => None,
        }
    }
}

impl fmt::Display for SubAgentSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubAgentSource::Review => f.write_str("review"),
            SubAgentSource::Compact => f.write_str("compact"),
            SubAgentSource::MemoryConsolidation => f.write_str("memory_consolidation"),
            SubAgentSource::ThreadSpawn {
                parent_thread_id,
                depth,
                ..
            } => {
                write!(f, "thread_spawn_{parent_thread_id}_d{depth}")
            }
            SubAgentSource::Other(other) => f.write_str(other),
        }
    }
}

impl SubAgentSource {
    pub fn kind(&self) -> &str {
        match self {
            SubAgentSource::Review => "review",
            SubAgentSource::Compact => "compact",
            SubAgentSource::ThreadSpawn { .. } => "thread_spawn",
            SubAgentSource::MemoryConsolidation => "memory_consolidation",
            SubAgentSource::Other(other) => other,
        }
    }

    pub fn parent_thread_id(&self) -> Option<ThreadId> {
        match self {
            SubAgentSource::ThreadSpawn {
                parent_thread_id, ..
            } => Some(*parent_thread_id),
            SubAgentSource::Review
            | SubAgentSource::Compact
            | SubAgentSource::MemoryConsolidation
            | SubAgentSource::Other(_) => None,
        }
    }
}

impl fmt::Display for InternalSessionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalSessionSource::MemoryConsolidation => f.write_str("memory_consolidation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "ts", ts(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum Product {
    #[cfg_attr(feature = "serde", serde(alias = "CHATGPT"))]
    Chatgpt,
    #[cfg_attr(feature = "serde", serde(alias = "CODEX"))]
    Codex,
    #[cfg_attr(feature = "serde", serde(alias = "ATLAS"))]
    Atlas,
}

impl Product {
    pub fn to_app_platform(self) -> &'static str {
        match self {
            Self::Chatgpt => "chat",
            Self::Codex => "codex",
            Self::Atlas => "atlas",
        }
    }

    pub fn matches_product_restriction(&self, products: &[Self]) -> bool {
        products.is_empty() || products.contains(self)
    }
}

#[cfg_attr(feature = "js", wasm_bindgen)]
impl Product {
    pub fn from_session_source_name(value: &str) -> Option<Self> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "chatgpt" => Some(Self::Chatgpt),
            "codex" => Some(Self::Codex),
            "atlas" => Some(Self::Atlas),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResponsesApiRequest {
    pub model: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub instructions: String,
    pub input: Vec<ResponseItem>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub tools: Option<Vec<serde_json::Value>>,
    pub tool_choice: String,
    pub parallel_tool_calls: bool,
    pub reasoning: Option<Reasoning>,
    pub store: bool,
    pub stream: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub stream_options: Option<StreamOptions>,
    pub include: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub service_tier: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub prompt_cache_key: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub text: Option<TextControls>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub client_metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Reasoning {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub effort: Option<ReasoningEffort>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub summary: Option<ReasoningSummary>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub context: Option<ReasoningContext>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ReasoningContext {
    Auto,
    CurrentTurn,
    AllTurns,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub struct StreamOptions {
    pub reasoning_summary_delivery: ReasoningSummaryDelivery,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum ReasoningSummaryDelivery {
    SequentialCutoff,
}

/// Controls the `text` field for the Responses API, combining verbosity and
/// optional JSON schema output formatting.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextControls {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub verbosity: Option<OpenAiVerbosity>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub format: Option<TextFormat>,
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum OpenAiVerbosity {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextFormat {
    /// Format type used by the OpenAI text controls.
    pub r#type: TextFormatType,
    /// When true, the server is expected to strictly validate responses.
    pub strict: bool,
    /// JSON schema for the desired output.
    pub schema: serde_json::Value,
    /// Friendly name for the format, used in telemetry/debugging.
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum TextFormatType {
    #[default]
    JsonSchema,
}

impl From<Verbosity> for OpenAiVerbosity {
    fn from(v: Verbosity) -> Self {
        match v {
            Verbosity::Low => OpenAiVerbosity::Low,
            Verbosity::Medium => OpenAiVerbosity::Medium,
            Verbosity::High => OpenAiVerbosity::High,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "serde")]
    use serde_json::{from_str, to_string};

    fn test_model(spec: Option<ModelMessages>) -> ModelInfo {
        ModelInfo {
            slug: "test-model".to_string(),
            display_name: "Test Model".to_string(),
            description: None,
            default_reasoning_level: None,
            supported_reasoning_levels: vec![],
            shell_type: ConfigShellToolType::ShellCommand,
            visibility: ModelVisibility::List,
            supported_in_api: true,
            priority: 1,
            additional_speed_tiers: Vec::new(),
            service_tiers: Vec::new(),
            default_service_tier: None,
            availability_nux: None,
            upgrade: None,
            base_instructions: "base".to_string(),
            model_messages: spec,
            include_skills_usage_instructions: false,
            supports_reasoning_summary_parameter: true,
            default_reasoning_summary: ReasoningSummary::Auto,
            support_verbosity: false,
            default_verbosity: None,
            apply_patch_tool_type: None,
            web_search_tool_type: WebSearchToolType::Text,
            truncation_policy: TruncationPolicyConfig::bytes(/*limit*/ 10_000),
            supports_parallel_tool_calls: false,
            supports_image_detail_original: false,
            context_window: None,
            max_context_window: None,
            auto_compact_token_limit: None,
            comp_hash: None,
            effective_context_window_percent: 95,
            experimental_supported_tools: vec![],
            input_modalities: default_input_modalities(),
            used_fallback_model_metadata: false,
            supports_search_tool: false,
            use_responses_lite: false,
            auto_review_model_override: None,
            tool_mode: None,
            multi_agent_version: None,
        }
    }

    fn personality_variables() -> ModelInstructionsVariables {
        ModelInstructionsVariables {
            personality_default: Some("default".to_string()),
            personality_friendly: Some("friendly".to_string()),
            personality_pragmatic: Some("pragmatic".to_string()),
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn model_messages_deserialize_without_approvals() {
        let messages: ModelMessages =
            from_str(r#"{"instructions_template":null,"instructions_variables":null}"#)
                .expect("model messages should deserialize");

        assert_eq!(messages.approvals, None);
        assert_eq!(messages.permissions, None);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn approval_messages_preserve_missing_and_empty_values() {
        let messages: ModelMessages = from_str(
            r#"{
                "instructions_template": null,
                "instructions_variables": null,
                "approvals": {
                    "on_request": ""
                }
            }"#,
        )
        .expect("approval messages should deserialize");

        assert_eq!(
            messages.approvals,
            Some(ApprovalMessages {
                on_request: Some(String::new()),
                on_request_auto_review: None,
            })
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn auto_review_messages_preserve_missing_and_empty_template_values() {
        let missing_template: ModelMessages = from_str(
            r#"{
                "instructions_template": null,
                "instructions_variables": null,
                "auto_review": {
                    "policy": "policy"
                }
            }"#,
        )
        .expect("auto-review messages should deserialize without a policy template");
        let empty_template: ModelMessages = from_str(
            r#"{
                "instructions_template": null,
                "instructions_variables": null,
                "auto_review": {
                    "policy": "policy",
                    "policy_template": ""
                }
            }"#,
        )
        .expect("auto-review messages should deserialize with an empty policy template");

        assert_eq!(
            missing_template.auto_review,
            Some(AutoReviewMessages {
                policy: Some("policy".to_string()),
                policy_template: None,
            })
        );
        assert_eq!(
            empty_template.auto_review,
            Some(AutoReviewMessages {
                policy: Some("policy".to_string()),
                policy_template: Some(String::new()),
            })
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn permission_messages_preserve_missing_and_empty_values() {
        let messages: ModelMessages = from_str(
            r#"{
                "instructions_template": null,
                "instructions_variables": null,
                "permissions": {
                    "workspace_write": ""
                }
            }"#,
        )
        .expect("permission messages should deserialize");

        assert_eq!(
            messages.permissions,
            Some(PermissionMessages {
                danger_full_access: None,
                workspace_write: Some(String::new()),
                read_only: None,
            })
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn reasoning_effort_accepts_known_and_custom_values() {
        let custom = ReasoningEffort::Custom("future".to_string());
        let deserialized = from_str::<ReasoningEffort>(r#""future""#)
            .expect("custom reasoning effort should deserialize");
        let serialized = to_string(&custom).expect("custom reasoning effort should serialize");
        let serialized_max = to_string(&ReasoningEffort::Max).expect("Max should serialize");
        let serialized_ultra = to_string(&ReasoningEffort::Ultra).expect("Ultra should serialize");

        assert_eq!(
            (
                "high".parse(),
                "max".parse(),
                "ultra".parse(),
                "future".parse(),
                deserialized,
                serialized,
                serialized_max,
                serialized_ultra,
                custom.to_string(),
            ),
            (
                Ok(ReasoningEffort::High),
                Ok(ReasoningEffort::Max),
                Ok(ReasoningEffort::Ultra),
                Ok(custom.clone()),
                custom,
                r#""future""#.to_string(),
                r#""max""#.to_string(),
                r#""ultra""#.to_string(),
                "future".to_string(),
            )
        );
    }

    #[test]
    fn reasoning_effort_rejects_empty_values() {
        assert_eq!(
            "".parse::<ReasoningEffort>(),
            Err("reasoning_effort must not be empty".to_string())
        );
    }

    #[test]
    fn get_model_instructions_uses_template_when_placeholder_present() {
        let model = test_model(Some(ModelMessages {
            instructions_template: Some("Hello {{ personality }}".to_string()),
            instructions_variables: Some(personality_variables()),
            approvals: None,
            auto_review: None,
            permissions: None,
        }));

        let instructions = model.get_model_instructions(Some(Personality::Friendly));

        assert_eq!(instructions, "Hello friendly");
    }

    #[test]
    fn get_model_instructions_always_strips_placeholder() {
        let model = test_model(Some(ModelMessages {
            instructions_template: Some("Hello\n{{ personality }}".to_string()),
            instructions_variables: Some(ModelInstructionsVariables {
                personality_default: None,
                personality_friendly: Some("friendly".to_string()),
                personality_pragmatic: None,
            }),
            approvals: None,
            auto_review: None,
            permissions: None,
        }));
        assert_eq!(
            model.get_model_instructions(Some(Personality::Friendly)),
            "Hello\nfriendly"
        );
        assert_eq!(
            model.get_model_instructions(Some(Personality::Pragmatic)),
            "Hello\n"
        );
        assert_eq!(
            model.get_model_instructions(Some(Personality::None)),
            "Hello\n"
        );
        assert_eq!(
            model.get_model_instructions(/*personality*/ None),
            "Hello\n"
        );

        let model_no_personality = test_model(Some(ModelMessages {
            instructions_template: Some("Hello\n{{ personality }}".to_string()),
            instructions_variables: Some(ModelInstructionsVariables {
                personality_default: None,
                personality_friendly: None,
                personality_pragmatic: None,
            }),
            approvals: None,
            auto_review: None,
            permissions: None,
        }));
        assert_eq!(
            model_no_personality.get_model_instructions(Some(Personality::Friendly)),
            "Hello\n"
        );
        assert_eq!(
            model_no_personality.get_model_instructions(Some(Personality::Pragmatic)),
            "Hello\n"
        );
        assert_eq!(
            model_no_personality.get_model_instructions(Some(Personality::None)),
            "Hello\n"
        );
        assert_eq!(
            model_no_personality.get_model_instructions(/*personality*/ None),
            "Hello\n"
        );
    }

    #[test]
    fn get_model_instructions_falls_back_when_template_is_missing() {
        let model = test_model(Some(ModelMessages {
            instructions_template: None,
            instructions_variables: Some(ModelInstructionsVariables {
                personality_default: None,
                personality_friendly: None,
                personality_pragmatic: None,
            }),
            approvals: None,
            auto_review: None,
            permissions: None,
        }));

        let instructions = model.get_model_instructions(Some(Personality::Friendly));

        assert_eq!(instructions, "base");
    }

    #[test]
    fn get_personality_message_returns_default_when_personality_is_none() {
        let personality_template = personality_variables();
        assert_eq!(
            personality_template.get_personality_message(/*personality*/ None),
            Some("default".to_string())
        );
    }

    #[test]
    fn get_personality_message() {
        let personality_variables = personality_variables();
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::Friendly)),
            Some("friendly".to_string())
        );
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::Pragmatic)),
            Some("pragmatic".to_string())
        );
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::None)),
            Some(String::new())
        );
        assert_eq!(
            personality_variables.get_personality_message(/*personality*/ None),
            Some("default".to_string())
        );

        let personality_variables = ModelInstructionsVariables {
            personality_default: Some("default".to_string()),
            personality_friendly: None,
            personality_pragmatic: None,
        };
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::Friendly)),
            None
        );
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::Pragmatic)),
            None
        );
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::None)),
            Some(String::new())
        );
        assert_eq!(
            personality_variables.get_personality_message(/*personality*/ None),
            Some("default".to_string())
        );

        let personality_variables = ModelInstructionsVariables {
            personality_default: None,
            personality_friendly: Some("friendly".to_string()),
            personality_pragmatic: Some("pragmatic".to_string()),
        };
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::Friendly)),
            Some("friendly".to_string())
        );
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::Pragmatic)),
            Some("pragmatic".to_string())
        );
        assert_eq!(
            personality_variables.get_personality_message(Some(Personality::None)),
            Some(String::new())
        );
        assert_eq!(
            personality_variables.get_personality_message(/*personality*/ None),
            None
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn model_info_defaults_availability_nux_to_none_when_omitted() {
        let model: ModelInfo = serde_json::from_value(serde_json::json!({
            "slug": "test-model",
            "display_name": "Test Model",
            "description": null,
            "supported_reasoning_levels": [],
            "shell_type": "shell_command",
            "visibility": "list",
            "supported_in_api": true,
            "priority": 1,
            "upgrade": null,
            "base_instructions": "base",
            "model_messages": null,
            "default_reasoning_summary": "auto",
            "support_verbosity": false,
            "default_verbosity": null,
            "apply_patch_tool_type": null,
            "truncation_policy": {
                "mode": "bytes",
                "limit": 10000
            },
            "supports_parallel_tool_calls": false,
            "supports_image_detail_original": false,
            "context_window": null,
            "auto_compact_token_limit": null,
            "effective_context_window_percent": 95,
            "experimental_supported_tools": [],
            "input_modalities": ["text", "image"]
        }))
        .expect("deserialize model info");

        assert_eq!(model.availability_nux, None);
        assert!(!model.include_skills_usage_instructions);
        assert!(model.supports_reasoning_summary_parameter);
        assert!(!model.supports_image_detail_original);
        assert_eq!(model.web_search_tool_type, WebSearchToolType::Text);
        assert!(!model.supports_search_tool);
        assert!(!model.use_responses_lite);
        assert_eq!(model.comp_hash, None);
        assert_eq!(model.auto_review_model_override, None);
        assert_eq!(model.tool_mode, None);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn model_info_deserializes_known_tool_mode() {
        let mut value =
            serde_json::to_value(test_model(/*spec*/ None)).expect("serialize test model");
        let object = value
            .as_object_mut()
            .expect("model info should be an object");
        object.insert(
            "tool_mode".to_string(),
            serde_json::Value::String("code_mode_only".to_string()),
        );
        let model = serde_json::from_value::<ModelInfo>(value).expect("deserialize model info");

        assert_eq!(model.tool_mode, Some(ToolMode::CodeModeOnly));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn model_info_treats_unknown_tool_mode_as_omitted() {
        let mut value =
            serde_json::to_value(test_model(/*spec*/ None)).expect("serialize test model");
        let object = value
            .as_object_mut()
            .expect("model info should be an object");
        object.insert(
            "tool_mode".to_string(),
            serde_json::Value::String("future_tool_mode".to_string()),
        );
        let model = serde_json::from_value::<ModelInfo>(value).expect("deserialize model info");

        assert_eq!(model.tool_mode, None);
        let serialized = serde_json::to_value(model).expect("serialize model info");
        let object = serialized
            .as_object()
            .expect("model info should be an object");
        assert!(!object.contains_key("tool_mode"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn model_info_treats_unknown_multi_agent_version_as_omitted() {
        let mut value =
            serde_json::to_value(test_model(/*spec*/ None)).expect("serialize test model");
        let object = value
            .as_object_mut()
            .expect("model info should be an object");
        object.insert(
            "multi_agent_version".to_string(),
            serde_json::Value::String("future_multi_agent_version".to_string()),
        );
        let model = serde_json::from_value::<ModelInfo>(value).expect("deserialize model info");

        assert_eq!(model.multi_agent_version, None);
    }

    #[test]
    fn resolved_context_window_prefers_context_window() {
        let model = ModelInfo {
            context_window: Some(273_000),
            max_context_window: Some(400_000),
            ..test_model(/*spec*/ None)
        };

        assert_eq!(model.resolved_context_window(), Some(273_000));
    }

    #[test]
    fn resolved_context_window_falls_back_to_max_context_window() {
        let model = ModelInfo {
            context_window: None,
            max_context_window: Some(400_000),
            ..test_model(/*spec*/ None)
        };

        assert_eq!(model.resolved_context_window(), Some(400_000));
        assert_eq!(model.auto_compact_token_limit(), Some(360_000));
    }

    #[test]
    fn model_preset_preserves_availability_nux() {
        let preset = ModelPreset::from(ModelInfo {
            availability_nux: Some(ModelAvailabilityNux {
                message: "Try Spark.".to_string(),
            }),
            additional_speed_tiers: vec![SPEED_TIER_FAST.to_string()],
            default_service_tier: Some(ServiceTier::Fast.request_value().to_string()),
            service_tiers: Vec::new(),
            ..test_model(/*spec*/ None)
        });

        assert_eq!(
            preset.availability_nux,
            Some(ModelAvailabilityNux {
                message: "Try Spark.".to_string(),
            })
        );
        assert!(preset.supports_fast_mode());
        assert_eq!(
            preset.default_service_tier,
            Some(ServiceTier::Fast.request_value().to_string())
        );
    }

    #[test]
    fn model_preset_supports_fast_mode_from_service_tiers() {
        let preset = ModelPreset::from(ModelInfo {
            service_tiers: vec![ModelServiceTier {
                id: ServiceTier::Fast.request_value().to_string(),
                name: "Fast".to_string(),
                description: "Priority processing.".to_string(),
            }],
            ..test_model(/*spec*/ None)
        });

        assert!(preset.supports_fast_mode());
    }

    #[test]
    fn service_tier_for_request_omits_explicit_default_tier() {
        let model = ModelInfo {
            default_service_tier: Some(ServiceTier::Fast.request_value().to_string()),
            service_tiers: vec![ModelServiceTier {
                id: ServiceTier::Fast.request_value().to_string(),
                name: "Fast".to_string(),
                description: "Priority processing.".to_string(),
            }],
            ..test_model(/*spec*/ None)
        };

        assert_eq!(
            model.service_tier_for_request(Some(SERVICE_TIER_DEFAULT_REQUEST_VALUE.to_string())),
            None
        );
    }

    #[test]
    fn service_tier_for_request_filters_unsupported_tiers() {
        let model = ModelInfo {
            default_service_tier: Some(ServiceTier::Fast.request_value().to_string()),
            service_tiers: vec![ModelServiceTier {
                id: ServiceTier::Fast.request_value().to_string(),
                name: "Fast".to_string(),
                description: "Priority processing.".to_string(),
            }],
            ..test_model(/*spec*/ None)
        };

        assert_eq!(
            model.service_tier_for_request(Some(ServiceTier::Fast.request_value().to_string())),
            Some(ServiceTier::Fast.request_value().to_string())
        );
        assert_eq!(
            model.service_tier_for_request(Some("unsupported".to_string())),
            None
        );
        assert_eq!(model.service_tier_for_request(/*service_tier*/ None), None);
    }

    #[test]
    fn service_tier_for_request_does_not_apply_catalog_default() {
        let model = ModelInfo {
            default_service_tier: Some(ServiceTier::Fast.request_value().to_string()),
            service_tiers: vec![ModelServiceTier {
                id: ServiceTier::Fast.request_value().to_string(),
                name: "Fast".to_string(),
                description: "Priority processing.".to_string(),
            }],
            ..test_model(/*spec*/ None)
        };

        assert_eq!(model.service_tier_for_request(/*service_tier*/ None), None);
    }
}
