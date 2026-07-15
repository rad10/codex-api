/// Response wrapper for `/models`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, TS, JsonSchema, Default)]
pub struct ModelsResponse {
    pub models: Vec<ModelInfo>,
}

/// Model metadata returned by the Codex backend `/models` endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ModelInfo {
    pub slug: String,
    pub display_name: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_reasoning_level: Option<ReasoningEffort>,
    pub supported_reasoning_levels: Vec<ReasoningEffortPreset>,
    pub shell_type: ConfigShellToolType,
    pub visibility: ModelVisibility,
    pub supported_in_api: bool,
    pub priority: i32,
    #[serde(default)]
    pub additional_speed_tiers: Vec<String>,
    #[serde(default)]
    pub service_tiers: Vec<ModelServiceTier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_service_tier: Option<String>,
    pub availability_nux: Option<ModelAvailabilityNux>,
    pub upgrade: Option<ModelInfoUpgrade>,
    pub base_instructions: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_messages: Option<ModelMessages>,
    #[serde(default)]
    pub include_skills_usage_instructions: bool,
    /// Whether the model accepts the Responses API `reasoning.summary` parameter.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub supports_reasoning_summary_parameter: bool,
    #[serde(default)]
    pub default_reasoning_summary: ReasoningSummary,
    pub support_verbosity: bool,
    pub default_verbosity: Option<Verbosity>,
    pub apply_patch_tool_type: Option<ApplyPatchToolType>,
    #[serde(default)]
    pub web_search_tool_type: WebSearchToolType,
    pub truncation_policy: TruncationPolicyConfig,
    pub supports_parallel_tool_calls: bool,
    #[serde(default)]
    pub supports_image_detail_original: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_window: Option<i64>,
    /// Maximum context window allowed for config overrides.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_context_window: Option<i64>,
    /// Token threshold for automatic compaction. When omitted, core derives it
    /// from `context_window` (90%). When provided, core clamps it to 90% of the
    /// context window when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_compact_token_limit: Option<i64>,
    /// Opaque identifier for compaction-compatible model configurations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comp_hash: Option<String>,
    /// Percentage of the context window considered usable for inputs, after
    /// reserving headroom for system prompts, tool overhead, and model output.
    #[serde(default = "default_effective_context_window_percent")]
    pub effective_context_window_percent: i64,
    pub experimental_supported_tools: Vec<String>,
    /// Input modalities accepted by the backend for this model.
    #[serde(default = "default_input_modalities")]
    pub input_modalities: Vec<InputModality>,
    /// Internal-only marker set by core when a model slug resolved to fallback metadata.
    #[serde(default, skip_serializing, skip_deserializing)]
    #[schemars(skip)]
    #[ts(skip)]
    pub used_fallback_model_metadata: bool,
    #[serde(default)]
    pub supports_search_tool: bool,
    #[serde(default)]
    pub use_responses_lite: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_review_model_override: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_model_selector"
    )]
    pub tool_mode: Option<ToolMode>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_model_selector"
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
            match personality {
                Some(personality @ (Personality::Friendly | Personality::Pragmatic)) => {
                    trace!(
                        model = %self.slug,
                        %personality,
                        "Model personality requested but model_messages is missing, falling back to base instructions."
                    );
                }
                Some(Personality::None) | None => {}
            }
            self.base_instructions.clone()
        }
    }
}
