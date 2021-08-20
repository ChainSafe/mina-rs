use std::marker::PhantomData;

use crate::{ReadBinProtExt, WriteBinProtExt};

use num::{FromPrimitive, Unsigned};
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use std::io::Cursor;

pub fn serialize<T, S>(n: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Unsigned + Into<u64> + Copy,
{
    let mut bytes = Vec::new();
    bytes.bin_write_nat0(*n).unwrap();
    s.serialize_bytes(&bytes)
}

struct Nat0Visitor<T>(PhantomData<T>);

impl<T> Nat0Visitor<T> {
    pub fn new() -> Self {
        Nat0Visitor::<T>(PhantomData)
    }
}

impl<'de, T> Visitor<'de> for Nat0Visitor<T>
where
    T: Unsigned + FromPrimitive,
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
        rdr.bin_read_nat0().map_err(|_| de::Error::custom(""))
    }
}

pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Unsigned + FromPrimitive,
{
    d.deserialize_bytes(Nat0Visitor::new())
}
