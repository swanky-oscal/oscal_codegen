use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn generate_mod(path: &PathBuf, mods: &[&str]) -> Result<()> {
    let mut file_path = path.to_owned();
    file_path.push("mod.rs");
    let mut buffer = File::create(file_path)?;

    for mod_name in mods.iter() {
        writeln!(buffer, "pub mod {};", mod_name)?;
    }

    Ok(())
}
