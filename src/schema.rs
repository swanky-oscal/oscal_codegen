#![allow(dead_code)]
use anyhow::Result;
use indexmap::{map::Iter, IndexMap};

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

pub struct Schema {
    pub schema: String,
    pub version: String,
    pub comment: String,
    tree: IndexMap<String, TreeEntry>,
}

impl Schema {
    pub fn new(schema: &str, version: &str, comment: &str) -> Self {
        Self {
            schema: schema.to_owned(),
            version: version.to_owned(),
            comment: comment.to_owned(),
            tree: IndexMap::new(),
        }
    }
    pub fn iter(&self) -> Iter<'_, String, TreeEntry> {
        self.tree.iter()
    }

    pub fn keys(&self) -> indexmap::map::Keys<'_, String, TreeEntry> {
        self.tree.keys()
    }
    /// The schema only has a single TreeEntry object
    pub fn is_reducable(&self, name: &str) -> bool {
        if self.tree.len() != 1 {
            return false;
        }

        let Some((key, value)) = self.tree.get_index(0) else {
            return false;
        };

        if value.is_object() {
            return key == name;
        }

        false
    }

    pub fn has(&self, key: &str) -> bool {
        self.tree.contains_key(key)
    }

    pub fn get(&self, name: &str) -> Result<&TreeEntry> {
        self.tree
            .get(name)
            .ok_or(Error::EntryNotFound(name.to_owned()).into())
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut TreeEntry> {
        self.tree
            .get_mut(name)
            .ok_or(Error::EntryNotFound(name.to_owned()).into())
    }

    pub fn add_object(&mut self, name: &str, obj: &SchemaObject) -> Result<()> {
        self.tree
            .insert(name.to_owned(), TreeEntry::Object(Box::new(obj.to_owned())));
        Ok(())
    }

    pub fn get_object(&self, name: &str) -> Result<&SchemaObject> {
        self.get(name)?.as_object()
    }

    pub fn add_tree(&mut self, name: &str) -> Result<()> {
        self.tree
            .insert(name.to_owned(), TreeEntry::Tree(Self::new("", "", "")));

        Ok(())
    }
    pub fn get_tree(&self, name: &str) -> Result<&Self> {
        self.get(name)?.as_tree()
    }

    pub fn get_tree_mut(&mut self, name: &str) -> Result<&mut Self> {
        self.get_mut(name)?.as_tree_mut()
    }

    pub fn get_or_add_tree(&mut self, name: &str) -> Result<&mut Self> {
        if self.has(name) {
            return self.get_tree_mut(name);
        }
        self.add_tree(name)?;
        self.get_tree_mut(name)
    }
}
