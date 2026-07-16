use std::{borrow::Borrow, fmt, ops::Deref};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

/// A Responses API item ID. New IDs require an explicit prefix; deserialization
/// remains permissive so legacy rollouts can still be read.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "ts", ts(type = "string"))]
pub struct ResponseItemId(String);

impl ResponseItemId {
    pub fn new(prefix: &str) -> Self {
        Self::with_suffix(prefix, uuid::Uuid::now_v7())
    }

    pub fn with_suffix(prefix: &str, suffix: impl fmt::Display) -> Self {
        Self(format!("{prefix}_{suffix}"))
    }

    pub fn from_server(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_prefixed(&self) -> bool {
        self.split_once('_')
            .is_some_and(|(prefix, suffix)| !prefix.is_empty() && !suffix.is_empty())
    }
}

impl Deref for ResponseItemId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for ResponseItemId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for ResponseItemId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ResponseItemId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<ResponseItemId> for String {
    fn from(value: ResponseItemId) -> Self {
        value.0
    }
}

#[cfg(test)]
#[path = "response_item_id_tests.rs"]
mod tests;
