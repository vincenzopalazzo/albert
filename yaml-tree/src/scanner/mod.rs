/// TODO: adding doc here

/// YAML token definition during the parsing
/// of the YAML file
use std::vec::Vec;

pub mod scanner;

enum YamlToken {
    EOF,
}

trait YamlScanner<T> {
    fn scan(&self, content: &str) -> &Vec<T>;
}
