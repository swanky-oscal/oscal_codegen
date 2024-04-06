#![allow(dead_code)]
use anyhow::Result;
use indexmap::{map::Iter, IndexMap};
use serde::Serialize;

use crate::{SchemaObject, TreeEntry};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Entry not found: {0}")]
    EntryNotFound(String),
    #[error("Entry is not an object: {0}")]
    NotObject(String),
    #[error("Entry is not a tree: {0}")]
    NotTree(String),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SchemaTree(IndexMap<String, TreeEntry>);

impl SchemaTree {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn iter(&self) -> Iter<'_, String, TreeEntry> {
        self.0.iter()
    }

    pub fn keys(&self) -> indexmap::map::Keys<'_, String, TreeEntry> {
        self.0.keys()
    }
    /// The schema only has a single TreeEntry object
    pub fn is_reducable(&self, name: &str) -> bool {
        if self.0.len() != 1 {
            return false;
        }

        let Some((key, value)) = self.0.get_index(0) else {
            return false;
        };

        if value.is_object() {
            return key == name;
        }

        false
    }

    pub fn has(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn get(&self, name: &str) -> Result<&TreeEntry> {
        self.0
            .get(name)
            .ok_or(Error::EntryNotFound(name.to_owned()).into())
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut TreeEntry> {
        self.0
            .get_mut(name)
            .ok_or(Error::EntryNotFound(name.to_owned()).into())
    }

    pub fn add_object(&mut self, name: &str, obj: &SchemaObject) -> Result<()> {
        self.0
            .insert(name.to_owned(), TreeEntry::Object(Box::new(obj.to_owned())));
        Ok(())
    }

    pub fn get_object(&self, name: &str) -> Result<&SchemaObject> {
        self.get(name)?.as_object()
    }

    pub fn add_tree(&mut self, name: &str) -> Result<()> {
        self.0.insert(name.to_owned(), TreeEntry::Tree(Self::new()));

        Ok(())
    }
    pub fn get_tree(&self, name: &str) -> Result<&SchemaTree> {
        self.get(name)?.as_tree()
    }

    pub fn get_tree_mut(&mut self, name: &str) -> Result<&mut SchemaTree> {
        self.get_mut(name)?.as_tree_mut()
    }

    pub fn get_or_add_tree(&mut self, name: &str) -> Result<&mut SchemaTree> {
        if self.has(name) {
            return self.get_tree_mut(name);
        }
        self.add_tree(name)?;
        self.get_tree_mut(name)
    }
}
