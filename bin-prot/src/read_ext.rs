use crate::consts::*;
use crate::error::{Error, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use num::{FromPrimitive, Unsigned};
use std::io;

// Extension trait for readers implementing io::Read to allow them to read a bin_prot encoded
// integer
pub trait ReadBinProtExt: io::Read {
    fn bin_read_unit(&mut self) -> Result<()> {
        match self.read_u8()? {
            0x00 => Ok(()),
            b => Err(Error::InvalidByte {
                byte: b,
                dtype: "unit".to_string(),
                allowed: vec![0x00],
            }),
        }
    }

    fn bin_read_bool(&mut self) -> Result<bool> {
        match self.read_u8()? {
            0x00 => Ok(false),
            0x01 => Ok(true),
            b => Err(Error::InvalidByte {
                byte: b,
                dtype: "bool or option".to_string(),
                allowed: vec![0x00, 0x01],
            }),
        }
    }

    // This function reads a single byte as char
    fn bin_read_char(&mut self) -> Result<char> {
        Ok(self.read_u8()? as char)
    }

    fn bin_read_integer<T: FromPrimitive>(&mut self) -> Result<T> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        // for the possibly signed cases, read them as signed and allow
        // the conversion to fail if trying to convert a negative value
        // to an unsigned integer
        match buf[0] {
            CODE_INT16 => {
                // positive or negative 16 bit int
                T::from_i16(self.read_i16::<LittleEndian>()?)
            }
            CODE_INT32 => {
                // positive or negative 32 bit int
                T::from_i32(self.read_i32::<LittleEndian>()?)
            }
            CODE_INT64 => {
                // positive or negative 64 bit int
                T::from_i64(self.read_i64::<LittleEndian>()?)
            }
            CODE_NEG_INT8 => {
                // a negative signed i8
                T::from_i8(self.read_i8()?)
            }
            byte0 => {
                // first byte isnt a code so interpret it as a u8
                if byte0 > 0x000000080 {
                    return Err(Error::InvalidIntegerByte { byte: byte0 });
                }
                T::from_u8(byte0)
            }
        }
        .ok_or(Error::DestinationIntegerOverflow)
    }

    fn bin_read_nat0<T: FromPrimitive + Unsigned>(&mut self) -> Result<T> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        // In this case it is always reading an unsigned integer
        match buf[0] {
            CODE_INT16 => {
                // positive or negative 16 bit int
                T::from_u16(self.read_u16::<LittleEndian>()?)
            }
            CODE_INT32 => {
                // positive or negative 32 bit int
                T::from_u32(self.read_u32::<LittleEndian>()?)
            }
            CODE_INT64 => {
                // positive or negative 64 bit int
                T::from_u64(self.read_u64::<LittleEndian>()?)
            }
            byte0 => {
                // first byte isnt a code so interpret it as a u8
                if byte0 > 0x000000080 {
                    return Err(Error::InvalidIntegerByte { byte: byte0 });
                }
                T::from_u8(byte0)
            }
        }
        .ok_or(Error::DestinationIntegerOverflow)
    }

    fn bin_read_variant_index(&mut self) -> Result<u8> {
        self.read_u8().map_err(Error::Io)
    }

    fn bin_read_string(&mut self) -> Result<String> {
        let len = self.bin_read_nat0::<u64>()? as usize;
        let mut buf = vec![0u8; len as usize];
        self.read_exact(&mut buf)?;
        let s = std::str::from_utf8(&buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        Ok(s.to_string())
    }

    fn bin_read_bytes(&mut self) -> Result<Vec<u8>> {
        let len = self.bin_read_nat0::<u64>()? as usize;
        let mut buf = vec![0u8; len as usize];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

/// All types that implement `Read` get methods defined in `ReadBinProtIntegerExt`
/// for free.
impl<W: io::Read + ?Sized> ReadBinProtExt for W {}
