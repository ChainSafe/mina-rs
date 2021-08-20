mod array;
mod consts;
mod de;
pub mod error;
pub mod integers;
mod loose_deserializer;
mod read_ext;
mod ser;
pub mod value;
mod write_ext;

pub use array::OcamlArray;
pub use de::{from_reader, Deserializer};
pub use read_ext::ReadBinProtExt;
pub use ser::{to_writer, Serializer};
pub use write_ext::WriteBinProtExt;
