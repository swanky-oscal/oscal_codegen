use crate::SchemaObject;
use serde::Serialize;

/// SchemaType provides the ability to generate Rust `use` statements.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SchemaType {
    pub ns: String,
    pub name: String,
}
impl SchemaType {
    pub fn new(ns: &str, name: &str) -> Self {
        if name.contains('_') {
            eprintln!("SchemaType name should not contain underscores");
        }
        Self {
            ns: ns.to_owned(),
            name: name.to_owned(),
        }
    }
}

impl From<&SchemaObject> for SchemaType {
    fn from(obj: &SchemaObject) -> Self {
        Self {
            ns: obj.ns.clone(),
            name: obj.name.clone(),
        }
    }
}
