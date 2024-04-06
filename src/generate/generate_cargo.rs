use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn generate_cargo(path: PathBuf) -> Result<()> {
    let cargo_path = Path::new(&path).join("Cargo.toml").to_path_buf();

    let mut buffer = File::create(cargo_path)?;

    writeln!(buffer, r#"[package]"#)?;
    writeln!(buffer, r#"name = "oscal_lib""#)?;
    writeln!(buffer, r#"version = "0.1.1""#)?;
    writeln!(buffer, r#"edition = "2021""#)?;
    writeln!(
        buffer,
        r#"authors = ["David Skyberg <davidskyberg@gmail.com"]"#
    )?;
    writeln!(buffer, r#"description = "OSCAL lib in Rust""#)?;
    writeln!(buffer, r#"keywords = ["rust", "oscal"]"#)?;
    writeln!(
        buffer,
        r#"repository = "https://github.com/dskyberg/oscal""#
    )?;
    writeln!(buffer, r#"license = "MIT OR Apache 2.0""#)?;
    writeln!(buffer)?;
    writeln!(buffer, r#"[dependencies]"#)?;
    writeln!(
        buffer,
        r#"oscal_types = {{ git = "https://github.com/dskyberg/oscal_types.git" }}"#
    )?;
    writeln!(
        buffer,
        r#"chrono = {{ version = "0.4", features = ["serde", "alloc", "now", "serde"] }}"#
    )?;
    writeln!(
        buffer,
        r#"semver = {{ version = "1.0.18", features = ["serde"] }}"#
    )?;
    writeln!(
        buffer,
        r#"serde = {{ version = "1.0.171", features = ["derive"] }}"#
    )?;
    writeln!(buffer, r#"serde_json = "1.0.103""#)?;
    writeln!(
        buffer,
        r#"serde_with = {{ version = "3.0.0", features = ["chrono"] }}"#
    )?;
    writeln!(buffer, r#"thiserror = "1.0.43""#)?;

    Ok(())
}
