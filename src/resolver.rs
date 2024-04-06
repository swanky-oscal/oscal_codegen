use std::collections::BTreeMap;

use crate::{SchemaType, TypeRef, DATA_TYPES};

pub struct Resolver {
    map: BTreeMap<String, SchemaType>,
}

impl Resolver {
    pub fn new() -> Self {
        let mut result = Self {
            map: BTreeMap::new(),
        };
        load_data_types(&mut result);
        result
    }

    pub fn add_type(&mut self, id: &str, type_ref: SchemaType) {
        self.map.insert(id.to_owned(), type_ref);
    }

    pub fn resolve(&self, type_ref: &TypeRef) -> Option<SchemaType> {
        // If the type_ref contains a _ref,
        if let Some(_ref) = &type_ref._ref {
            return self.map.get(_ref).cloned();
        } else if let Some(_type) = &type_ref._type {
            return Some(_type.clone());
        }
        None
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

fn load_data_types(map: &mut Resolver) {
    for data_type in DATA_TYPES {
        map.add_type(
            &format!("#/definitions/{}", data_type),
            SchemaType::new("oscal_types", data_type),
        );
    }
}
