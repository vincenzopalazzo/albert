//! Code Generator implementation
//!
//! author: https://github.com/vincenzopalazzo
use std::io;

use crate::{CodeGenStrategy, DataModel};

pub struct Golang;

impl<'a> CodeGenStrategy<'a> for Golang {
    fn generate(&self, model: &'a DataModel) -> Result<(), io::Error> {
        Ok(())
    }
}
