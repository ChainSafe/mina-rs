use std::io::{BufReader, Read};

use crate::error::{Error, Result};
use crate::value::layout::{BinProtRule, BinProtRuleIterator};
use crate::ReadBinProtExt;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::{self, value::U8Deserializer, EnumAccess, IntoDeserializer, Visitor};
use serde::Deserialize;

pub struct Deserializer<R: Read> {
    pub rdr: BufReader<R>,
    pub layout_iter: Option<BinProtRuleIterator>,
}

impl<R: Read> Deserializer<R> {
    pub fn from_reader(rdr: R) -> Self {
        Self {
            rdr: BufReader::new(rdr),
            layout_iter: None,
        }
    }

    pub fn from_reader_with_layout(rdr: R, layout: BinProtRule) -> Self {
        Self {
            rdr: BufReader::new(rdr),
            layout_iter: Some(layout.into_branching_iter()),
        }
    }
}

pub fn from_reader<'de, R: Read, T: Deserialize<'de>>(rdr: R) -> Result<T> {
    let mut de = Deserializer::from_reader(rdr);
    let value = Deserialize::deserialize(&mut de)?;
    Ok(value)
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_loose(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.rdr.bin_read_bool()?)
    }

    // all native integer types targets are interpreted as variable length integer
    // THey will attempt to fit into the target byte size and error if too large
    // Implementations should use attribute tags to deserialize fixed length types
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.rdr.bin_read_integer()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.rdr.bin_read_integer()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.rdr.bin_read_integer()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.rdr.bin_read_integer()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.rdr.bin_read_integer()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.rdr.bin_read_integer()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.rdr.bin_read_integer()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.rdr.bin_read_integer()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.rdr.read_f32::<LittleEndian>()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.rdr.read_f64::<LittleEndian>()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.rdr.bin_read_char()?)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.rdr.bin_read_string()?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::WontImplement)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::WontImplement)
    }

    // An absent optional is represented as 0x00
    // A present optional is 0x01 followed by the encoded value
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.rdr.bin_read_bool()? {
            false => visitor.visit_none(),
            true => visitor.visit_some(self),
        }
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.rdr.bin_read_unit()?;
        visitor.visit_unit()
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Parsing an unknown length seq (e.g array, list) involves
    // first reading the length as a Nat0 and then parsing the next values
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.rdr.bin_read_nat0()?;
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    // Tuples look just like sequences
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    // Tuple structs look just like sequences
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    // Similarly need to read the length as a Nat0 then decode values
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // we can't know the field names (and don't need to) if we are deserializing in
        // stronly typed mode. To make everything work just add some dummy field names
        let len: usize = self.rdr.bin_read_nat0()?;
        let dummy_fields = std::iter::repeat("".to_string()).take(len).collect();
        visitor.visit_map(MapAccess::new(self, dummy_fields))
    }

    // Structs look just like sequences
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, fields.len()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let index = self.rdr.bin_read_variant_index()?;
        visitor.visit_enum(Enum::new(self, index))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // identifiers are not used as it is a binary protocol
        Err(Error::WontImplement)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

pub(crate) struct SeqAccess<'a, R: Read + 'a> {
    de: &'a mut Deserializer<R>,
    len: usize,
}

impl<'a, R: Read + 'a> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, len: usize) -> Self {
        Self { de, len }
    }
}

impl<'de: 'a, 'a, R: Read> de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>> {
        if self.len > 0 {
            self.len -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

pub(crate) struct MapAccess<'a, R: Read + 'a> {
    de: &'a mut Deserializer<R>,
    field_names: Vec<String>, // field names should be stored as a stack (first element last)
}

impl<'a, R: Read + 'a> MapAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, field_names: Vec<String>) -> Self {
        Self { de, field_names }
    }
}

impl<'de: 'a, 'a, R: Read> de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if let Some(name) = self.field_names.pop() {
            // create a new deserializer to read the name from memory
            // as it isn't present in the serialized output
            seed.deserialize(name.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<T::Value> {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.field_names.len())
    }
}

pub struct Enum<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    index: u8,
}

impl<'a, 'de, R: Read> Enum<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, index: u8) -> Self {
        Enum { de, index }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
impl<'de, 'a, R: Read> EnumAccess<'de> for Enum<'a, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let de: U8Deserializer<Self::Error> = (self.index as u8).into_deserializer();
        let v = seed.deserialize(de)?;
        Ok((v, self))
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a, R: Read> de::VariantAccess<'de> for Enum<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}
