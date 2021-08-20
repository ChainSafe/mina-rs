use crate::error::{Error, Result};
use crate::WriteBinProtExt;
use serde::ser;
use serde::Serialize;

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Serializer<W>
where
    W: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.writer.write_all(buf)?;
        Ok(())
    }

    fn write_byte(&mut self, b: u8) -> Result<()> {
        self.write(&[b])
    }
}

pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: Serialize,
{
    value.serialize(&mut Serializer::new(writer))
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // the spec defines booleans be written as:
    // false  ->  0x00
    // true   ->  0x01
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.writer.bin_write_bool(v)?;
        Ok(())
    }

    // All integers by default get mapped to the Integer bin_prot
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.writer.bin_write_integer(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.writer.bin_write_integer(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.writer.bin_write_integer(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.writer.bin_write_integer(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write_byte(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.writer.bin_write_integer(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.writer.bin_write_integer(v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        // Requires cast to i64 as u64 does not implement Into<i64>
        // This is because a u64 can hold larger positive values than an i64 as it
        // doesn't require reserving the sign bit.
        // This is ok to do as it is never compared with a value larger than 0x80000000
        // and it is cast back to a u64 before it is serialized. Just something to be aware of.
        self.writer.bin_write_integer(v as i64)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.writer.bin_write_float32(&v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.writer.bin_write_float64(&v)?;
        Ok(())
    }

    // Chars are ascii encoded and should be 1 byte
    fn serialize_char(self, v: char) -> Result<()> {
        self.writer
            .bin_write_char(v)
            .map(|_| ())
            .map_err(Into::into)
    }

    // First the length of the string is written as a Nat0 (in characters?)
    // Then the bytes of the string verbatim
    fn serialize_str(self, v: &str) -> Result<()> {
        self.writer.bin_write_nat0(v.len() as u64)?;
        self.write(v.as_bytes())
    }

    // This must simply write the bytes to the output as is
    // The custom implementations for different integer types
    // depends on this
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write(v)
    }

    // An absent optional is represented as a unit or zero byte
    fn serialize_none(self) -> Result<()> {
        self.writer.bin_write_unit()?;
        Ok(())
    }

    // A present optional is represented as a 0x01 byte
    // followed by the encoding of the value
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_byte(0x01)?;
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data.
    // This is a zero byte
    fn serialize_unit(self) -> Result<()> {
        self.writer.bin_write_unit()?;
        Ok(())
    }

    // Unit struct means a named value containing no data.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // Newtype struct are like tuple structs with a single value
    // Just serialize the contained value
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Now we get to the serialization of compound types.
    //
    // For lists and arrays the length is written out as a Nat0.t first,
    // followed by all values in the same order as in the data structure.
    // This function only handles writing of the first element
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            // write the output length first
            self.writer.bin_write_nat0(len as u64)?;
            Ok(self) // pass self as the handler for writing the elements
        } else {
            Err(Error::SeqSizeNotProvided)
        }
    }

    // Values in tuples and records are written out one after the other in the order
    // specified in the type definition.
    // Polymorphic record fields are supported unless a value of the type bound
    // by the field were accessed, which would lead to an exception.
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    // First the size of the hash table is written out as a Nat0.t.
    // Then the writer iterates over each binding in the hash table
    // and writes out the key followed by the value.
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if let Some(len) = len {
            self.writer.bin_write_nat0(len as u64)?;
            Ok(self)
        } else {
            // size not provided. We cannot proceed
            Err(Error::MapSizeNotProvided)
        }
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    // Variants/Enums
    //
    // // Variants refer to the different possibilities of enums that hold data
    // e.g. enum Message {
    //     Quit,                          // unit variant
    //     ChangeColor(i32, i32, i32),    // tuple variant
    //     Move { x: i32, y: i32 },       // struct variant
    // }
    // In each of these cases the index of the variant, n, is written out
    // first followed by the data
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.writer.bin_write_variant_index(variant_index)?;
        Ok(())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.writer.bin_write_variant_index(variant_index)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.writer.bin_write_variant_index(variant_index)?;
        Ok(self)
    }

    // These are enum variants like Some(value)
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.writer.bin_write_variant_index(variant_index)?;
        value.serialize(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.

// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Tuples are serialized just as the elements written consecutively
impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously.
// This is no more efficient in this case
impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Structs are written exactly the same as Tuples
// Field values are written one after the other in order
// keys are ignored
impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
