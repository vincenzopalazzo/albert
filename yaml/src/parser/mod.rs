/// Main Yaml parser crate
///
/// author: https://github.com/vincenzopalazzo
use std::vec::Vec;

use crate::scanner::tokens::YamlToken;

pub mod parser;
pub mod tokens;

/// Interface Yaml Interface
trait YamlParser<T> {
    fn parse(&mut self, tokens: &Vec<YamlToken>) -> &Vec<T>;
}
