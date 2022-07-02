//! Main Yaml parser crate
//!
//! author: https://github.com/vincenzopalazzo
use std::error::Error;
use std::fmt;
use std::fmt::{write, Formatter};
use std::vec::Vec;

use crate::scanner::tokens::YamlToken;

pub mod parser;
pub mod tokens;

/// Interface Yaml Interface
trait YamlParser<T> {
    fn parse(&mut self, tokens: &Vec<YamlToken>) -> &Vec<T>;
}

#[derive(Debug)]
pub struct YamlParserError {
    cause: String,
}

impl Error for YamlParserError {}

impl fmt::Display for YamlParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}
