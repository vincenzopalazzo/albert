/// Core implementation of the YAML scanner
///
/// author: https://github.com/vincenzopalazzo
use super::{tokens, YamlScanner};

/// Core implementation of the scanner
struct Scanner {
    pos: usize,
    line: u64,
    tokens: Vec<tokens::YamlToken>,
}

impl Scanner {
    /// Entry point of the scanner to parser
    /// the string and return a list of token
    fn run(&mut self, content: &str) {
        while !self.is_at_end(&content) {
            self.contrue_token(&content);
        }
    }

    /// Core function to try to understand the
    /// core function of the scanner too contrue
    /// the token from the content
    fn contrue_token(&mut self, content: &str) {
        let char_token = self.next(&content);
        match char_token {
            ':' => self.add_token(tokens::YamlToken::DotDot),
            '#' => self.parse_comment_line(&content),
            _ => panic!("pos {} with val {}", self.pos, char_token),
        };
    }

    /// Function to parse the comment line and store
    /// the content of the comment inside the function
    fn parse_comment_line(&mut self, content: &str) {
        let mut line_comment = String::from("");
        let mut start_comment = self.pos;

        while content.chars().nth(start_comment).unwrap() != '\n' {
            line_comment.push(content.chars().nth(start_comment).unwrap());
            start_comment += 1;
        }
        println!("pos {} with val {}", self.pos, line_comment);
        let token = tokens::YamlToken::Pount(line_comment);
        self.add_token(token);
        // with increase the value by one because we want
        // skip the \n in the comment line
        self.pos = start_comment + 1;
        self.line += 1;
    }

    /// Function to add a token inside the list of tokens founds
    fn add_token<'a>(&'a mut self, token: tokens::YamlToken) {
        self.tokens.push(token);
    }

    /// Take the next token in the stream
    // TODO: can be improvend in performance?
    fn next(&mut self, stream: &str) -> char {
        self.pos += 1;
        stream.chars().nth(self.pos - 1).unwrap()
    }

    /// Check if we reach the end of the content
    /// by checking the lenght of the token with
    /// the actual position
    fn is_at_end(&self, content: &str) -> bool {
        content.len() <= self.pos
    }
}

/// YamlScanner implementation for the Scanner struct
impl YamlScanner<tokens::YamlToken> for Scanner {
    fn new() -> Self {
        Scanner {
            pos: 0,
            line: 0,
            tokens: vec![],
        }
    }

    fn scan(&mut self, content: &str) -> &Vec<tokens::YamlToken> {
        self.run(&content);
        &self.tokens
    }
}

#[cfg(test)]
mod test {
    use crate::scanner::scanner::{Scanner, YamlScanner};
    use indoc::indoc;

    #[test]
    fn scan_simple_one() {
        let mut scanner = Scanner::new();
        let simple_yaml = indoc! {"# This is a list of document
american:
 - Boston Red Sox
 - Detroit Tigers
 - New York Yankees
national:
 - New York Mets
 - Chicago Cubs
 - Atlanta Braves
"};
        let tokens = scanner.scan(&simple_yaml);
        assert_eq!(tokens.len(), 0);
    }
}
