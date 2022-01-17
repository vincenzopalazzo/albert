//! Source Core Intermediate Rapresentation (IR)
//!
//! The following IR give us the possibility to move
//! from a data model to a source code.
//!
//! author: https://github.com/vincenzopalazzo
use std::vec::Vec;

use super::src_visit::OutVisitor;

/// Source code node that contains all the information
/// useful to encode a source code metadata
pub trait SrcNode {
    /// Give the possibility to the node to
    /// to be traversed from a OutVisitor
    fn accept(&self, visitor: &mut dyn OutVisitor);
}

/// Visibility enum to add metadata about the visibility
pub enum VisibilityMeta {
    Public,
    Private,
    /// Some language as different visibility like
    /// protected in Java.
    Custom(String),
}

/// Metadata Node, a struct that contains all the metadata
/// useful for the code generator, like comment over a node
/// public/private, ecc.
pub struct MetaNode {
    /// Comment metadata
    comment: Option<String>,
    /// Visibility of the node in the target language
    visibility: Option<VisibilityMeta>,
    /// Mutability of the node in the target language
    mutability: Option<bool>,
    nullable: Option<bool>,
    pointer: Option<bool>,
}

/// Concrete implementation of a obj/struct data model
pub struct ObjMetaNode<'a> {
    metadata: &'a MetaNode,
}

/// Custom interface of the ObjMetaNode
impl<'a> ObjMetaNode<'a> {
    fn new(metadata: &'a MetaNode) -> Self {
        ObjMetaNode { metadata }
    }
}

/// Common interface SrcNode implemented over the ObjMetaNode
impl SrcNode for ObjMetaNode<'_> {
    fn accept(&self, visitor: &mut dyn OutVisitor) {
        visitor.visit_obj(self);
    }
}

/// Concrete implementation of a string data model
pub enum CharNode {
    Char(char),
    Str(String),
}

pub struct CharMetaNode<'a> {
    metadata: &'a MetaNode,
    tipe: &'a CharNode,
}

impl<'a> CharMetaNode<'a> {
    fn new(metadata: &'a MetaNode, tipe: &'a CharNode) -> Self {
        CharMetaNode {
            metadata: metadata,
            tipe: tipe,
        }
    }
}

impl SrcNode for CharMetaNode<'_> {
    fn accept(&self, visitor: &mut dyn OutVisitor) {
        visitor.visit_char(self);
    }
}

pub enum NumberNode {
    Int64Node(i64),
    Float64Node(f64),
}

pub struct NumberMetaNode<'a> {
    metadata: &'a MetaNode,
    tipe: &'a NumberNode,
}

impl<'a> NumberMetaNode<'a> {
    fn new(metadata: &'a MetaNode, tipe: &'a NumberNode) -> Self {
        NumberMetaNode { metadata, tipe }
    }
}

impl SrcNode for NumberMetaNode<'_> {
    fn accept(&self, visitor: &mut dyn OutVisitor) {
        visitor.visit_number(self);
    }
}

/// ListType defintion
pub struct ListType {
    type_name: Option<String>,
    complex_type: Box<Option<BagNode>>,
}

/// map type definition
pub struct MapType {
    key_type: String,
    value_type: Option<String>,
    //TODO: Why the compiler suggest to  use Box?
    complex_value: Box<Option<BagNode>>,
}

/// Collection enum type
pub enum BagNode {
    ListNode(ListType),
    MapNode(MapType),
    Enum(Vec<String>),
}

pub struct BagMetaNode<'a> {
    metadata: &'a MetaNode,
    tipe: &'a BagNode,
}

impl<'a> BagMetaNode<'a> {
    fn new(metadata: &'a MetaNode, tipe: &'a BagNode) -> Self {
        BagMetaNode { metadata, tipe }
    }
}

impl SrcNode for BagMetaNode<'_> {
    fn accept(&self, visitor: &mut dyn OutVisitor) {
        visitor.visit_collection(self);
    }
}
