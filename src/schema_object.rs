use anyhow::Result;
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{try_string_entry, Property, StringType};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SchemaObject {
    pub ns: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<IndexMap<String, Property>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_type: Option<StringType>,
}

impl SchemaObject {
    pub fn parse(ns: &str, name: &str, map: &Map<String, Value>) -> Result<Self> {
        let id = try_string_entry("$id", map).ok().map(|s| s.to_owned());
        let description = try_string_entry("description", map)
            .ok()
            .map(|s| s.to_owned());
        let _type = try_string_entry("type", map).ok().map(|s| s.to_owned());
        let type_ref = try_string_entry("$ref", map).ok().map(|s| s.to_owned());
        let string_type = StringType::parse(map)?;
        Ok(Self {
            ns: ns.to_owned(),
            name: name.to_owned(),
            id,
            description,
            _type,
            type_ref,
            props: None,
            string_type,
        })
    }

    pub fn has_options(&self) -> bool {
        match &self.props {
            None => false,
            Some(props) => {
                for (_name, prop) in props {
                    if prop.optional {
                        return true;
                    }
                }
                false
            }
        }
    }
}
