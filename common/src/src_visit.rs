//! Source code Intermediate Rapresentation (IR) visitor.
//!
//! Give the possibility to a SrcNode
//!
//! author: https://github.com/vincenzopalazzo
use super::src_ir::{BagMetaNode, CharMetaNode, NumberMetaNode, ObjMetaNode};

/// Out Visitor traits that contains all the
/// method to implement a new type of visit
pub trait OutVisitor {
    /// Core implementation logic to visit a Object
    fn visit_obj(&mut self, obj: &ObjMetaNode);

    /// Core implementation logic to visit a char type
    /// included strings.
    fn visit_char(&mut self, string: &CharMetaNode);

    /// Core implementation logic to visit a number
    fn visit_number(&mut self, int: &NumberMetaNode);

    /// Core implementation logic to visit a collection
    fn visit_collection(&mut self, bag: &BagMetaNode);
}
