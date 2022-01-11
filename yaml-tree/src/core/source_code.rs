/// TODO: document the code
use super::IRNode;

// Intermediary rappresentation
pub struct SourceCodeIRNode {
    pub name: String,
    pub doc_str: Option<String>,
}

impl IRNode for SourceCodeIRNode {
    fn verify() -> bool {
        true
    }
}
