//! parser crate to implement the core implementation
//! of the Yaml parser
//!
//! author: https://github.com/vincenzopalazzo
use std::error::Error;
use std::vec::Vec;

use super::tokens::{YamlObject, YamlScope};
use super::YamlParser;
use crate::parser::YamlParserError;
use crate::scanner::tokens::YamlToken;
use utils::tracer::{DummyTracer, Tracer};

#[derive(Debug)]
pub struct Parser {
    ir: Vec<YamlObject>,
    current_pos: u32,
    current_scope_size: Option<u32>,
    indentation_size: u16,
    tracer: Box<dyn Tracer>,
}

/// Core implementation of the Yaml Parser.
impl Parser {
    fn new() -> Self {
        Parser::new_with_tracer(Box::new(DummyTracer {}))
    }

    fn new_with_tracer(tracer: Box<dyn Tracer>) -> Self {
        Parser {
            ir: vec![],
            current_pos: 0,
            current_scope_size: None,
            indentation_size: 0,
            tracer: tracer,
        }
    }

    /// Walk through the list of tokens
    fn walk(&mut self, tokens: &Vec<YamlToken>) {
        while !self.is_the_end(tokens) {
            self.parse_document(tokens);
        }
    }

    /// Parse the YAML document
    fn parse_document(&mut self, tokens: &Vec<YamlToken>) {
        self.tracer.info(&"Start parsing Yaml".to_string());
        // Open the scope
        while !self.is_the_end(tokens) {
            let token = self.take(tokens);
            self.tracer
                .info(&format!("Parse document start with token: {:?}", token));
            // Now, there are two way to start a document in yaml
            // on with the start docs symbol, that is represented with `----`
            // or with a plain identifier.
            match token {
                YamlToken::StartDoc => {
                    // consume the token where we are matching!
                    self.consume(tokens);
                    match self.next(tokens) {
                        YamlToken::Identifier(name) => {
                            let yaml_identifier = self.parse_identifier(name.as_str(), tokens);
                            self.add_to_ir(yaml_identifier);
                        }
                        _ => {
                            self.tracer
                                .info(&format!("Bad token: {:?}", self.take(tokens)));
                            self.tracer.info(&format!("Parser Status: {:?}", self));
                            panic!("Document bad formatted: {:?}", self.take(tokens));
                        }
                    }
                }
                YamlToken::Identifier(name) => {
                    self.consume(tokens);
                    let yaml_identifier = self.parse_identifier(name.as_str(), tokens);
                    self.add_to_ir(yaml_identifier);
                }
                YamlToken::Pount(_) => self.parse_comment(tokens),
                YamlToken::EOF => self.consume(tokens),
                YamlToken::Space => {
                    // Fallback case, in this block we check if we are still in the current scope
                    // or we are in a new scope, and this mean that we need to close the prev scope.
                    self.consume_or_close_scope(tokens);
                }
                _ => panic!(
                    "Document bad formatted: {:?}\n\n Parse {:?} \n\nTokens {:?}",
                    self.take(tokens),
                    self,
                    tokens
                ),
            }
        }

        if !self.is_end_of_doc(tokens) && !self.is_the_end(tokens) {
            panic!(
                "The document it is no ended, we have the token in the stream {:?}",
                self.take(tokens)
            );
        }
    }

    fn parse_identifier(&mut self, ref_id: &str, tokens: &Vec<YamlToken>) -> YamlObject {
        assert_eq!(self.next(tokens), YamlToken::DotDot);
        // TODO: check the case where the literal is not a list but is a value
        if self.open_scope_if_needed(tokens) {
            self.consume_scope(tokens);
        } else {
            // in case of literal we need to consume the space, because
            // there is no scope
            self.consume_space_if_exit(tokens);
        }
        self.tracer.info(&format!(
            "In identifier: open scope, last token: {:?}",
            self.take(tokens)
        ));
        self.parse_mapping(&ref_id, tokens)
    }

    fn open_scope_if_needed(&mut self, tokens: &Vec<YamlToken>) -> bool {
        let current_scope = self.current_scope_size.unwrap_or(0) as u32;
        let space_in_front = self.count_spaces(tokens).unwrap_or(0) as u32;
        if current_scope != 0 && space_in_front < (current_scope / 2) {
            return false;
        }

        self.open_scope(tokens).unwrap();
        return true;
    }

    /// Check if the token in current position is a end of document.
    fn is_end_of_doc(&self, tokens: &Vec<YamlToken>) -> bool {
        tokens[self.current_pos as usize] == YamlToken::EndDoc
    }

    fn count_spaces(&mut self, tokens: &Vec<YamlToken>) -> Result<u8, YamlParserError> {
        let token = self.take(tokens);
        if token != YamlToken::Space {
            return Ok(0);
        }
        let mut scope_size: u8 = 0;
        let mut pos = self.current_pos as usize;
        while tokens.len() > pos && tokens[pos] == YamlToken::Space {
            scope_size = scope_size + 1;
            pos += 1;
        }
        Ok(scope_size)
    }

