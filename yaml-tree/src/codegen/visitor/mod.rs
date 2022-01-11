/// Visitor that implement a visit over YAML node
use thiserror::Error;

pub mod source_code;

/// Visitor Trait that provide the interface
/// to visit a YAML node
pub trait YamlVisitor<T> {
    fn visit_array() -> Result<(), VisitErr>;
    /// Return the value inflate by the visitor
    fn result() -> T;
}

#[derive(Error, Debug)]
pub enum VisitErr {
    #[error("generic error happens during the visti")]
    Any(String),
    #[error("unkonwn the error that happens")]
    Unknown,
}
