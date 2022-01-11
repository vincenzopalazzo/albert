/// Main Yaml scanner crate
///
/// author: https://github.com/vincenzopalazzo
use std::vec::Vec;

pub mod scanner;

/// YAML token definition during the parsing
/// of the YAML file
pub enum YamlToken {
    EOF,
}

trait YamlScanner<T> {
    fn scan(&mut self, content: &str) -> &Vec<T>;
}
