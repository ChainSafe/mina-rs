use crate::error::Error;
use crate::Deserializer;
use serde::{Deserialize, Serialize};

/// Trait to allow a type to give a size it will serialize to/from
pub trait BinProtEncodable {
    /// yep
    const PREALLOCATE_BUFFER_BYTES: usize;

    /// yep
    fn try_encode_binprot(&self) -> Result<Vec<u8>, Error>
    where
        Self: Serialize,
    {
        let mut output = Vec::with_capacity(Self::PREALLOCATE_BUFFER_BYTES);
        crate::to_writer(&mut output, &self)?;
        Ok(output)
    }

    /// yep
    fn try_decode_binprot<'de>(bytes: impl AsRef<[u8]>) -> Result<Self, Error>
    where
        Self: Deserialize<'de> + Sized,
    {
        let mut de = Deserializer::from_reader(bytes.as_ref());
        Deserialize::deserialize(&mut de)
    }
}
