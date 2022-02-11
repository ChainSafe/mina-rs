// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Defines a BinProt layout
//! A layout is a data structure that defines a BinProt type.
//! This is essentially moving the type information from compile time to runtime.

use core::convert::TryFrom;

use serde::{Deserialize, Serialize};
use serde_json::from_value;

mod list_tagged_enum;
pub(crate) mod traverse;

pub(crate) use traverse::BinProtRuleIterator;

use list_tagged_enum::ListTaggedEnum;

/// The main top level type for a layout file.
/// Parse into this from json
#[derive(Serialize, Deserialize, Debug)]
pub struct Layout {
    layout_loc: String,
    version_opt: Option<i32>,
    type_decl: String,
    bin_io_derived: bool,
    /// Rule defining this layout
    pub bin_prot_rule: BinProtRule,
}

/// Recursively defined BinProtRule is how the type tree is constructed
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(try_from = "ListTaggedEnum")]
pub enum BinProtRule {
    /// Variable length natural Integer
    Nat0,
    /// Unit type
    Unit,
    /// Boolean
    Bool,
    /// OCaml string type (different to a rust string)
    String,
    /// OCaml char type
    Char,
    /// Variable length integer
    Int,
    /// 32 bit Integer
    Int32,
    /// 64 bit Integer
    Int64,
    /// OS Native integer
    NativeInt,
    /// Floating point number
    Float,
    /// OCaml option
    Option(Box<BinProtRule>),
    /// records/structs
    Record(Vec<RecordField>),
    /// tuple
    Tuple(Vec<BinProtRule>),
    /// sum types/enums
    Sum(Vec<Summand>),
    /// τ ≤ Γ(a), τ is an instance of Γ(a) and (Γ(a) a type scheme
    Polyvar(Vec<Polyvar>),
    /// Variable length list of any BinProt type
    List(Box<BinProtRule>),
    /// Hash table of BinProt types
    Hashtable(HashTblEntry),
    /// fixed length of BinProt types
    Vec(usize, Box<BinProtRule>),
    /// A set of utils for dealing with `bigarrays` of `char`
    Bigstring,
    ///  track indirections for debugging
    Reference(RuleRef),
    /// your type
    TypeVar(String),
    ///  inside a recursive type, list of type parameters
    SelfReference(Vec<BinProtRule>),
    ///  parameterized type: 'a t = ...
    TypeAbstraction(Vec<String>, Box<BinProtRule>),
    ///  recursive parameterized type with bindings
    TypeClosure(Vec<(String, BinProtRule)>, Box<BinProtRule>),
    /// Type that does not use standard derived BinProt encoding
    Custom(Vec<BinProtRule>),
    /// does not occur in source files, used in traverse
    CustomForPath(String, Vec<BinProtRule>),
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

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Field of a BinProt record with a name and a value
pub struct RecordField {
    pub(crate) field_name: String,
    field_rule: BinProtRule,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Variant of a BinProt sum type (enum)
/// has a name, index and zero-or-more BinProt values
pub struct Summand {
    pub(crate) ctor_name: String,
    pub(crate) index: i32,
    pub(crate) ctor_args: Vec<BinProtRule>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Entry in a BinProt hash table
/// hash types for the key and value
pub struct HashTblEntry {
    key_rule: Box<BinProtRule>,
    value_rule: Box<BinProtRule>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(try_from = "ListTaggedEnum")]
/// τ ≤ Γ(a), τ is an instance of Γ(a) and (Γ(a) a type scheme
pub enum Polyvar {
    /// An instance of Γ(a)
    Tagged(TaggedPolyvar),
    /// An instance of Γ(a)
    Inherited(BinProtRule),
}

impl TaggedPolyvar {
    /// Convert a tagged polyvar to a summand since Rust has no concept of polvar types
    /// they can be represented as an enum instead for now
    /// Note: This is not perfect since it means round-trips are not possible!
    pub fn to_summand(self, index: usize) -> Summand {
        Summand {
            ctor_name: self.polyvar_name,
            index: index as i32,
            ctor_args: self.polyvar_args,
        }
    }
}

/// 4 bytes hash used to identify a variant of a polyvar
/// These are used instead of an index as in sum types
pub type PolyvarTag = u32;

#[derive(Clone, Serialize, Deserialize, Debug)]
/// τ ≤ Γ(a), τ is an instance of Γ(a) and (Γ(a) a type scheme
pub struct TaggedPolyvar {
    pub(crate) polyvar_name: String,
    pub(crate) hash: PolyvarTag,
    pub(crate) polyvar_args: Vec<BinProtRule>,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(try_from = "ListTaggedEnum")]
/// Define rule reference into Resolved and Unresolved Payload
pub enum RuleRef {
    /// Unresolved Payload
    Unresolved(UnresolvedPayload),
    /// Resolved Payload
    Resolved(ResolvedPayload),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Unable to resolve reference of payload
pub struct UnresolvedPayload {
    params: Vec<BinProtRule>,
    layout_id: String, // what is longident?
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Resolved reference of payload
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
