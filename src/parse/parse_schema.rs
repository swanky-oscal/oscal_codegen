use anyhow::Result;
use serde_json::Value;

use crate::{is_datatype, util::*, NamespaceEntry, Resolver, SchemaTree};

use super::parse_object;

pub fn parse_schema(value: &Value, resolver: &mut Resolver) -> Result<SchemaTree> {
    let mut crate_tree = SchemaTree::new();
    let schema = value_to_map(value)?;
    let definitions = try_map_entry("definitions", schema)?;

    for (name, value) in definitions {
        if name.eq_ignore_ascii_case("json-schema-directive") {
            continue;
        }
        if is_datatype(name) {
            continue;
        }

        let names = NamespaceEntry::from(name.as_ref());
        let map = value_to_map(value)?;
        // So, now we have 2 choices.
        // 1. The object is a top level object, like StringDatatype
        // 2. The object is a 3 part name
        if names.left.is_empty() {
            // Type 1: top level object
            parse_object(
                &names.right,
                &names.right,
                &names.rust,
                map,
                &mut crate_tree,
                resolver,
            )?;
        } else {
            // Type 2, multiple names
            // Add or get the first part
            let left_tree = crate_tree.get_or_add_tree(&names.left)?;
            let right_tree = left_tree.get_or_add_tree(&names.right)?;
            let ns = format!("{}::{}", &names.left, &names.right);

            parse_object(&names.right, &ns, &names.rust, map, right_tree, resolver)?;
        }
    }
    //println!("{}", &tree);
    Ok(crate_tree)
}
