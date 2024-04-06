use anyhow::Result;
use serde_json::{Map, Value};

use crate::{Resolver, SchemaObject, SchemaTree, SchemaType};

use super::parse_props;

pub fn parse_object(
    entry_name: &str,
    ns: &str,
    name: &str,
    map: &Map<String, Value>,
    tree: &mut SchemaTree,
    resolver: &mut Resolver,
) -> Result<()> {
    let mut obj = SchemaObject::parse(ns, name, map)?;
    if let Some(id) = &obj.id {
        // This object has an ID.  Add it to the resolver
        let schema_type = SchemaType::from(&obj);
        resolver.add_type(id, schema_type);
    }
    parse_props(map, &mut obj, tree, resolver)?;
    tree.add_object(entry_name, &obj)?;

    Ok(())
}
