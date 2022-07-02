/// Token implementation for parser
/// in particular this class represent Object
/// definition that we can find in a YAML definition
///
/// author: https://github.com/vincenzopalazzo

/// Enum for YAML document definition
#[derive(Debug)]
pub enum YamlObject {
    /// Document Object that contains the all the yaml definition
    Document(Vec<YamlObject>),
    /// Reference to another Yaml document
    // TODO: see how the reference works
    Reference(Box<YamlObject>),
    /// Mapping value, that can be between { and }.
    /// if bool is specified, the {} are specified.
    Mapping(String, Box<YamlObject>, bool),
    /// The key - value object that can represent a single yaml value
    KeyVal(String, Box<YamlObject>),
    /// List Object that contains all list yaml element definition
    List(Vec<YamlObject>),
    Str(String),
    Int(i64),
    Float(f64),
    /// Yaml Comment that store the content of the comment.
    Comment(String),
    Fake,
}

/// Enum definition to mark the end of new scope
/// or the start of one scope.
pub enum YamlScope {
    /// Open a new scope, contains only the position
    /// where this scope is started
    OpenScope(u32),
    /// Close a new scope
    /// contains the start of the scope and the
    /// end of new one
    CloseScope(u32, u32),
    /// Invalid scope, it contains also an error
    /// message
    InvalidScope(String),
}
