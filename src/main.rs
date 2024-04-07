use anyhow::Result;
use clap::Parser;
use serde_json::Value;
use std::fs::{self, remove_dir_all};
use std::path::PathBuf;

use any_of::*;
use data_type::*;
use error::Error;
use generate::*;
use parse::*;
use property::*;
use resolver::*;
use schema::*;
use schema_object::*;
use schema_type::*;
use string_type::*;
use tree_entry::*;
use type_ref::*;
use util::*;

mod any_of;
mod data_type;
mod error;
mod generate;
mod parse;
mod property;
mod resolver;
mod schema;
mod schema_object;
mod schema_type;
mod string_type;
mod tree_entry;
mod type_ref;
mod util;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The input schema file
    #[arg(short, long, value_name = "FILE")]
    schema: PathBuf,
    /// Where to put the output
    #[arg(short, long, value_name = "DIR", default_value = "./output")]
    output: PathBuf,
    /// Delete output folder first
    #[arg(long)]
    remove: bool,
    /// Where to get the static content
    #[arg(short, long, value_name = "DIR", default_value = "./static")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.remove {
        // Don't worry about the result.  It will throw an error
        // if the folder doesn't exist.
        let _ = remove_dir_all(&cli.output);
    }

    let mut resolver = Resolver::new();
    let json_path = cli.schema;
    let schema_file = fs::read_to_string(json_path)?;
    let json = serde_json::from_str::<Value>(&schema_file)?;

    let schema = parse_schema(&json, &mut resolver)?;

    generate(cli.output, cli.input, &schema, &resolver)?;

    Ok(())
}
