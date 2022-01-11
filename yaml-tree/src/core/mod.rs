/// TODO: adding doc module  here
pub mod source_code;

/// Intermedia Rapresentation node
pub trait IRNode {
    /// Verify if the doc contains
    /// all the necessary information
    fn verify() -> bool;
}
