use anyhow::Result;
use serde_json::{Map, Value};

use crate::try_string_entry;

/// NumberType represents a constraint on the IntegrDatatype
#[derive(Debug, Clone, PartialEq)]
pub struct NumberType {
    pub minimum: Option<u32>,
}

impl StringType {
    pub fn is_empty(&self) -> bool {
        self.format.is_none() && self.pattern.is_none() && self.content_encoding.is_none()
    }

    pub fn parse(map: &Map<String, Value>) -> Result<Option<Self>> {
        let result = Self::from(map);
        match result.is_empty() {
            true => Ok(Some(result)),
            false => Ok(None),
        }
    }
}

impl From<&Map<String, Value>> for StringType {
    fn from(map: &Map<String, Value>) -> Self {
        let format = try_string_entry("format", map).ok().map(|s| s.to_string());
        let pattern = try_string_entry("pattern", map).ok().map(|s| s.to_string());
        let content_encoding = try_string_entry("contentEncoding", map)
            .ok()
            .map(|s| s.to_string());
        Self {
            format,
            pattern,
            content_encoding,
        }
    }
}
