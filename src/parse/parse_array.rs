use anyhow::Result;
use convert_case::Casing;
use serde_json::{Map, Value};

use crate::{util::*, AnyOf, Error, Resolver, Schema, SchemaObject, SchemaType, TypeRef};

use super::{get_any_of, parse_object};

pub fn parse_array(
    entry_name: &str,
    items: &Map<String, Value>,
    parent_obj: &SchemaObject,
    parent_tree: &mut Schema,
    resolver: &mut Resolver,
) -> Result<(Option<TypeRef>, Option<AnyOf>)> {
    // Simple case: items has a `$ref`
    if let Ok(_ref) = try_string_entry("$ref", items) {
        return Ok((Some(TypeRef::from_ref(Some(_ref.to_owned()))), None));
    }
    // Second simplest type - an AnyOf
    else if let Some(any_of) = get_any_of(items)? {
        return Ok((None, Some(any_of)));
    }

    // If there's not a `$ref`, then there needs to be a `type`, and it
    // needs to be an object
    let Ok(_type) = try_string_entry("type", items) else {
        println!("Bad Array!");
        println!("{:?}", &items);
        return Err(Error::MalformedArray.into());
    };

    if _type == "object" {
        let entry_name = match try_string_entry("title", items) {
            Ok(title) => title.as_str().unplural().to_case(convert_case::Case::Snake),
            // If there is no title, remove the 's' from the array property name
            Err(_) => entry_name.unplural(),
        };
        let sub_tree = parent_tree.get_or_add_tree(&entry_name)?;
        let ns = format!("{}::{}", &parent_obj.ns, &entry_name);
        let name = entry_name.to_case(convert_case::Case::Pascal);
        parse_object(&entry_name, &ns, &name, items, sub_tree, resolver)?;
        return Ok((Some(TypeRef::from_type(&SchemaType::new(&ns, &name))), None));
    } else if _type == "string" {
        return Ok((Some(TypeRef::native_string()), None));
    } else {
        println!("Unexpected type in array: {}", _type);
    }

    // This is an array item with an embedded type.  Create the new type

    Ok((None, None))
}
