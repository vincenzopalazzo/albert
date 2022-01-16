//! Yaml Tokens definition
//!
//! author: https://github.com/vincenzopalazzo

/// Enumerator to define the Yaml language
pub enum YamlToken {
    /// Yaml Key, element to the left of :
    Key(String),
    /// Yaml Value definition, element to the right of :
    StringVal(String),
    NullVal,
    IntVal(i64),
    FloatVal(f64),
    BoolVal(bool),
    TimestampVal(String),
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
    /// > Multiline symbols
    GraterThan,
    /// | Pipe symbols
    Pipe,
    /// ', Single Quotes to manage a string
    SingleQuote,
    /// ", Double Quotes to manage a string
    DoubleQuote,
    /// EOF put the end of the tokens
    EOF,
}
