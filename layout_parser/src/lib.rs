use serde::Deserializer;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Layout {
    layout_loc: String,
    version_opt: Option<i32>,
    type_decl: String,
    bin_io_derived: bool,
    bin_prot_rule: BinProtRule,
}

#[derive(Deserialize, Debug)]
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
    Sum(Vec<Summand>),
    Polyvar(Vec<Polyvar>),
    List(Box<BinProtRule>),
    Hashtable(HashTblEntry),
    Vec,
    Bigstring,
    //  track indirections for debugging *),
    Reference(RuleRef),
    Type_var(String),
    //  inside a recursive type, list of type parameters *),
    Self_reference(Vec<BinProtRule>),
    //  parameterized type: 'a t = ... *),
    Type_abstraction(Vec<String>, Box<BinProtRule>),
    //  recursive parameterized type with bindings *),
    Type_closure(Vec<(String, BinProtRule)>, Box<BinProtRule>),
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
pub enum RuleRef {
    Unresolved {
        params: Vec<BinProtRule>,
        layout_id: String,
    }, // what is longident?
    Resolved {
        source_type_decl: String,
        bin_io_derived: bool,
        ref_rule: Box<BinProtRule>,
    },
}
