use anyhow::Result;
use regex::Regex;
use serde_json::{Map, Value};

use crate::{is_datatype, util::*, Error, NamespaceEntry, Resolver, Schema};

use super::parse_object;

pub fn parse_schema(value: &Value, resolver: &mut Resolver) -> Result<Schema> {
    let schema_map = value_to_map(value)?;

    let schema = try_string_entry("$schema", schema_map)?;
    let id = try_string_entry("$id", schema_map)?;
    let version = parse_version_from_id(id)?;
    let comment = try_string_entry("$comment", schema_map)?;

    let mut crate_tree = Schema::new(schema, &version, comment);
    parse_definitions(schema_map, &mut crate_tree, resolver)?;

    //println!("{}", &tree);
    Ok(crate_tree)
}

fn parse_definitions(
    schema_map: &Map<String, Value>,
    crate_tree: &mut Schema,
    resolver: &mut Resolver,
) -> Result<()> {
    let definitions = try_map_entry("definitions", schema_map)?;

    for (name, value) in definitions {
        if is_datatype(name) {
            // Don't worry about the data types.  They are provided by oscal_types
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
                crate_tree,
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

    Ok(())
}

fn parse_version_from_id(id: &str) -> Result<String> {
    let r = Regex::new(
        r##"^http://csrc.nist.gov/ns/oscal/1.0/(?<version>[1-9][0-9]?\.\d+\.\d+)/oscal-complete-schema.json$"##,
    )?;
    let caps = r.captures(id).ok_or(Error::VersionParse)?;
    let version = caps["version"].to_owned();
    Ok(version)
}