    /// Start a new scope from the token that can open a scope.
    ///
    /// N.B We check the indentation size from the first number of space
    /// that we found in the stream, so in this way we make sure that the
    /// document is well format.
    fn open_scope(&mut self, tokens: &Vec<YamlToken>) -> Result<YamlToken, YamlParserError> {
        let scope_size = self.count_spaces(tokens).unwrap_or(0) as u32;
        self.tracer.info(&format!(
            "Actual scope size: {}",
            self.current_scope_size.unwrap_or(0)
        ));
        self.tracer.info(&format!("new scope: {}", scope_size));
        match self.current_scope_size {
            Some(scope_dim) => {
                self.current_scope_size = Some(scope_size);
                Ok(self.take(tokens))
            }
            None => {
                // init indentation size
                self.indentation_size = scope_size as u16;
                self.current_scope_size = Some(scope_size);
                Ok(self.take(tokens))
            }
        }
    }

    fn close_scope(&mut self, tokens: &Vec<YamlToken>) {
        let spaces = self.count_spaces(tokens).unwrap() as u32;
        if spaces < self.current_scope_size.unwrap() {
            // TODO: we should divided the space / 2 and not trust the tokens
            self.current_scope_size = Some(spaces);
        } else {
            self.tracer
                .info(&"You should not close the scope here".to_string());
        }
    }

    fn consume_or_close_scope(&mut self, tokens: &Vec<YamlToken>) {
        // Fallback case, in this block we check if we are still in the current scope
        if self.current_scope(tokens) {
            self.consume_scope(tokens);
        } else {
            self.close_scope(tokens);
        }
    }

    fn consume_scope(&mut self, tokens: &Vec<YamlToken>) {
        self.tracer.info(&"consume_scope: start".to_string());
        match self.current_scope_size {
            Some(size) => {
                for _ in 0..size {
                    self.tracer.debug(&format!("Parse status: {:?}", self));
                    assert_eq!(YamlToken::Space, self.next(tokens));
                    self.tracer.info(&format!(
                        "Next token after space consumed: {:?}",
                        self.take(tokens)
                    ));
                }
            }
            None => panic!("No scope found"),
        }
        self.tracer.info(&"consume_scope: end".to_string());
    }

    fn consume_space_if_exit(&mut self, tokens: &Vec<YamlToken>) {
        if self.take(tokens) == YamlToken::Space {
            self.consume(tokens);
        }
    }

    fn current_scope(&mut self, tokens: &Vec<YamlToken>) -> bool {
        match self.current_scope_size {
            Some(spaces) => {
                if let Ok(current_size) = self.count_spaces(tokens) {
                    current_size as u32 == spaces
                } else {
                    false
                }
            }
            None => true,
        }
    }

    /// wrapping the way of the compiler to build the error
    fn return_error(&self, cause: &str) -> YamlParserError {
        YamlParserError {
            cause: cause.to_string(),
        }
    }

    /// Parse the document comment and store in a new comment node
    fn parse_comment(&mut self, tokens: &Vec<YamlToken>) {
        match self.take(tokens) {
            YamlToken::Pount(content) => {
                let comment = YamlObject::Comment(content.to_string());
                self.add_to_ir(comment);
                self.consume(tokens);
            }
            _ => self.skip(),
        }
    }

    /// parse Yaml mapping key: value or other type of mapping like:
    /// ----- Scalar to Sequence ----
    /// american:
    // - Boston Red Sox
    // - Detroit Tigers
    // - New York Yankees
    /// --------- Sequence of mapping ---------
    /// -
    ///   name: Mark McGwire
    ///   hr:   65
    ///   avg:  0.278
    /// --------- Sequence of Sequence ---------
    /// - [name        , hr, avg  ]
    /// --------- Mapping of Mappings ------
    /// Mark McGwire: {hr: 65, avg: 0.278}
    fn parse_mapping(&mut self, tag: &str, tokens: &Vec<YamlToken>) -> YamlObject {
        let token = self.take(tokens);
        self.tracer
            .info(&format!("Parse mapping: start with token {:?}", token));
        self.tracer.info(&format!("Parser State {:?}", self));
        let item = match token {
            YamlToken::Dash => self.parse_scalar_to_sequence(tokens),
            YamlToken::LeftSquareBrace => self.parse_sequence_of_sequence(tokens),
            YamlToken::LeftCurlyBrace => self.parse_mapping_of_mapping(tokens),
            YamlToken::Identifier(ref sub_tag) => {
                self.consume(tokens);
                let yaml_identifier = self.parse_identifier(sub_tag.as_str(), tokens);
                YamlObject::Mapping(sub_tag.to_string(), Box::new(yaml_identifier), false)
            }
            // Literal values
            YamlToken::IntVal(_) | YamlToken::StringVal(_) | YamlToken::FloatVal(_) => {
                let yaml_val = self.parse_literal_val(tokens);
                YamlObject::Mapping(tag.to_string(), Box::new(yaml_val), false)
            }
            _ => panic!(
                "Invalid mapping format, encounter token {:?} and parse state: {:?}",
                token, self
            ),
        };
        YamlObject::Mapping(tag.to_string(), Box::new(item), false)
    }

