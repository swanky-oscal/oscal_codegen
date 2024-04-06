use anyhow::Result;
use serde_json::{Map, Value};

use crate::Error;

#[allow(dead_code)]
pub fn is_string(value: &Value) -> bool {
    matches!(value, Value::String(_))
}

#[allow(dead_code)]
pub fn is_map(value: &Value) -> bool {
    matches!(value, Value::Object(_))
}

#[allow(dead_code)]
pub fn is_array(value: &Value) -> bool {
    matches!(value, Value::Array(_))
}

#[allow(dead_code)]
pub fn value_to_string(value: &Value) -> Result<&String> {
    match value {
        Value::String(s) => Ok(s),
        _ => Err(Error::StringExpected.into()),
    }
}

#[allow(dead_code)]
pub fn value_to_map(value: &Value) -> Result<&Map<String, Value>> {
    match value {
        Value::Object(s) => Ok(s),
        _ => Err(Error::StringExpected.into()),
    }
}

#[allow(dead_code)]
pub fn value_to_array(value: &Value) -> Result<&Vec<Value>> {
    match value {
        Value::Array(s) => Ok(s),
        _ => Err(Error::StringExpected.into()),
    }
}

pub fn try_get_attr<'a>(key: &str, map: &'a Map<String, Value>) -> Result<&'a Value> {
    map.get(key)
        .ok_or(Error::AttributeNotFound(key.to_owned()).into())
}

pub fn try_string_entry<'a>(key: &str, map: &'a Map<String, Value>) -> Result<&'a String> {
    value_to_string(try_get_attr(key, map)?)
}

#[allow(dead_code)]
pub fn try_map_entry<'a>(
    key: &'a str,
    map: &'a Map<String, Value>,
) -> Result<&'a Map<String, Value>> {
    value_to_map(try_get_attr(key, map)?)
}

#[allow(dead_code)]
pub fn try_array_entry<'a>(key: &str, map: &'a Map<String, Value>) -> Result<&'a Vec<Value>> {
    value_to_array(try_get_attr(key, map)?)
}
