/// Main Yaml scanner crate
///
/// author: https://github.com/vincenzopalazzo
use std::vec::Vec;

pub mod scanner;
pub mod tokens;

/// Yaml Scanner interface
pub trait YamlScanner<T> {
    /// Create a new instance of the Yaml Scanner
    fn new() -> Self;

    /// Run the scan of the yaml content provided as str
    fn scan(&mut self, content: &str) -> &Vec<T>;
}