    fn parse_literal_val(&mut self, tokens: &Vec<YamlToken>) -> YamlObject {
        let literal = match self.take(tokens) {
            YamlToken::IntVal(val) => YamlObject::Int(val),
            YamlToken::FloatVal(val) => YamlObject::Float(val),
            YamlToken::StringVal(val) => YamlObject::Str(val),
            YamlToken::NullVal => YamlObject::Null,
            _ => panic!("Not a literal value {:?}. \n{:?}", self.take(tokens), self),
        };
        self.consume(tokens);
        literal
    }

    /// parse Yaml mapping key: value or other type of mapping like:
    /// ----- Scalar to Sequence ----
    /// american:
    /// - Boston Red Sox
    /// - Detroit Tigers
    /// - New York Yankees
    ///
    /// In this case there are situation where we need to parse the indentation
    fn parse_scalar_to_sequence(&mut self, tokens: &Vec<YamlToken>) -> YamlObject {
        assert_eq!(self.take(tokens), YamlToken::Dash);
        self.tracer.info(&format!(
            "parse scalar to sequence: Start with token {:?}",
            self.take(tokens)
        ));
        let mut yaml_seq = Vec::<YamlObject>::new();
        while self.take(tokens) == YamlToken::Dash {
            self.consume(tokens); //consume dash
                                  // TODO: this need to be a must, we need to change it
            self.consume_space_if_exit(tokens);
            match self.take(tokens) {
                YamlToken::IntVal(_) | YamlToken::StringVal(_) | YamlToken::FloatVal(_) => {
                    let yaml_literal = self.parse_literal_val(tokens);
                    yaml_seq.push(yaml_literal);
                    self.tracer.info(&format!(
                        "parse scalar to sequence: token after literlar: {:?}",
                        self.take(tokens)
                    ));
                    self.consume_or_close_scope(tokens);
                }
                YamlToken::Identifier(name) => {
                    self.consume(tokens);
                    let yaml_identifier = self.parse_identifier(name.as_str(), tokens);
                    yaml_seq.push(yaml_identifier);
                }
                _ => panic!("Unsupported: {:?}", self.take(tokens)),
            }
        }
        YamlObject::List(yaml_seq)
    }

    fn parse_sequence_of_sequence(&mut self, tokens: &Vec<YamlToken>) -> YamlObject {
        YamlObject::Fake
    }

    fn parse_mapping_of_mapping(&mut self, tokens: &Vec<YamlToken>) -> YamlObject {
        YamlObject::Fake
    }

    /// Add the yaml node to the list of Yaml node
    /// collected from the parser
    // TODO: find the error here generated by the & of the yaml_node
    // it is related to the copy but I don't know much
    fn add_to_ir(&mut self, yaml_node: YamlObject) {
        self.ir.push(yaml_node);
    }

    /// Check for scope definition in the tokens stream
    /// and return a flag with the scope that the parser found
    fn parse_scope(&self, tokens: &Vec<YamlToken>) -> YamlScope {
        YamlScope::InvalidScope("TODO: implement this part".to_string())
    }

    /// Take the next element in the stream, and increase the current position
    /// of it.
    fn next<'a>(&'a mut self, tokens: &'a Vec<YamlToken>) -> YamlToken {
        self.current_pos += 1;
        tokens[(self.current_pos - 1) as usize].clone()
    }

    /// Take the element at the current position of the stream.
    fn take(&self, tokens: &Vec<YamlToken>) -> YamlToken {
        tokens[self.current_pos as usize].clone()
    }

    fn is_the_end(&self, tokens: &Vec<YamlToken>) -> bool {
        tokens[self.current_pos as usize] == YamlToken::EOF
    }

    fn is_the_biginning(&self) -> bool {
        self.current_pos == 0
    }

    /// do nothings, just make the code more readble.
    fn skip(&self) {}

    /// consume the token in current position without store
    /// the value.
    /// it is the same of next, but it just make the code more readble
    fn consume(&mut self, tokens: &Vec<YamlToken>) {
        self.next(tokens);
    }
}

/// Common implementation of the Yaml Parser
impl YamlParser<YamlObject> for Parser {
    fn parse(&mut self, tokens: &Vec<YamlToken>) -> &Vec<YamlObject> {
        self.walk(tokens);
        &self.ir
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parser::{Parser, YamlParser};
    use crate::scanner::{scanner::Scanner, YamlScanner};
    use utils::tracer::{DefLogTracer, Tracer};

    use indoc::indoc;
    #[test]
    fn parse_simple_one() {
        env_logger::init();
        let mut scanner = Scanner::new();
        let mut parser = Parser::new_with_tracer(Box::new(DefLogTracer::new()));
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

        let ir = parser.parse(&tokens);
        assert!(ir.len() > 0);
    }

    #[test]
    fn parse_simple_two() {
        let mut scanner = Scanner::new();
        let mut parser = Parser::new_with_tracer(Box::new(DefLogTracer::new()));
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
        let ir = parser.parse(&tokens);
        assert!(ir.len() > 0);
    }
}
