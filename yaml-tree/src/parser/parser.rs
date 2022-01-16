/// parser crate to implement the core implementation
/// of the Yaml parser
///
/// author: https://github.com/vincenzopalazzo
use std::vec::Vec;

use super::{YamlNode, YamlParser};
use crate::scanner::tokens;

struct Parser {
    pub ir: Vec<YamlNode>,
}

/// Core implementation of the Yaml Parser.
impl Parser {
    fn walk(&mut self, tokens: &Vec<tokens::YamlToken>) {}
}

/// Common implementation of the Yaml Parser
impl YamlParser<YamlNode> for Parser {
    fn parse(&mut self, tokens: &Vec<tokens::YamlToken>) -> &Vec<YamlNode> {
        self.walk(tokens);
        &self.ir
    }
}
