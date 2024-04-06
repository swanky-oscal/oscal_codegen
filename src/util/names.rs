#![allow(dead_code)]
/// Top level entries in #definitions the have the structure of either
///
///  `<Rustable-name>`
/// `
/// Exammple:
///
/// `StringDatatype``
///
/// or
///
/// `(model name)-(namespace):(namespace)`
///
/// Example:
/// - `oscal-complete-oscal-catalog:catalog`
/// - `oscal-complete-oscal-ap:assessment-plan`
/// - `oscal-ap-oscal-ap:assessment-plan`
///
/// For the firt case, convert the input name into snake_case:
///
/// `StringDatatype -> string_datatype`
///
///  This creates the schema reference:
///  - ns:  `name.to_case(Case::Snake)` -> `string_datatype`
/// - name: `name` -> `StringDatatype`
///
/// For the second type, it's a 3 part job:
///  #: Remove the model name
///  #: Split the namespaces at the colon
///  #: Fix up the cases for the namespace and Rust type
///
/// Example: `oscal-ap-oscal-ap:assessment-plan`
/// # Remove the model name: -> `oscal-ap:assessment-plan`
/// # Split the namespaces at the colon: `["oscal-ap", "assessment-plan"]`
/// # Fix up the cases
///     - left ns: parts[0].to_case(Case::Snake) -> oscal_ap
///     - right ns: parts[1].to_case(Case::Snake) -> assessment_plan
///     - name: parts[1].to_case(Case::Pascal) -> AssessmentPlan
///  The resulting Rust module path becomes:
///
/// `oscal_ap::assessment_plan::AssessmentPlan`
use convert_case::{Case, Casing};

pub static OSCAL_COMPLETE: &str = "oscal-complete-";

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceEntry {
    pub left: String,
    pub right: String,
    pub rust: String,
}

impl From<&str> for NamespaceEntry {
    fn from(name: &str) -> Self {
        if is_namespace(name) {
            let parts = name_parts(name);
            return NamespaceEntry {
                left: parts[0].to_case(Case::Snake),
                right: parts[1].to_case(Case::Snake),
                rust: parts[1].to_case(Case::Pascal),
            };
        }
        NamespaceEntry {
            left: "".to_string(),
            right: name.to_case(Case::Snake),
            rust: name.to_owned(),
        }
    }
}

/// Strip the model name, and break the result name at the ':'
pub fn name_parts(name: &str) -> Vec<&str> {
    let name = strip_oscal(name);
    let names = name.split(':').collect::<Vec<&str>>();
    names
}

/// Strip off the starting oscal-complete-oscal-
pub fn strip_oscal(name: &str) -> &str {
    if !name.starts_with(OSCAL_COMPLETE) {
        return name;
    }
    &name[OSCAL_COMPLETE.len()..]
}

fn is_namespace(name: &str) -> bool {
    name.starts_with(OSCAL_COMPLETE) && name.contains(':')
}

pub trait UnPlural {
    fn unplural(&self) -> String;
}

impl UnPlural for &str {
    fn unplural(&self) -> String {
        match self.chars().nth(self.len() - 1) {
            Some('s') => self[..self.len() - 1].to_string(),
            _ => self.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_plural() {
        assert_eq!(
            "assessment_assets".unplural(),
            "assessment_asset".to_owned()
        );
    }

    #[test]
    fn test_is_namespace() {
        assert!(is_namespace("oscal-complete-oscal-ap:assessment-plan"));

        assert!(!is_namespace("StringDatatype"));
    }

    #[test]
    fn test_strip_oscal() {
        assert_eq!(
            strip_oscal("oscal-complete-oscal-ap:assessment-plan"),
            "oscal-ap:assessment-plan"
        );
    }

    #[test]
    fn test_name_parts() {
        assert_eq!(
            name_parts("oscal-complete-oscal-ap:assessment-plan"),
            vec!["oscal-ap", "assessment-plan"]
        )
    }

    #[test]
    fn test_name_rutify_namespace() {
        assert_eq!(
            NamespaceEntry::from("oscal-complete-oscal-ap:assessment-plan"),
            NamespaceEntry {
                left: String::from("oscal_ap"),
                right: String::from("assessment_plan"),
                rust: String::from("AssessmentPlan")
            }
        );
    }
    #[test]
    fn test_name_rutify_not_namespace() {
        assert_eq!(
            NamespaceEntry::from("StringDatatype"),
            NamespaceEntry {
                left: String::from(""),
                right: String::from("string_datatype"),
                rust: String::from("StringDatatype")
            }
        );
    }
}
