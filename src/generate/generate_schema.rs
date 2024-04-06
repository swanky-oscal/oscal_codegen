use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{Resolver, SchemaTree, TreeEntry};

use super::{generate_mod, generate_struct};

pub fn generate_schema(path: &PathBuf, tree: &SchemaTree, resolver: &Resolver) -> Result<()> {
    let mut mods: Vec<&str> = vec![];
    let mut mods_done = false;

    create_dir_all(path)?;

    // Assemble the mod info
    for (name, _) in tree.iter() {
        mods.push(name);
    }

    for (name, entry) in tree.iter() {
        match entry {
            TreeEntry::Object(obj) => {
                if path.ends_with(name) {
                    // parent::parent should be parent/mod.rs
                    generate_struct(path, name, true, obj, Some(&mods), resolver)?;
                    mods_done = true;
                } else {
                    // parent::child should be parent/child.rs
                    generate_struct(path, name, false, obj, None, resolver)?;
                }
            }

            TreeEntry::Tree(tree) => {
                // Reduce Tree(name)/Object(name) to Object(name)
                if tree.is_reducable(name) {
                    let obj = tree.get_object(name)?;
                    generate_struct(path, name, false, obj, None, resolver)?;
                    continue;
                }

                let path = Path::new(&path).join(name).to_path_buf();

                generate_schema(&path, tree, resolver)?;
            }
        }
    }

    if mods_done {
        return Ok(());
    }

    generate_mod(path, &mods)
}
