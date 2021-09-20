use crate::Value;

use serde::ser::{SerializeStruct, SerializeTuple, SerializeTupleVariant};
use serde::Serialize;

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match *self {
            Value::Unit => serializer.serialize_unit(),
            Value::Nat0(v) => v.serialize(serializer),
            Value::Bool(b) => serializer.serialize_bool(b),
            Value::String(ref s) => s.serialize(serializer),
            Value::Char(c) => serializer.serialize_char(c),
            Value::Int(v) => v.serialize(serializer),
            Value::Float(v) => v.serialize(serializer),
            Value::Option(ref o) => {
                if let Some(v) = o {
                    serializer.serialize_some(v)
                } else {
                    serializer.serialize_none()
                }
            }
            Value::Record(ref m) => {
                // name of struct and field are lost when serializing
                let mut map = serializer.serialize_struct(&"", m.len())?;
                for (_k, v) in m {
                    map.serialize_field(&"", v)?;
                }
                map.end()
            }
            Value::Tuple(ref vals) => {
                let mut t = serializer.serialize_tuple(vals.len())?;
                for v in vals {
                    t.serialize_element(v)?;
                }
                t.end()
            }
            Value::Sum {
                name: _,
                ref index,
                ref value,
            } => {
                let mut sum = serializer.serialize_tuple_variant(&"", *index as u32, &"", 1)?;
                sum.serialize_field(value)?;
                sum.end()
            } // sum types/enums
            Value::List(ref v) => v.serialize(serializer),
        }
    }
}
