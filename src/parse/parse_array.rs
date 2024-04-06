use anyhow::Result;
use convert_case::Casing;
use serde_json::{Map, Value};

use crate::{util::*, AnyOf, Error, Resolver, SchemaObject, SchemaTree, SchemaType, TypeRef};

use super::{get_any_of, parse_object};

pub fn parse_array(
    entry_name: &str,
    items: &Map<String, Value>,
    parent_obj: &SchemaObject,
    parent_tree: &mut SchemaTree,
    resolver: &mut Resolver,
) -> Result<(Option<TypeRef>, Option<AnyOf>)> {
    // Simple case: items has a `$ref`
    if let Ok(_ref) = try_string_entry("$ref", items) {
        return Ok((Some(TypeRef::from_ref(Some(_ref.to_owned()))), None));
    }
    // Second simplest type - an AnyOf
    else if let Ok(any_of) = get_any_of(items) {
        return Ok((None, any_of));
    }
    // If there's not a `$ref`, then there needs to be a `type`, and it
    // needs to be an object
    let Ok(_type) = try_string_entry("type", items) else {
        println!("Bad Array!");
        println!("{:?}", &items);
        return Err(Error::MalformedArray.into());
    };

    if _type == "object" {
        // Remove the 's' from the array property name
        let entry_name = entry_name.unplural();
        let sub_tree = parent_tree.get_or_add_tree(&entry_name)?;
        let ns = format!("{}::{}", &parent_obj.ns, &entry_name);
        let name = entry_name.to_case(convert_case::Case::Pascal);
        parse_object(&entry_name, &ns, &name, items, sub_tree, resolver)?;
        return Ok((
            Some(TypeRef::from_type(&SchemaType::new(&ns, &entry_name))),
            None,
        ));
    } else {
        println!("Unexpected type in array: {}", _type);
    }

    // This is an array item with an embedded type.  Create the new type

    Ok((None, None))
}
