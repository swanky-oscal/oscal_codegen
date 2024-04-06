use anyhow::Result;
use indexmap::IndexMap;

use crate::{Error, Namespace, Property, Resolver};

pub(super) fn generate_props(
    props: &Option<IndexMap<String, Property>>,
    resolver: &Resolver,
    name_space: &mut Namespace,
) -> Result<Option<String>> {
    let Some(props) = props else {
        return Ok(None);
    };

    let mut result = vec![];

    for (_prop_name, prop) in props {
        let (fixed_name, name, is_reserved) = prop.name();
        if is_reserved {
            result.push(format!(r##"    #[serde(rename = "{}")]"##, &name));
        }
        let Some(schema_type) = resolver.resolve(&prop.type_ref) else {
            eprintln!("Failed to resolve: {} {:?}", &prop.name, &prop.type_ref);
            return Err(Error::ResolverFailure.into());
        };

        name_space.add_type(&schema_type)?;

        if let Some(title) = &prop.title {
            result.push(format!("    /// {}", title));
        }
        if let Some(description) = &prop.description {
            result.push(format!("    /// {}", description));
        }
        let mut target_name = schema_type.name;
        if prop.array {
            target_name = format!("Vec<{}>", &target_name);
        }
        if prop.optional {
            target_name = format!("Option<{}>", &target_name);
        }
        result.push(format!("    pub {}: {},", &fixed_name, &target_name));
    }
    Ok(Some(result.join("\n")))
}
