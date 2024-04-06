use anyhow::Result;
use serde_json::{Map, Value};

use crate::util::*;

pub fn get_required(obj_map: &Map<String, Value>) -> Result<Vec<String>> {
    let mut result = vec![];
    if let Ok(required) = try_array_entry("required", obj_map) {
        for value in required {
            result.push(value_to_string(value)?.to_owned());
        }
    }
    Ok(result)
}
