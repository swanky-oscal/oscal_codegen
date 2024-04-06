use anyhow::Result;
use convert_case::Casing;
use indexmap::IndexMap;
use serde_json::{Map, Value};

use crate::{util::*, Property, Resolver, SchemaObject, SchemaTree, SchemaType, TypeRef};

use super::{get_any_of, get_required, parse_array, parse_object};

pub fn parse_props(
    obj_map: &Map<String, Value>,
    parent_obj: &mut SchemaObject,
    parent_tree: &mut SchemaTree,
    resolver: &mut Resolver,
) -> Result<()> {
    let Ok(props_value) = try_map_entry("properties", obj_map) else {
        return Ok(());
    };
    let mut props: IndexMap<String, Property> = IndexMap::new();
    let requireds = get_required(obj_map)?;

    for (prop_name, prop_value) in props_value {
        let required = requireds.contains(prop_name);

        let entry_name = prop_name.to_case(convert_case::Case::Snake);

        let prop_map = value_to_map(prop_value)?;

        let _type = try_string_entry("type", prop_map)
            .ok()
            .map(|s| s.to_owned());
        let title = try_string_entry("title", prop_map)
            .ok()
            .map(|s| s.to_owned());
        let description = try_string_entry("description", prop_map)
            .ok()
            .map(|s| s.to_owned());
        let ref_name = try_string_entry("$ref", prop_map)
            .ok()
            .map(|s| s.to_owned());

        // Some properties are easy.  they don't have a type, and do have a $ref
        if _type.is_none() && ref_name.is_some() {
            let prop = Property {
                optional: !required,
                array: false,
                name: entry_name.clone(),
                title,
                description,
                type_ref: TypeRef::from_ref(ref_name),
                enums: None,
            };
            props.insert(entry_name, prop);

            // If this has anyOf, then it's a $ref with an enum
        } else if let Some(any_of) = get_any_of(prop_map)? {
            let prop = Property {
                optional: !required,
                array: false,
                name: entry_name.clone(),
                title,
                description,
                type_ref: TypeRef {
                    _type: None,
                    _ref: Some(any_of.ref_name.to_owned()),
                },
                enums: Some(any_of.enums),
            };
            props.insert(entry_name, prop);
        } else if _type == Some("string".to_owned()) {
            let prop = Property {
                optional: !required,
                array: false,
                name: entry_name.clone(),
                title,
                description,
                type_ref: TypeRef::native_string(),
                enums: None,
            };
            props.insert(entry_name, prop);

        // The next easy type is if the prop is an array
        } else if _type == Some("array".to_owned()) {
            // All arrays have an 'items' element
            let items = try_map_entry("items", prop_map)?;
            let (t_opt, any_opt) =
                parse_array(&entry_name, items, parent_obj, parent_tree, resolver)?;
            if let Some(type_ref) = t_opt {
                let prop = Property {
                    optional: !required,
                    array: true,
                    name: entry_name.clone(),
                    title,
                    description,
                    type_ref,
                    enums: None,
                };
                props.insert(entry_name, prop);
            } else if let Some(any_of) = any_opt {
                let prop = Property {
                    optional: !required,
                    array: true,
                    name: entry_name.clone(),
                    title,
                    description,
                    type_ref: TypeRef {
                        _type: None,
                        _ref: Some(any_of.ref_name.to_owned()),
                    },
                    enums: Some(any_of.enums),
                };
                props.insert(entry_name, prop);
            }
        } else if _type == Some("object".to_owned()) {
            // We encountered a nested object.
            let sub_tree = parent_tree.get_or_add_tree(&entry_name)?;
            let ns = format!("{}::{}", &parent_obj.ns, &entry_name);
            let name = entry_name.to_case(convert_case::Case::Pascal);
            parse_object(&entry_name, &ns, &name, prop_map, sub_tree, resolver)?;
            // We added the object.  Now add the property
            let prop = Property {
                optional: !required,
                array: false,
                name: entry_name.clone(),
                title,
                description,
                type_ref: TypeRef::from_type(&SchemaType::new(&ns, &name)),
                enums: None,
            };
            props.insert(entry_name, prop);
        } else {
            println!("Unhandled property type: {:?}", prop_map);
        }
    }

    if !props.is_empty() {
        parent_obj.props = Some(props);
    }

    Ok(())
}
