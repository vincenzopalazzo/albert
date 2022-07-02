/// Core implementation of the YAML scanner
///
/// author: https://github.com/vincenzopalazzo
use super::tokens::*;
use super::YamlScanner;

/// Core implementation of the scanner
pub struct Scanner {
    pos: usize,
    line: u64,
    tokens: Vec<YamlToken>,
    identifier_symbols: Vec<char>,
}

impl Scanner {
    /// Entry point of the scanner to parser
    /// the string and return a list of token
    fn run(&mut self, content: &str) {
        while !self.is_at_end(&content) {
            self.contrue_token(&content);
        }
        self.add_token(YamlToken::EOF);
    }

    /// Core function to try to understand the
    /// core function of the scanner too contrue
    /// the token from the content
    fn contrue_token(&mut self, content: &str) {
        let char_token = self.next(&content);
        match char_token {
            '\n' => self.line += 1,
            ' ' => self.add_token(YamlToken::Space),
            ':' => self.add_token(YamlToken::DotDot),
            '|' | '>' => {
                let multiline = "TODO: need to parse multiline content";
                self.add_token(YamlToken::Multiline(multiline.to_string()));
                todo!("{}", multiline);
            }
            '[' => self.add_token(YamlToken::LeftSquareBrace),
            ']' => self.add_token(YamlToken::RightCurlyBrace),
            '{' => self.add_token(YamlToken::LeftCurlyBrace),
            '}' => self.add_token(YamlToken::RightCurlyBrace),
            '\'' | '"' => self.parse_string(content),
            '-' => {
                // Check if the first dash is the item list
                // or it is a token to start a new document
                if self.advance_if_match(content, '-') {
                    // we need another - otherwise we emit an error
                    if self.advance_if_match(content, '-') {
                        self.add_token(YamlToken::StartDoc);
                    } else {
                        // TODO emit a scanner error here
                        panic!("Error we have a from start document token");
                    }
                } else {
                    self.add_token(YamlToken::Dash);
                }
            }
            '.' => {
                if self.advance_if_match(content, '.') && self.advance_if_match(content, '.') {
                    self.add_token(YamlToken::EndDoc);
                } else {
                    // TODO emit scanner errors
                    panic!("End document token malformed");
                }
            }
            '#' => self.parse_comment_line(&content),
            _ => {
                if !self.parse_number(char_token, content)
                    && !self.parse_identifier(char_token, content)
                {
                    panic!(
                        "Content not recognized at line {}: {}",
                        self.line, char_token
                    );
                }
                // Try to decode the number
                // and the string
            }
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
        let token = YamlToken::Pount(line_comment);
        self.add_token(token);
        // with increase the value by one because we want
        // skip the \n in the comment line
        self.pos = start_comment + 1;
        self.line += 1;
    }

    fn is_valid_for_str(&self, char_at: char) -> bool {
        char_at != '\'' && char_at != '"'
    }

    /// Parsing a string contained inside a ' or ".
    fn parse_string(&mut self, stream: &str) {
        if self.is_at_end(stream) {
            return;
        }
        let start = self.pos;
        let mut start_char = self.peek(stream);
        while self.is_valid_for_str(start_char) && !self.is_at_end(stream) {
            if start_char == '\n' {
                self.line += 1;
                break;
            }
            start_char = self.next(stream);
        }
        let identifier = &stream[start..self.pos];
        self.add_token(YamlToken::StringVal(identifier.to_string()));
    }

    /// Decode a in the correct Yaml Token a correct number, in the correct
    /// form
    fn parse_number<'a>(&'a mut self, char_at: char, stream: &'a str) -> bool {
        if char_at.is_numeric() {
            let mut is_decimal = false;
            let start = self.pos - 1;
            let mut start_dig = self.next(stream);
            while start_dig.is_numeric() {
                start_dig = self.next(stream);
            }

            if start_dig == '.' {
                is_decimal = true;
                start_dig = self.next(stream);
                while start_dig.is_numeric() {
                    start_dig = self.next(stream);
                }
            }
            // restore the token that the while has consumed
            self.pos -= 1;
            let number = &stream[start..self.pos];
            if !is_decimal {
                self.add_token(YamlToken::IntVal(number.parse().unwrap()));
            } else {
                self.add_token(YamlToken::FloatVal(number.parse().unwrap()));
            }
            return true;
        }
        false
    }

