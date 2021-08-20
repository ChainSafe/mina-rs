// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;

use serde::Deserialize;
use serde_json::from_value;

mod list_tagged_enum;
mod traverse;

pub use traverse::{BinProtRuleIterator, BranchingIterator};

use list_tagged_enum::ListTaggedEnum;

/// The main top level type for a layout file.
/// Parse into this from json
#[derive(Deserialize, Debug)]
pub struct Layout {
    pub layout_loc: String,
    pub version_opt: Option<i32>,
    pub type_decl: String,
    pub bin_io_derived: bool,
    pub bin_prot_rule: BinProtRule,
}

/// Recursively defined BinProtRule is how the type tree is constructed
#[derive(Clone, Deserialize, Debug)]
#[serde(try_from = "ListTaggedEnum")]
pub enum BinProtRule {
    Nat0,
    Unit,
    Bool,
    String,
    Char,
    Int,
    Int32,
    Int64,
    NativeInt,
    Float,
    Option(Box<BinProtRule>),
    Record(Vec<RecordField>), // records/structs
    Tuple(Vec<BinProtRule>),
    Sum(Vec<Summand>), // sum types/enums
    Polyvar(Vec<Polyvar>),
    List(Box<BinProtRule>),
    Hashtable(HashTblEntry),
    Vec(usize, Box<BinProtRule>),
    Bigstring,
    // //  track indirections for debugging *),
    Reference(RuleRef),
    TypeVar(String),
    // //  inside a recursive type, list of type parameters *),
    SelfReference(Vec<BinProtRule>),
    // //  parameterized type: 'a t = ... *),
    TypeAbstraction(Vec<String>, Box<BinProtRule>),
    // //  recursive parameterized type with bindings *),
    TypeClosure(Vec<(String, BinProtRule)>, Box<BinProtRule>),
    Custom(Vec<BinProtRule>),
    CustomForPath(String, Vec<BinProtRule>), // does not occur in source files, used in traverse
}

// required due to the strange enum encoding used by yojson (see list_tagged_enum.rs)
impl TryFrom<ListTaggedEnum> for BinProtRule {
    type Error = String;
    fn try_from(v: ListTaggedEnum) -> Result<Self, Self::Error> {
        match v {
            ListTaggedEnum::None((t,)) => match t.as_str() {
                "Nat0" => Ok(BinProtRule::Nat0),
                "Unit" => Ok(BinProtRule::Unit),
                "Bool" => Ok(BinProtRule::Bool),
                "String" => Ok(BinProtRule::String),
                "Char" => Ok(BinProtRule::Char),
                "Int" => Ok(BinProtRule::Int),
                "Int32" => Ok(BinProtRule::Int32),
                "Int64" => Ok(BinProtRule::Int64),
                "Native_int" => Ok(BinProtRule::NativeInt),
                "Float" => Ok(BinProtRule::Float),
                "Bigstring" => Ok(BinProtRule::Bigstring),
                _ => Err(format!("Unexpected enum tag: {}", t)),
            },
            ListTaggedEnum::One((t, v)) => match t.as_str() {
                "Option" => Ok(BinProtRule::Option(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Record" => Ok(BinProtRule::Record(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Tuple" => Ok(BinProtRule::Tuple(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Sum" => Ok(BinProtRule::Sum(from_value(v).map_err(|e| e.to_string())?)),
                "Polyvar" => Ok(BinProtRule::Polyvar(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "List" => Ok(BinProtRule::List(from_value(v).map_err(|e| e.to_string())?)),
                "Hashtable" => Ok(BinProtRule::Hashtable(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Reference" => Ok(BinProtRule::Reference(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Type_var" => Ok(BinProtRule::TypeVar(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Self_reference" => Ok(BinProtRule::SelfReference(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Custom" => Ok(BinProtRule::Custom(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                _ => Err(format!("Unexpected enum tag: {}", t)),
            },
            ListTaggedEnum::Two((t, v1, v2)) => match t.as_str() {
                "Vec" => Ok(BinProtRule::Vec(
                    from_value(v1).map_err(|e| e.to_string())?,
                    from_value(v2).map_err(|e| e.to_string())?,
                )),
                "Type_abstraction" => Ok(BinProtRule::TypeAbstraction(
                    from_value(v1).map_err(|e| e.to_string())?,
                    from_value(v2).map_err(|e| e.to_string())?,
                )),
                "Type_closure" => Ok(BinProtRule::TypeClosure(
                    from_value(v1).map_err(|e| e.to_string())?,
                    from_value(v2).map_err(|e| e.to_string())?,
                )),
                _ => Err(format!("Unexpected enum tag: {}", t)),
            },
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct RecordField {
    pub field_name: String,
    pub field_rule: BinProtRule,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Summand {
    pub ctor_name: String,
    pub index: i32,
    pub ctor_args: Vec<BinProtRule>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct HashTblEntry {
    key_rule: Box<BinProtRule>,
    value_rule: Box<BinProtRule>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(try_from = "ListTaggedEnum")]
pub enum Polyvar {
    Tagged(TaggedPolyvar),
    Inherited(BinProtRule),
}

#[derive(Clone, Deserialize, Debug)]
pub struct TaggedPolyvar {
    polyvar_name: String,
    hash: i32,
    polyvar_args: Vec<BinProtRule>,
}

impl TryFrom<ListTaggedEnum> for Polyvar {
    type Error = String;
    fn try_from(v: ListTaggedEnum) -> Result<Self, Self::Error> {
        match v {
            ListTaggedEnum::One((t, v)) => match t.as_str() {
                "Tagged" => Ok(Polyvar::Tagged(from_value(v).map_err(|e| e.to_string())?)),
                "Inherited" => Ok(Polyvar::Inherited(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                _ => Err(format!("Unexpected enum tag: {}", t)),
            },
            _ => Err("Unexpected number of items in enum body".to_string()),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(try_from = "ListTaggedEnum")]
pub enum RuleRef {
    Unresolved(UnresolvedPayload),
    Resolved(ResolvedPayload),
}

#[derive(Clone, Deserialize, Debug)]
pub struct UnresolvedPayload {
    params: Vec<BinProtRule>,
    layout_id: String, // what is longident?
}

#[derive(Clone, Deserialize, Debug)]
pub struct ResolvedPayload {
    source_type_decl: String,
    #[serde(default)]
    source_module_path: String,
    #[serde(default)]
    bin_io_derived: bool, // This is present only in later versions of the type layouts. Probably don't trust its value
    ref_rule: Box<BinProtRule>,
}

impl TryFrom<ListTaggedEnum> for RuleRef {
    type Error = String;
    fn try_from(v: ListTaggedEnum) -> Result<Self, Self::Error> {
        match v {
            ListTaggedEnum::One((t, v)) => match t.as_str() {
                "Unresolved" => Ok(RuleRef::Unresolved(
                    from_value(v).map_err(|e| e.to_string())?,
                )),
                "Resolved" => Ok(RuleRef::Resolved(from_value(v).map_err(|e| e.to_string())?)),
                _ => Err("Unexpected tag for this type".to_string()),
            },
            _ => Err("Unexpected number of items in enum body".to_string()),
        }
    }
}
