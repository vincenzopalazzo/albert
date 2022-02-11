/// parser crate to implement the core implementation
/// of the Yaml parser
///
/// author: https://github.com/vincenzopalazzo
use std::vec::Vec;

use super::tokens::{YamlObject, YamlScope};
use super::YamlParser;
use crate::scanner::tokens::YamlToken;

pub struct Parser {
    ir: Vec<YamlObject>,
    current_pos: u32,
}

/// Core implementation of the Yaml Parser.
impl Parser {
    fn new() -> Self {
        Parser {
            ir: vec![],
            current_pos: 0,
        }
    }

    /// Check for scope definition in the tokens stream
    /// and return a flag with the scope that the parser found
    fn parse_scope(&self, tokens: &Vec<YamlToken>) -> YamlScope {
        YamlScope::InvalidScope("TODO: implement this part".to_string())
    }

    /// Walk through the list of tokens
    fn walk(&mut self, tokens: &Vec<YamlToken>) {}
}

/// Common implementation of the Yaml Parser
impl YamlParser<YamlObject> for Parser {
    fn parse(&mut self, tokens: &Vec<YamlToken>) -> &Vec<YamlObject> {
        self.walk(tokens);
        &self.ir
    }
}
