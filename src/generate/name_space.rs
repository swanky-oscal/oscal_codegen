use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::BTreeMap};

use crate::SchemaType;

/// Namespaces for struct properties
/// Collected in generate_struct, and used for generating
/// `use` statements
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Namespace {
    pub name: String,
    pub entries: Vec<String>,
    pub subs: BTreeMap<String, Namespace>,
}

impl Namespace {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            entries: Vec::new(),
            subs: BTreeMap::new(),
        }
    }

    pub fn cmp(ns: &str, super_ns: &str) -> Option<Ordering> {
        if ns.is_empty() {
            return Some(Ordering::Less);
        }

        let ns_parts = ns.split("::").collect::<Vec<&str>>();
        let super_parts = super_ns.split("::").collect::<Vec<&str>>();

        match std::iter::zip(&ns_parts, &super_parts).fold(true, |acc, (l, r)| acc && *l == *r) {
            false => None,
            true => Some(ns_parts.len().cmp(&super_parts.len())),
        }
    }

    pub fn add_type(&mut self, schema_type: &SchemaType) -> Result<()> {
        // If everything is empty, this is probably a Rust type, such as String.
        // Let's try just skipping altogether.
        if self.name.is_empty() && schema_type.ns.is_empty() {
            /*
            if !self.entries.contains(&schema_type.name) {
                self.entries.push(schema_type.name.clone());
            }
            */
            return Ok(());
        }
        let mut ns = schema_type.ns.split("::").collect::<Vec<&str>>();
        ns.reverse();
        self.add(&mut ns, &schema_type.name)?;
        Ok(())
    }

    pub fn add(&mut self, ns: &mut Vec<&str>, name: &str) -> Result<()> {
        // If we are at the end of the line, add the name here
        if ns.is_empty() {
            if !self.entries.iter().any(|x| x == name) {
                self.entries.push(name.to_owned());
            }
            return Ok(());
        }

        // Else, descend
        let branch = ns.pop().unwrap(); // safe to unwrap

        match self.subs.get_mut(branch) {
            Some(sub) => sub.add(ns, name),

            None => {
                // Add a new namespace and descend
                let mut sub = Self::new(branch);
                sub.add(ns, name)?;
                self.subs.insert(branch.to_owned(), sub);
                Ok(())
            }
        }
    }

    pub fn use_types(&self) -> Option<String> {
        let Some(types_ns) = self.subs.get("oscal_types") else {
            return None;
        };
        let entries = types_ns.entries.join(", ");
        match types_ns.entries.len() {
            1 => Some(entries),
            _ => Some(format!("{{ {} }}", entries)),
        }
    }

    pub fn use_crates(&self, super_ns: &str, parent_ns: &str) -> String {
        let ns = match parent_ns.is_empty() {
            true => self.name.clone(),
            false => format!("{}::{}", parent_ns, &self.name),
        };

        if !super_ns.is_empty() && super_ns == ns {
            return String::from("");
        }

        let mut parts = vec![];

        if !self.entries.is_empty() {
            parts.push(self.entries.join(", "));
        }

        let mut subs = self
            .subs
            .values()
            .filter(|ns| ns.name != "oscal_types")
            .map(|sub| sub.use_crates(super_ns, &ns))
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();
        parts.append(&mut subs);

        let mut use_statement = String::from("");
        if parts.is_empty() {
            return use_statement;
        }

        if !self.name.is_empty() {
            use_statement.push_str(&format!("{}::", &self.name));
        }

        if parts.len() > 1 {
            use_statement.push('{');
        }

        use_statement.push_str(&parts.join(", "));

        if parts.len() > 1 {
            use_statement.push('}');
        }

        use_statement
    }

    pub fn use_supers(&self, super_ns: &str, parent_name: &str, parent_ns: &str) -> Result<String> {
        // If there is no parent namespace, then this is the top level.
        // The namespace is whatever self is.
        let ns = match parent_ns.is_empty() {
            true => self.name.clone(),
            false => format!("{}::{}", parent_ns, &self.name),
        };

        let ns_cmp = Namespace::cmp(&ns, super_ns);
        if ns_cmp.is_none() {
            println!("Bailing: {} is not in {}", &ns, super_ns);
            return Ok(String::new());
        }

        // If this namespace is part of the super_ns, then there must be a single sub
        // that continues the namespace.
        if ns_cmp == Some(Ordering::Less) {
            // There will be a single sub that matches.
            for sub_name in self.subs.keys() {
                let sub_ns = match ns.is_empty() {
                    true => sub_name.to_owned(),
                    false => format!("{}::{}", &ns, sub_name),
                };
                match Namespace::cmp(&sub_ns, super_ns) {
                    None => {
                        continue;
                    }
                    Some(_) => {
                        let sub_val = self.subs.get(sub_name).unwrap();
                        return sub_val.use_supers(super_ns, parent_name, &ns);
                    }
                }
            }

            return Ok(String::new());
        }

        // If we are here, then this namespace is part of the super
        let mut parts = vec![];
        if !self.entries.is_empty() {
            parts.push(
                self.entries
                    .iter()
                    .filter(|name| !name.is_empty() && *name != parent_name)
                    .map(|name| name.to_owned())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        }

        let mut subs = vec![];
        for (key, sub) in &self.subs {
            let sub_ns = format!("{}::{}", &ns, key);
            let result = sub.use_supers(super_ns, parent_name, &sub_ns);
            if let Ok(value) = result {
                subs.push(value.clone());
            }
        }

        if !subs.is_empty() {
            let mut s = subs
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            if !s.is_empty() {
                parts.append(&mut s);
            }
        }

        if parts.is_empty() {
            return Ok(String::new());
        }

        let parts = parts
            .iter()
            .filter(|part| !part.is_empty())
            .map(|part| part.to_owned())
            .collect::<Vec<String>>();

        let mut use_statement = String::from("");

        if ns_cmp != Some(Ordering::Equal) {
            use_statement.push_str(&format!("{}::", &self.name));
        }

        if parts.len() > 1 {
            use_statement.push('{');
        }

        use_statement.push_str(&parts.join(", "));

        if parts.len() > 1 {
            use_statement.push('}');
        }

        Ok(use_statement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_namespace() {
        let namespace = test_data();
        println!(
            "{}",
            serde_json::to_string_pretty(&namespace).expect("oops")
        );
    }
    #[test]
    fn test_complex_supers() {
        let namespace = test_json_data_1();
        let super_ns = "oscal_ap::assessment_plan::local_definitions";
        let use_supers = namespace.use_supers(super_ns, "", "").expect("oops");
        println!("{}", &use_supers);
    }

    #[test]
    fn test_top_level_entry() {
        let schema_type = SchemaType::new("", "UUIDDatatype");
        let mut namespace = Namespace::new("");
        namespace.add_type(&schema_type).expect("oops");

        assert_eq!(namespace.use_crates("", ""), "UUIDDatatype");

        println!(
            "{}",
            namespace
                .use_supers("oscal_ar::assessment_results", "", "")
                .expect("oops")
        );
    }

    #[test]
    fn test_namespace_cmp() {
        let super_ns = "a::b";
        let empty_ns = "";
        let lt_ns = "a";
        let gt_ns = "a::b::c";
        let bad_ns = "c";

        assert_eq!(Namespace::cmp(empty_ns, super_ns), Some(Ordering::Less));
        assert_eq!(Namespace::cmp(lt_ns, super_ns), Some(Ordering::Less));
        assert_eq!(Namespace::cmp(gt_ns, super_ns), Some(Ordering::Greater));
        assert_eq!(Namespace::cmp(super_ns, super_ns), Some(Ordering::Equal));
        assert_eq!(Namespace::cmp(bad_ns, super_ns), None);
    }

    #[test]
    fn test_use_supers() {
        let ns = "oscal_ar::assessment_results";
        let name = "AssessmentResults";
        let schema_type = SchemaType::new(ns, name);

        let mut namespace = Namespace::new("");
        namespace.add_type(&schema_type).expect("oops");

        let ns = "oscal_ar::assessment_results::local_definitions";
        let name = "LocalDefinitions";
        let schema_type = SchemaType::new(ns, name);
        namespace.add_type(&schema_type).expect("oops");

        // use crate::oscal_ar::assessment_results::{AssessmentResults, local_definitions::LocalDefinitions};
        println!(
            "{}",
            &namespace
                .use_supers("oscal_ar::assessment_results", "", "")
                .expect("oops")
        );
    }

    #[test]
    fn test_use_crates() {
        let ns = "oscal_ar::assessment_results";
        let name = "AssessmentResults";
        let schema_type = SchemaType::new(ns, name);

        let mut namespace = Namespace::new("");
        namespace.add_type(&schema_type).expect("oops");

        let ns = "oscal_ar::assessment_results::local_definitions";
        let name = "LocalDefinitions";
        let schema_type = SchemaType::new(ns, name);
        namespace.add_type(&schema_type).expect("oops");

        assert_eq!("use crate::oscal_ar::assessment_results::{AssessmentResults, local_definitions::LocalDefinitions};",
         &namespace.use_crates("", ""));
    }

    fn test_json_data_1() -> Namespace {
        let json = r##"{
            "name": "",
            "entries": [],
            "subs": {
              "oscal_assessment_common": {
                "name": "oscal_assessment_common",
                "entries": [],
                "subs": {
                  "activity": {
                    "name": "activity",
                    "entries": [
                      "Activity"
                    ],
                    "subs": {}
                  },
                  "local_objective": {
                    "name": "local_objective",
                    "entries": [
                      "LocalObjective"
                    ],
                    "subs": {}
                  }
                }
              },
              "oscal_implementation_common": {
                "name": "oscal_implementation_common",
                "entries": [],
                "subs": {
                  "inventory_item": {
                    "name": "inventory_item",
                    "entries": [
                      "InventoryItem"
                    ],
                    "subs": {}
                  },
                  "system_component": {
                    "name": "system_component",
                    "entries": [
                      "SystemComponent"
                    ],
                    "subs": {}
                  },
                  "system_user": {
                    "name": "system_user",
                    "entries": [
                      "SystemUser"
                    ],
                    "subs": {}
                  }
                }
              },
              "oscal_metadata": {
                "name": "oscal_metadata",
                "entries": [],
                "subs": {
                  "remarks": {
                    "name": "remarks",
                    "entries": [
                      "Remarks"
                    ],
                    "subs": {}
                  }
                }
              }
            }
          }
          "##;
        serde_json::from_str::<Namespace>(json).expect("oops")
    }

    fn test_data() -> Namespace {
        let mut namespace = Namespace::new("");

        namespace
            .add_type(&SchemaType::new(
                "oscal_assessment_common::assessment_assets",
                "AssessmentAssets",
            ))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new(
                "oscal_assessment_common::assessment_subject",
                "AssessmentSubject",
            ))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new(
                "oscal_metadata::back_matter",
                "BackMatter",
            ))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new(
                "oscal_assessment_common::import_ssp",
                "ImportSsp",
            ))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new(
                "oscal_ap::assessment_plan::local_definitions",
                "LocalDefinitions",
            ))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new("oscal_metadata::metadata", "Metadata"))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new(
                "oscal_assessment_common::reviewed_controls",
                "ReviewedControls",
            ))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new("oscal_assessment_common::task", "Task"))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new(
                "oscal_ap::assessment_plan::terms_and_conditions",
                "TermsAndConditions",
            ))
            .expect("failed to add type");
        namespace
            .add_type(&SchemaType::new("", "UUIDDatatype"))
            .expect("failed to add type");
        namespace
    }
}
