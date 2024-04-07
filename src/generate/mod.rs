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

use crate::{Resolver, Schema};

pub fn generate(
    path: PathBuf,
    static_input: PathBuf,
    tree: &Schema,
    resolver: &Resolver,
) -> Result<()> {
    let src_path = Path::new(&path).join("src").to_path_buf();
    let lib_path = Path::new(&src_path).join("lib.rs").to_path_buf();

    generate_schema(&src_path, tree, resolver)?;

    let mut buffer = File::create(lib_path)?;

    generate_lib_header(&mut buffer)?;

    writeln!(
        buffer,
        "/// The OSCAL schema version this code was generated from"
    )?;
    writeln!(
        buffer,
        r#"pub static SCHEMA_VERSION: &str = "{}";"#,
        tree.version
    )?;
    writeln!(buffer)?;

    for mod_name in tree.keys() {
        writeln!(buffer, "pub mod {};", mod_name)?;
    }
    writeln!(buffer, "pub mod error;")?;

    generate_oscal_document(&mut buffer)?;

    generate_static(&path, &static_input)?;
    generate_cargo(path)?;
    Ok(())
}

fn generate_lib_header(buffer: &mut File) -> Result<()> {
    writeln!(
        buffer,
        r##"//! OSCAL Lib
//!
//! This file was auto-generated at {}"##,
        oscal_types::DateTimeDatatype::new().to_rfc2822()
    )?;
    writeln!(buffer)?;

    Ok(())
}

fn generate_oscal_document(buffer: &mut File) -> Result<()> {
    writeln!(
        buffer,
        r##"

use serde::{{Deserialize, Serialize}};
use serde_with::skip_serializing_none;

use crate::{{
    oscal_ap::assessment_plan::AssessmentPlan, oscal_ar::assessment_results::AssessmentResults,
    oscal_catalog::catalog::Catalog,
    oscal_component_definition::component_definition::ComponentDefinition,
    oscal_poam::plan_of_action_and_milestones::PlanOfActionAndMilestones,
    oscal_profile::profile::Profile,
    oscal_ssp::system_security_plan::SystemSecurityPlan,
}};

    "##
    )?;
    writeln!(
        buffer,
        r##"#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OscalDocumentType {{
    Catalog(Box<Catalog>),
    Profile(Box<Profile>),
    ComponentDefinition(Box<ComponentDefinition>),
    SystemSecurityPlan(Box<SystemSecurityPlan>),
    AssessmentPlan(Box<AssessmentPlan>),
    AssessmentResults(Box<AssessmentResults>),
    PlanOfActionAndMilestones(Box<PlanOfActionAndMilestones>),
}}
"##
    )?;

    writeln!(buffer)?;

    writeln!(
        buffer,
        r##"#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OscalDocument {{
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(flatten)]
    pub document: OscalDocumentType
}}
    "##
    )?;
    Ok(())
}
