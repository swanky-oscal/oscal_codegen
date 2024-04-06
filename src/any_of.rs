use serde_json::Value;

use crate::{
    try_array_entry, try_string_entry, value_to_array, value_to_map, value_to_string, Error,
};

/// "properties": {
///     "prop": {
///         "anyOf"
///     }
/// }
///
#[derive(Debug, Clone, PartialEq)]
pub struct AnyOf {
    pub ref_name: String,
    pub enums: Vec<String>,
}

impl TryFrom<&Value> for AnyOf {
    type Error = anyhow::Error;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let any_of_array = value_to_array(value)?;
        if any_of_array.len() != 2 {
            return Err(Error::MalformedAnyOf.into());
        }

        let ref_map = value_to_map(&any_of_array[0]).map_err(|_| Error::MalformedAnyOf)?;
        let ref_name = try_string_entry("$ref", ref_map)
            .map_err(|_| Error::MissingAnyOfRef)?
            .to_owned();

        let enum_map = value_to_map(&any_of_array[1]).map_err(|_| Error::MalformedAnyOf)?;
        let enum_array = try_array_entry("enum", enum_map).map_err(|_| Error::MissingAnyOfEnum)?;

        let mut enums = vec![];
        for enum_value in enum_array {
            enums.push(
                value_to_string(enum_value)
                    .map_err(|_| Error::MalformedAnyOf)?
                    .to_owned(),
            );
        }
        Ok(Self { ref_name, enums })
    }
}
