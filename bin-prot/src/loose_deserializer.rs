use std::convert::TryInto;
use std::io::Read;

use crate::de::{Enum, MapAccess, SeqAccess};
use crate::error::{Error, Result};
use crate::value::layout::Summand;
use crate::value::layout::{BinProtRule, BranchingIterator};
use crate::Deserializer as DS;
use crate::ReadBinProtExt;
use serde::de::{Deserializer, Visitor};
use serde::{Deserialize, Serialize};

use byteorder::ReadBytesExt;

impl<'de, 'a, R: Read> DS<R> {
    pub fn deserialize_loose<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(iter) = &mut self.layout_iter {
            loop {
                match iter.next() {
                    Ok(Some(rule)) => {
                        match rule {
                            BinProtRule::Unit => {
                                return self.deserialize_unit(visitor);
                            }
                            BinProtRule::Record(fields) => {
                                // Grab the field names from the rule to pass to the map access
                                return visitor.visit_map(MapAccess::new(
                                    self,
                                    fields.into_iter().map(|f| f.field_name).rev().collect(),
                                ));
                            }
                            BinProtRule::Tuple(items) => {
                                return visitor.visit_seq(SeqAccess::new(self, items.len()));
                            }
                            BinProtRule::Sum(summands) => {
                                // read the enum variant index.
                                // We need this to select which variant layout to use
                                // when deserializing the variants data
                                let index = self.rdr.bin_read_variant_index()?;
                                iter.branch(index.into()).expect("invalid branch index");
                                return visitor.visit_enum(ValueEnum::new(
                                    self,
                                    summands[index as usize].clone(),
                                ));
                            }
                            BinProtRule::Bool => {
                                return self.deserialize_bool(visitor);
                            }
                            BinProtRule::Option(_) => {
                                return self.deserialize_option(visitor);
                            }
                            BinProtRule::Reference(_) => {} // continue iterator
                            BinProtRule::String => {
                                return visitor.visit_bytes(&self.rdr.bin_read_bytes()?);
                            }
                            BinProtRule::Float => return self.deserialize_f64(visitor),
                            BinProtRule::Char => {
                                let c = self.rdr.read_u8()?;
                                return visitor.visit_char(c as char);
                            }
                            BinProtRule::List(_) => {
                                // read the length
                                let len = self.rdr.bin_read_nat0()?;
                                // request the iterator repeats the list elements the current number of times
                                iter.repeat(len);
                                // read the elements
                                return visitor.visit_seq(SeqAccess::new(self, len));
                            }
                            BinProtRule::Int
                            | BinProtRule::Int32
                            | BinProtRule::Int64
                            | BinProtRule::NativeInt => {
                                return visitor.visit_i64(self.rdr.bin_read_integer()?);
                            }
                            BinProtRule::Polyvar(_)
                            | BinProtRule::Vec(_, _)
                            | BinProtRule::Nat0
                            | BinProtRule::Hashtable(_)
                            | BinProtRule::TypeVar(_)
                            | BinProtRule::Bigstring
                            | BinProtRule::SelfReference(_)
                            | BinProtRule::TypeClosure(_, _)
                            | BinProtRule::TypeAbstraction(_, _) => {
                                return Err(Error::Custom {
                                    message: format!("No strategy to deserialize {:?}", rule),
                                })
                            } // Don't know how to implement these yet
                            BinProtRule::Custom(_) => {
                                // the traverse function should never produce this
                                return Err(Error::Custom {
                                    message: "Cannot deserialize custom without providing context"
                                        .to_string(),
                                });
                            }
                            BinProtRule::CustomForPath(path, rules) => {
                                // here is where custom deser methods can be looked up by path
                                match path.as_str() {
	                                "Pickles_type.Vector.Vector2" // the missing 's' on 'types' here is intention due to a big in layout producing code
	                                | "Pickles_types.Vector.Vector4"
	                                | "Pickles_types.Vector.Vector8"
	                                | "Pickles_types.Vector.Vector17"
	                                | "Pickles_types.Vector.Vector18" => {
	                                    let element_rule = rules.first().unwrap();

	                                    let len = match path.as_str() {
	                                        "Pickles_type.Vector.Vector2" => 2,
	                                        "Pickles_types.Vector.Vector4" => 4,
	                                        "Pickles_types.Vector.Vector8" => 8,
	                                        "Pickles_types.Vector.Vector17" => 17,
	                                        "Pickles_types.Vector.Vector18" => 18,
	                                        _ => {
	                                            return Err(Error::Custom {
	                                                message: "Unknown custom vector type"
	                                                    .to_string(),
	                                            })
	                                        }
	                                    };

	                                    iter.push(element_rule.clone());
	                                    iter.repeat(len);
	                                    let result = visitor.visit_seq(SeqAccess::new(self, len));
	                                    // burn the zero byte terminator
	                                    assert!(self.rdr.read_u8()? == 0x00);

	                                    return result;
	                                }

	                                _ => {
	                                    // all the others are just BigInt probably so burn 32 bytes
	                                    for _ in 0..(8 * 4) {
	                                        // TODO: Read these and wrap as a proper Value variant
	                                        self.rdr.read_u8()?;
	                                    }
	                                }
	                            }
                            }
                        }
                    }
                    Err(_e) => {
                        return Err(Error::Custom {
                            message: "Iterator errored...".to_string(),
                        })
                    }
                    Ok(None) => {
                        return Err(Error::Custom {
                            message: "Unxepected end of layout".to_string(),
                        })
                    }
                }
            }
        } else {
            Err(Error::WontImplement)
        }
    }
}

// for accessing enums when using the loosely typed method
// to deserialize into a Value
pub struct ValueEnum<'a, R: Read> {
    de: &'a mut DS<R>,
    variant: Summand,
}

#[derive(Serialize, Deserialize)]
pub struct EnumData {
    pub index: u8,
    pub name: String,
}

impl<'a, 'de, R: Read> ValueEnum<'a, R> {
    fn new(de: &'a mut DS<R>, variant: Summand) -> Self {
        Self { de, variant }
    }
}

impl<'de, 'a, R: Read> serde::de::EnumAccess<'de> for ValueEnum<'a, R> {
    type Error = Error;
    type Variant = Enum<'a, R>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let index = self.variant.index;

        // bit of a hack here. visit_enum in the visitor is expecting to be able to
        // deserialize the enum details (e.g. variant index and name) from the stream.
        // Since in this case it comes from the layout file we need to serialize this data
        // and then return the deserializer to be handled by visit_enum

        let enum_data = EnumData {
            index: index.try_into().unwrap(),
            name: self.variant.ctor_name,
        };
        let mut buf = Vec::<u8>::new();
        crate::to_writer(&mut buf, &enum_data).unwrap();
        let mut de = DS::from_reader(buf.as_slice());
        let v = seed.deserialize(&mut de)?;

        Ok((v, Enum::new(self.de, index.try_into().unwrap())))
    }
}
