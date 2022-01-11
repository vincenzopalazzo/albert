/// Core implementation of the YAML scanner
///
/// author: https://github.com/vincenzopalazzo
use super::{YamlScanner, YamlToken};

/// Core implementation of the scanner
struct Scanner {
    pos: u64,
    tokens: Vec<YamlToken>,
}

impl Scanner {
    fn run(&self, content: &str) {}
}

/// YamlScanner implementation for the Scanner struct
impl YamlScanner<YamlToken> for Scanner {
    fn scan(&mut self, content: &str) -> &Vec<YamlToken> {
        self.run(content);
        &self.tokens
    }
}
