#![allow(dead_code)]
use anyhow::Result;
use serde::Serialize;

use crate::{SchemaObject, SchemaTree};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
enum Error {
    #[error("Not an object")]
    NotObject,
    #[error("Not a tree")]
    NotTree,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TreeEntry {
    Object(Box<SchemaObject>),
    Tree(SchemaTree),
}

impl TreeEntry {
    pub fn is_object(&self) -> bool {
        matches!(self, TreeEntry::Object(_))
    }
    pub fn is_tree(&self) -> bool {
        matches!(self, TreeEntry::Tree(_))
    }

    pub fn as_object(&self) -> Result<&SchemaObject> {
        match self {
            TreeEntry::Object(obj) => Ok(obj),
            _ => Err(Error::NotObject.into()),
        }
    }

    pub fn as_tree(&self) -> Result<&SchemaTree> {
        match self {
            TreeEntry::Tree(tree) => Ok(tree),
            _ => Err(Error::NotTree.into()),
        }
    }
    pub fn as_tree_mut(&mut self) -> Result<&mut SchemaTree> {
        match self {
            TreeEntry::Tree(tree) => Ok(tree),
            _ => Err(Error::NotTree.into()),
        }
    }
}
