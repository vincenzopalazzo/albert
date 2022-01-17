//! Code Generation crate
//! Take in input a Data model implementation like Yaml, ecc
//! and run a strategy to generate the source code that the
//! the user want.
//!
//! author: https://github.com/vincenzopalazzo
use std::{io, vec};

use monkey_yaml::parser::YamlNode;

pub mod golang;

/// Data model enum with all type of Data model
/// supported by the code gen module.
pub enum DataModel<'a> {
    Yaml(&'a vec::Vec<YamlNode>),
    JsonSchema,
    Json,
}

/// Code Gene Strategy traits
pub trait CodeGenStrategy<'a> {
    /// generic method to generate source code
    /// from a Data Model Abstraction
    fn generate(&'a self, model: &'a DataModel) -> Result<(), io::Error>;
}
