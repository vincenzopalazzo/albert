/// Token implementation for parser
/// in particular this class represent Object
/// definition that we can find in a YAML definition
///
/// author: https://github.com/vincenzopalazzo

/// Enum for YAML document definition
pub enum YamlObject {
    /// Document Object that contains the all the yaml definition
    Document(Vec<YamlObject>),
    /// List Object that contains all list yaml element definition
    List(Vec<YamlObject>),
    /// Mapping value, that can be between { and }.
    /// if bool is specified, the {} are specified.
    Mapping(Vec<YamlObject>, bool),
    /// The key - value object that can represent a single yaml value
    KeyVal(String, Box<YamlObject>),
    /// Yaml Comment that store the content of the comment.
    Comment(String),
    /// Reference to another Yaml document
    // TODO: see how the reference works
    Reference(Box<YamlObject>),
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
