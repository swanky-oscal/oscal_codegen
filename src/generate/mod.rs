use generate_cargo::*;
use generate_mod::*;
use generate_props::*;
use generate_schema::*;
use generate_static::*;
use generate_struct::*;
pub use name_space::Namespace;

mod generate_cargo;
mod generate_mod;
mod generate_props;
mod generate_schema;
mod generate_static;
mod generate_struct;
mod name_space;

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::{Resolver, SchemaTree};

pub fn generate(
    path: PathBuf,
    static_input: PathBuf,
    tree: &SchemaTree,
    resolver: &Resolver,
) -> Result<()> {
    let src_path = Path::new(&path).join("src").to_path_buf();
    let lib_path = Path::new(&src_path).join("lib.rs").to_path_buf();

    generate_schema(&src_path, tree, resolver)?;

    let mut buffer = File::create(lib_path)?;

    for mod_name in tree.keys() {
        writeln!(buffer, "pub mod {};", mod_name)?;
    }
    writeln!(buffer, "pub mod error;")?;

    generate_static(&path, &static_input)?;
    generate_cargo(path)?;
    Ok(())
}
