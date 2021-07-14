use core::convert::TryFrom;

use serde::Deserialize;
use serde_json::{Value, from_value};

#[derive(Deserialize, Debug)]
pub struct Layout {
    layout_loc: String,
    version_opt: Option<i32>,
    type_decl: String,
    bin_io_derived: bool,
    bin_prot_rule: BinProtRule,
}

#[derive(Deserialize, Debug)]
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
    Native_int,
    Float,
    Option(Box<BinProtRule>),
    Record(Vec<RecordField>),
    Tuple(Vec<BinProtRule>),
    // Sum(Vec<Summand>),
    // Polyvar(Vec<Polyvar>),
    List(Box<BinProtRule>),
    // Hashtable(HashTblEntry),
    // Vec,
    // Bigstring,
    // //  track indirections for debugging *),
    Reference(RuleRef),
    // Type_var(String),
    // //  inside a recursive type, list of type parameters *),
    // Self_reference(Vec<BinProtRule>),
    // //  parameterized type: 'a t = ... *),
    // Type_abstraction(Vec<String>, Box<BinProtRule>),
    // //  recursive parameterized type with bindings *),
    // Type_closure(Vec<(String, BinProtRule)>, Box<BinProtRule>),
}


#[derive(Deserialize)]
#[serde(untagged)]
pub enum ListTaggedEnum {
    None((String, )),
    One((String, Value)),
    Two((String, Value, Value)),
}


impl TryFrom<ListTaggedEnum> for BinProtRule {
    type Error = String;
    fn try_from(v: ListTaggedEnum) -> Result<Self, Self::Error> {
        match v {
            ListTaggedEnum::None((t, )) => match t.as_str() {
                "Int" => Ok(BinProtRule::Int),
                "String" => Ok(BinProtRule::String),
                _ => Err(format!("{} not defined yet", t)),
            },
            ListTaggedEnum::One((t, v)) => match t.as_str() {
                "Option" => Ok(BinProtRule::Option(from_value(v).unwrap())),
                "Record" => Ok(BinProtRule::Record(from_value(v).unwrap())),
                "Tuple" => Ok(BinProtRule::Tuple(from_value(v).unwrap())),
                "List" => Ok(BinProtRule::List(from_value(v).unwrap())),
                "Reference" => Ok(BinProtRule::Reference(from_value(v).unwrap())),
                _ => Err(format!("{} not defined yet", t)),
            },
            ListTaggedEnum::Two((s, _, _)) => match s.as_str() {
                "Type_closure" => Err("yay".to_string()),
                _ => Err("Unrecognized tag with two payloads".to_string())
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RecordField {
    field_name: String,
    field_rule: BinProtRule,
}

#[derive(Deserialize, Debug)]
pub struct Summand {
    ctor_name: String,
    index: i32,
    ctor_args: Vec<BinProtRule>,
}

#[derive(Deserialize, Debug)]
pub struct HashTblEntry {
    key_rule: Box<BinProtRule>,
    value_rule: Box<BinProtRule>,
}

#[derive(Deserialize, Debug)]
pub enum Polyvar {
    Tagged {
        polyvar_name: String,
        hash: i32,
        polyvar_args: Vec<BinProtRule>,
    },
    Inherited(BinProtRule),
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "ListTaggedEnum")]
pub enum RuleRef {
    Unresolved(UnresolvedPayload), 
    Resolved(ResolvedPayload),
}

#[derive(Deserialize, Debug)]
pub struct UnresolvedPayload {
    params: Vec<BinProtRule>,
    layout_id: String, // what is longident?
}

#[derive(Deserialize, Debug)]
pub struct ResolvedPayload {
    source_type_decl: String,
    #[serde(default)]
    bin_io_derived: bool, // This is present only in later versions of the type layouts. Probably don't trust its value
    ref_rule: Box<BinProtRule>,
}

impl TryFrom<ListTaggedEnum> for RuleRef {
    type Error = String;
    fn try_from(v: ListTaggedEnum) -> Result<Self, Self::Error> {
        match v {
            ListTaggedEnum::One((t, v)) => match t.as_str() {
                "Unresolved" => Ok(RuleRef::Unresolved(from_value(v).unwrap())),
                "Resolved" => Ok(RuleRef::Resolved(from_value(v).unwrap())),
                _ => Err("Unexpected tag for this type".to_string())
            },
            _ => Err("Unexpected number of items in enum body".to_string()),
        }
    }
}
