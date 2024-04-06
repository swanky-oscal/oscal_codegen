#![allow(dead_code)]
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Object expected")]
    ObjectExpected,
    #[error("String expected")]
    StringExpected,
    #[error("Array expected")]
    ArrayExpected,
    #[error("map does not contain {0}")]
    AttributeNotFound(String),
    #[error("No definitions found")]
    NoDefinitions,
    #[error("Unexpected entry: {0}")]
    UnexpectedEntry(String),
    #[error("anyOf is Malformed")]
    MalformedAnyOf,
    #[error("Missing $ref")]
    MissingAnyOfRef,
    #[error("Missing enums")]
    MissingAnyOfEnum,
    #[error("Malformed Array")]
    MalformedArray,
    #[error("Resolver error")]
    ResolverFailure,
    #[error("Namespace was incomplete: {0} - {1}")]
    IncompleteNamespace(String, String),
}
