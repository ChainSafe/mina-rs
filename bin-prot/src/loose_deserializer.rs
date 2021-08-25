// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

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
            match iter.next() {
                Ok(Some(rule)) => {
                    match rule {
                        BinProtRule::Unit => {
                            self.deserialize_unit(visitor)
                        }
                        BinProtRule::Record(fields) => {
                            // Grab the field names from the rule to pass to the map access
                            visitor.visit_map(MapAccess::new(
                                self,
                                fields.into_iter().map(|f| f.field_name).rev().collect(),
                            ))
                        }
                        BinProtRule::Tuple(items) => {
                            visitor.visit_seq(SeqAccess::new(self, items.len()))
                        }
                        BinProtRule::Sum(summands) => {
                            // read the enum variant index.
                            // We need this to select which variant layout to use
                            // when deserializing the variants data
                            let index = self.rdr.bin_read_variant_index()?;
                            iter.branch(index.into())?;
                            visitor.visit_enum(ValueEnum::new(
                                self,
                                summands[index as usize].clone(),
                            ))
                        }
                        BinProtRule::Bool => {
                            self.deserialize_bool(visitor)
                        }
                        BinProtRule::Option(_) => {
                            let index = self.rdr.bin_read_variant_index()?; // 0 or 1
                            match index {
                                0 => {
                                    iter.branch(0)?;
                                    visitor.visit_none()
                                }
                                1 => {
                                    iter.branch(1)?;
                                    visitor.visit_some(self)
                                },
                                _ => {
                                    Err(Error::InvalidOptionByte{ got: index })
                                }
                            }

                        }
                        BinProtRule::String => {
                            visitor.visit_bytes(&self.rdr.bin_read_bytes()?)
                        }
                        BinProtRule::Float => self.deserialize_f64(visitor),
                        BinProtRule::Char => {
                            let c = self.rdr.read_u8()?;
                            visitor.visit_char(c as char)
                        }
                        BinProtRule::List(_) => {
                            // read the length
                            let len = self.rdr.bin_read_nat0()?;
                            // request the iterator repeats the list elements the current number of times
                            iter.repeat(len);
                            // read the elements
                            visitor.visit_seq(SeqAccess::new(self, len))
                        }
                        BinProtRule::Int
                        | BinProtRule::Int32
                        | BinProtRule::Int64
                        | BinProtRule::NativeInt => {
                            visitor.visit_i64(self.rdr.bin_read_integer()?)
                        }
                        BinProtRule::Polyvar(_)
                        | BinProtRule::Vec(_, _)
                        | BinProtRule::Nat0
                        | BinProtRule::Hashtable(_)
                        | BinProtRule::TypeVar(_)
                        | BinProtRule::Bigstring
                        | BinProtRule::SelfReference(_)
                        | BinProtRule::TypeClosure(_, _)
                        | BinProtRule::TypeAbstraction(_, _)
                        | BinProtRule::Reference(_) => {
                            Err(Error::UnimplementedRule)
                        } // Don't know how to implement these yet
                        BinProtRule::Custom(_) => {
                            // the traverse function should never produce this
                            Err(Error::LayoutIteratorError)
                        }
                        BinProtRule::CustomForPath(path, rules) => {
                            // here is where custom deser methods can be looked up by path
                            match path.as_str() {
                                // These vector types will be handled like any other sequence
                                "Pickles_type.Vector.Vector2" // the missing 's' on 'types' here is intention due to a big in layout producing code
                                |"Pickles_types.Vector.Vector2" // in case it gets fixed :P
                                | "Pickles_types.Vector.Vector4"
                                | "Pickles_types.Vector.Vector8"
                                | "Pickles_types.Vector.Vector17"
                                | "Pickles_types.Vector.Vector18" => {
                                    let element_rule = rules.first().unwrap();
                                    let len = match path.as_str() {
                                        "Pickles_type.Vector.Vector2"
                                        | "Pickles_types.Vector.Vector2" => 2,
                                        "Pickles_types.Vector.Vector4" => 4,
                                        "Pickles_types.Vector.Vector8" => 8,
                                        "Pickles_types.Vector.Vector17" => 17,
                                        "Pickles_types.Vector.Vector18" => 18,
                                        _ => unreachable!()
                                    };
                                    iter.push(element_rule.clone());
                                    iter.repeat(len);
                                    let result = visitor.visit_seq(SeqAccess::new(self, len));
                                    // burn the zero byte terminator
                                    assert!(self.rdr.read_u8()? == 0x00);

                                    result
                                }
                                "Ledger_hash0" // these are all BigInt (32 bytes)
                                | "State_hash"
                                | "Pending_coinbase.Stack_hash"
                                | "State_body_hash"
                                | "Pending_coinbase.Hash_builder"
                                | "Snark_params.Make_inner_curve_scalar"
                                | "Snark_params.Tick"
                                | "Epoch_seed"
                                | "Zexe_backend.Zexe_backend_common.Stable.Field"
                                | "Pending_coinbase.Coinbase_stack" => {                         
                                   let mut buf: [u8; 32] = [0x00; 32];
                                    for i in 0..32 {
                                        buf[i] = self.rdr.read_u8()?;
                                    }
                                    visitor.visit_bytes(&buf)
                                }
                                _ => Err(Error::UnknownCustomType{ typ: path })
                            }
                        }
                    }
                }
                Err(_e) => {
                    Err(Error::LayoutIteratorError)
                }
                Ok(None) => {
                    Err(Error::UnexpectedEndOfLayout)
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