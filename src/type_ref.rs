use super::SchemaType;
use serde::Serialize;

/// TypeRef enables eventual resolution of schema `$ref` to Rust types.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TypeRef {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub _type: Option<SchemaType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
    pub _ref: Option<String>,
}

impl TypeRef {
    pub fn from_ref(_ref: Option<String>) -> Self {
        Self { _type: None, _ref }
    }

    pub fn from_type(_type: &SchemaType) -> Self {
        Self {
            _type: Some(_type.to_owned()),
            _ref: None,
        }
    }

    pub fn native_string() -> Self {
        Self {
            _type: Some(SchemaType {
                ns: "".to_owned(),
                name: "String".to_owned(),
            }),
            _ref: None,
        }
    }
}
