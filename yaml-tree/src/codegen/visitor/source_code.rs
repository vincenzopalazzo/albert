/// TODO: adding the doc on this module
use super::{VisitErr, YamlVisitor};
use crate::core::source_code::SourceCodeIRNode;

/// module that implementing a specific visit
/// on a YAML tree.
pub struct VisitSrc {
    //TODO: declare the result of the visitor
}

impl YamlVisitor<SourceCodeIRNode> for VisitSrc {
    fn visit_array() -> Result<(), VisitErr> {
        Ok(())
    }

    fn result() -> SourceCodeIRNode {
        SourceCodeIRNode {
            name: String::from("TODO"),
            doc_str: None,
        }
    }
}
