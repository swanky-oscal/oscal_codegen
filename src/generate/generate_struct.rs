use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use oscal_types::get_ref_type;

use crate::{Error, Namespace, Resolver, SchemaObject, SchemaType, TypeRef};

use super::generate_props;

fn split_str_on_words(comment: &str) -> Vec<String> {
    if comment.len() < 80 {
        return vec![comment.to_owned()];
    }

    let mut results = vec![];

    let mut tmp_str = String::new();
    for word in comment.split_whitespace() {
        if tmp_str.is_empty() {
            tmp_str.push_str(word);
        } else {
            tmp_str.push_str(&format!(" {}", word));
        }
        if tmp_str.len() > 75 {
            results.push(tmp_str.clone());
            tmp_str.clear();
        }
    }
    if !tmp_str.is_empty() {
        results.push(tmp_str.clone());
    }

    results
}

pub(super) fn generate_struct(
    path: &PathBuf,
    name: &str,
    is_mod: bool,
    obj: &SchemaObject,
    mods: Option<&Vec<&str>>,
    resolver: &Resolver,
) -> Result<()> {
    let mut file_path = path.to_owned();
    if is_mod {
        file_path.push("mod.rs");
    } else {
        file_path.push(&format!("{}.rs", name));
    }

    let mut namespace = Namespace::new("");

    add_type_ref(&obj.type_ref, resolver, &mut namespace)?;

    let props_string = generate_props(&obj.props, resolver, &mut namespace)?;

    // IF this obj has a type_ref (rather than props), then resolve the ref
    if let Some(schema_type) = resolver.resolve(&TypeRef::from_ref(obj.type_ref.clone())) {
        namespace.add_type(&schema_type)?;
    }

    let use_types = namespace.use_types();

    let use_crates = namespace.use_crates(&obj.ns, "");

    let use_supers = namespace.use_supers(&obj.ns, &obj.name, "")?;

    //--------------- Build the actual file ----------------
    let mut buffer = File::create(file_path)?;

    writeln!(buffer, "use serde::{{Deserialize, Serialize}};")?;
    if obj.has_options() {
        writeln!(buffer, "use serde_with::skip_serializing_none;")?;
    }
    writeln!(buffer)?;

    // Add the mods, if provided
    if let Some(mods) = mods {
        for name in mods {
            // Don't write a mod statement for the mod itself.
            if path.ends_with(name) {
                continue;
            }
            writeln!(buffer, "pub mod {};", name)?;
        }
        writeln!(buffer)?;
    }
    if let Some(oscal_types) = use_types {
        writeln!(buffer, "use oscal_types::{};\n", oscal_types)?;
    }
    if !use_crates.is_empty() {
        writeln!(buffer, "use crate::{};\n", &use_crates)?;
    }

    if !use_supers.is_empty() {
        if path.ends_with(name) {
            writeln!(buffer, "use self::{};\n", &use_supers)?;
        } else {
            writeln!(buffer, "use super::{};\n", &use_supers)?;
        }
    }
    writeln!(buffer, "/// {}\n///", &obj.name)?;

    if let Some(desc) = &obj.description {
        for line in split_str_on_words(desc) {
            writeln!(buffer, "/// {}", line)?;
        }
    }
    if let Some(id) = &obj.id {
        writeln!(buffer, "/// $id: {}", id)?;
    }
    if has_options(obj) {
        writeln!(buffer, "#[skip_serializing_none]")?;
    }

    writeln!(
        buffer,
        "#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]"
    )?;
    if obj.type_ref.is_some() {
        format_tuple_struct(&mut buffer, &obj.name, &obj.type_ref, resolver)?;
    } else {
        format_prop_struct(&mut buffer, &obj.name, props_string)?;
    }

    Ok(())
}

fn has_options(obj: &SchemaObject) -> bool {
    let Some(props) = &obj.props else {
        return false;
    };
    for (_, prop) in props {
        if prop.optional {
            return true;
        }
    }
    false
}

fn add_type_ref(
    _ref: &Option<String>,
    resolver: &Resolver,
    namespace: &mut Namespace,
) -> Result<()> {
    if _ref.is_none() {
        return Ok(());
    }

    let Some(schema_type) = resolver.resolve(&TypeRef::from_ref(_ref.to_owned())) else {
        return Err(Error::ResolverFailure.into());
    };

    namespace.add_type(&schema_type)?;

    // Add the crate's Error
    namespace.add_type(&SchemaType {
        ns: "error".to_string(),
        name: "Error".to_string(),
    })?;

    Ok(())
}

fn format_tuple_struct(
    buffer: &mut File,
    name: &str,
    _ref: &Option<String>,
    resolver: &Resolver,
) -> Result<()> {
    let Some(schema_type) = resolver.resolve(&TypeRef::from_ref(_ref.to_owned())) else {
        eprintln!("Failed to resolve {:?}", _ref);
        return Err(Error::ResolverFailure.into());
    };

    writeln!(buffer, "pub struct {}({});", name, &schema_type.name)?;

    if let Ok(ref_type) = get_ref_type(&schema_type.name) {
        writeln!(buffer)?;
        // Deref
        writeln!(
            buffer,
            r##"impl std::ops::Deref for {name} {{
    type Target = {ref_type};
    fn deref(&self) -> &Self::Target {{
            self.0.deref()
    }}
}}"##
        )?;

        // TryFrom
        writeln!(
            buffer,
            r##"
impl TryFrom<&{ref_type}> for {name} {{
    type Error = Error;
    fn try_from(value: &{ref_type}) -> Result<Self, Self::Error> {{
        Ok(Self({}::try_from(value)?))
    }}
}}
"##,
            &schema_type.name
        )?;
    }

    Ok(())
}

fn format_prop_struct(buffer: &mut File, name: &str, props_string: Option<String>) -> Result<()> {
    writeln!(buffer, r##"#[serde(rename_all = "kebab-case")]"##)?;
    writeln!(buffer, "pub struct {} {{", name)?;

    if let Some(props) = props_string {
        writeln!(buffer, "{}", &props)?;
    }

    writeln!(buffer)?;
    writeln!(buffer, "}}")?;
    Ok(())
}
