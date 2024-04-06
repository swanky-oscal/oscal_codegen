use serde::Serialize;

use crate::TypeRef;

const RESERVED_NAMES: [&str; 2] = ["type", "ref"];

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Property {
    pub optional: bool,
    pub array: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub type_ref: TypeRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enums: Option<Vec<String>>,
}

impl Property {
    /// Returns the name, with a starting `_`, if it is
    /// reserved, along with an indicator.
    pub fn name(&self) -> (String, String, bool) {
        match RESERVED_NAMES.contains(&self.name.as_str()) {
            true => (format!("_{}", &self.name), self.name.clone(), true),
            false => (self.name.clone(), self.name.clone(), false),
        }
    }
}