    /// Check if a char contains a valid token for the an identifier
    fn valid_identifier_symbol(&self, char_at: char) -> bool {
        char_at.is_alphanumeric() || self.identifier_symbols.contains(&char_at)
    }

    /// Decode in a YamlToken an valid identifier
    fn parse_identifier<'a>(&'a mut self, char_at: char, stream: &'a str) -> bool {
        if self.is_at_end(stream) {
            return true;
        }
        if self.valid_identifier_symbol(char_at) {
            let start = self.pos - 1;
            let mut start_char = self.peek(stream);
            while self.valid_identifier_symbol(start_char) && !self.is_at_end(stream) {
                start_char = self.next(stream);
            }
            // restore the token that the while has consumed
            self.pos -= 1;
            let identifier = &stream[start..self.pos];
            self.add_token(YamlToken::Identifier(identifier.to_string()));
            return true;
        }
        false
    }

    /// Function to add a token inside the list of tokens founds
    // TODO: find the error generated by the token passed by reference
    // the error refers to the copy, but I don't know much
    fn add_token<'a>(&'a mut self, token: YamlToken) {
        self.tokens.push(token);
    }

    /// Peek the value in the current position
    fn peek(&self, stream: &str) -> char {
        stream.chars().nth(self.pos).unwrap()
    }

    /// Take the next token in the stream
    // TODO: can be improvend in performance?
    fn next(&mut self, stream: &str) -> char {
        self.pos += 1;
        stream.chars().nth(self.pos - 1).unwrap()
    }

    /// Advance in the stream if exist a token that match the
    /// target element given in input.
    /// Return the result of the operation as boolean value
    fn advance_if_match(&mut self, stream: &str, target: char) -> bool {
        if self.is_at_end(stream) {
            return false;
        }
        let elem = stream.chars().nth(self.pos).unwrap();
        if elem == target {
            self.pos += 1;
            return true;
        }
        false
    }

    /// Check if we reach the end of the content
    /// by checking the lenght of the token with
    /// the actual position
    fn is_at_end(&self, content: &str) -> bool {
        content.len() <= self.pos
    }
}

/// YamlScanner implementation for the Scanner struct
impl YamlScanner<YamlToken> for Scanner {
    fn new() -> Self {
        Scanner {
            pos: 0,
            line: 0,
            tokens: vec![],
            identifier_symbols: vec!['_'],
        }
    }

    fn scan(&mut self, content: &str) -> &Vec<YamlToken> {
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
---
american:
 - \"Boston Red Sox\"
 - \"Detroit Tigers\"
 - \"New York Yankees\"
national:
 - \"New York Mets\"
 - \"Chicago Cubs\"
 - \"Atlanta Braves\"
"};
        let tokens = scanner.scan(&simple_yaml);
        println!("{:?}", tokens);
        assert!(tokens.len() > 0);
    }

    #[test]
    fn scan_simple_two() {
        let mut scanner = Scanner::new();
        let simple_yaml = indoc! {"# This is a list of document
ModelOne:
  - PropOne:
      - doc_comment: \"Documentation document\"
      - type: \"int64\"
      - visibility: \"public\"
      - mutable: true
      - nullable: true
      - reference: false
  - PropTwo:
      - doc_comment: \"This is a prop that contains a custom type\"
      - type: \"ModelOne\"
      - visibility: \"public\"
      - mutable: true
      - nullable: true
      - reference: true"};

        let tokens = scanner.scan(&simple_yaml);
        println!("{:?}", tokens)
    }
}
