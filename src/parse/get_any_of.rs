use anyhow::Result;
use serde_json::{Map, Value};

use crate::AnyOf;

/// Previous versions of the schema used `allOf`.  Most of those have been
/// changed to `anyOf`.  But we need to handle both types.
pub fn get_any_of(obj_map: &Map<String, Value>) -> Result<Option<AnyOf>> {
    let mut any_of_val = obj_map.get("anyOf");
    if any_of_val.is_none() {
        any_of_val = obj_map.get("allOf");
    }

    match any_of_val {
        Some(any_of) => Ok(Some(AnyOf::try_from(any_of)?)),
        _ => Ok(None),
    }
}
