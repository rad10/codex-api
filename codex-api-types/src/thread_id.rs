use std::{borrow::Cow, fmt::Display};

use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TS, Hash)]
#[ts(type = "string")]
/// Identifier for a Codex thread.
///
/// Codex-generated thread IDs are UUIDv7, and some use cases rely on that.
pub struct ThreadId {
    pub(crate) uuid: Uuid,
}

impl ThreadId {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::now_v7(),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self {
            uuid: Uuid::parse_str(s)?,
        })
    }
}

impl FromStr for ThreadId {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { uuid: s.parse()? })
    }
}

impl TryFrom<&str> for ThreadId {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for ThreadId {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<ThreadId> for String {
    fn from(value: ThreadId) -> Self {
        value.to_string()
    }
}

impl Default for ThreadId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ThreadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.uuid.fmt(f)
    }
}

impl Serialize for ThreadId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.uuid.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ThreadId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self { uuid: Uuid::deserialize(deserializer)? })
    }
}

impl JsonSchema for ThreadId {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ThreadId")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        <String>::json_schema(generator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_thread_id_default_is_not_zeroes() {
        let id = ThreadId::default();
        assert_ne!(id.uuid, Uuid::nil());
    }
}
