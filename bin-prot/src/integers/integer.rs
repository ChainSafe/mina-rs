use core::marker::PhantomData;
use num::FromPrimitive;
use std::io::Cursor;

use crate::{ReadBinProtExt, WriteBinProtExt};
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;

pub fn serialize<T, S>(n: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<i64> + Copy,
{
    let mut bytes = Vec::<u8>::new();
    bytes.bin_write_integer(*n).unwrap();
    s.serialize_bytes(&bytes)
}

struct IntegerVisitor<T>(PhantomData<T>);

impl<T> IntegerVisitor<T> {
    pub fn new() -> Self {
        IntegerVisitor::<T>(PhantomData)
    }
}

impl<'de, T> Visitor<'de> for IntegerVisitor<T>
where
    T: FromPrimitive,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A bin_prot encoded integer (1, 3, 5, or 9 bytes depending on size)")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut rdr = Cursor::new(value);
        rdr.bin_read_integer()
            .map_err(|_| de::Error::custom("Failed to read valid integer"))
    }
}

pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromPrimitive,
{
    d.deserialize_bytes(IntegerVisitor::new())
}
