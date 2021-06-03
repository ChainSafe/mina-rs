use bs58::encode::EncodeBuilder;
use std::io::Write;

pub use bs58::decode::Error;
pub use bs58::{decode, encode};

pub trait MinaBase58: AsRef<[u8]> {
    /// This is the only method a custom implementation need provide.
    /// Should be a constant from the base58_version_bytes.rs file corresponding
    /// to the type.
    fn version_byte() -> u8;

    fn to_base58(self) -> EncodeBuilder<'static, Self>
    where
        Self: Sized,
    {
        encode(self).with_check_version(Self::version_byte())
    }

    fn from_base58<I>(i: I) -> Result<Self, Error>
    where
        I: AsRef<[u8]>,
        Self: Sized + AsMut<[u8]> + Default,
    {
        // because of the version prefix and check bytes we
        // need a buffer that is 5 bytes bigger than the data type
        // so we cannot just decode straight into the data structure :(
        let bytes: Vec<u8> = decode(i)
            .with_check(Some(Self::version_byte()))
            .into_vec()?;
        println!("{}", bytes.len());
        println!("{:?}", bytes);
        let mut o = Self::default();
        // skip the version byte that gets written to the buffer
        o.as_mut()
            .write(&bytes[1..])
            .map_err(|_| Error::BufferTooSmall)?;
        Ok(o)
    }
}
