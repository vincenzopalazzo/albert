//! Yaml Tokens definition
//!
//! author: https://github.com/vincenzopalazzo

/// Enumerator to define the Yaml language
#[derive(Debug, PartialEq, Clone)]
pub enum YamlToken {
    Identifier(String),
    NullVal,
    IntVal(i64),
    FloatVal(f64),
    StringVal(String),
    BoolVal(bool),
    /// ---, Star document token
    StartDoc,
    /// ..., End document token
    EndDoc,
    /// Space token
    // TODO: we need to put this there?
    Space,
    /// Tab space token
    // TODO: we need to put this there?
    TabSpace,
    /// :, Two Point to divide a key value
    DotDot,
    /// #, Comment line token
    Pount(String),
    /// -, Dash token to listing items
    Dash,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftSquareBrace,
    RightSquareBrace,
    /// Multiline Token include the | and > token
    Multiline(String),
    /// EOF put the end of the tokens
    EOF,
}
