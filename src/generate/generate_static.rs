use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate_static(output_path: &Path, static_path: &Path) -> Result<()> {
    let error_output_path = output_path.join("src/error.rs");
    let error_static_path = static_path.join("src/error.rs");
    fs::copy(error_static_path, error_output_path)?;

    Ok(())
}
