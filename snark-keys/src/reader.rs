// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::io::{BufRead, BufReader, Read};

use crate::header::{KeyFileHeader, FILE_ID};
use crate::error::{Error, Result};

use mina_rs_base::verification_key::VerificationKey;

/// Read a SNARK key file from a reader and return the header and encoded key itself
/// Note: This currently only support VerificationKey as its return type. In the future this
/// will need to support other key types as well
pub fn read_snark_key_file<R: Read>(r: R) -> Result<(KeyFileHeader, VerificationKey)> {
    let mut r = BufReader::new(r);
    read_file_id(&mut r)?;
    let header = read_header(&mut r)?;
    let key = read_body(&mut r)?;
    Ok((header, key))
}

fn read_file_id<R: BufRead>(r: &mut R) -> Result<()> {
    match r.lines().next() {
        Some(Ok(s)) => {
            if s == FILE_ID {
                Ok(())
            } else {
                Err(Error::FileIdError(s))
            }
        }
        Some(Err(e)) => Err(e.into()),
        None => Err(Error::FileIdError("".to_string())),
    }
}

fn read_header<R: BufRead>(r: &mut R) -> Result<KeyFileHeader> {
    match r.lines().next() {
        Some(Ok(s)) => {
            let header = serde_json::from_str(&s)?;
            Ok(header)
        }
        Some(Err(e)) => Err(e.into()),
        None => Err(Error::UnexpectedEndOfFileError),
    }
}

fn read_body<R: Read>(r: &mut R) -> Result<VerificationKey> {
    let result = bin_prot::from_reader(r)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_id_error_if_incorrect_file_id() {
        let file_id = br#"INCORRECT_FILE_ID"#;
        let mut r = BufReader::new(&file_id[..]);
        assert!(read_file_id(&mut r).is_err());
    }

    #[test]
    fn read_file_id_read_correct_file_id() {
        let file_id = br#"MINA_SNARK_KEYS"#;
        let mut r = BufReader::new(&file_id[..]);
        assert!(read_file_id(&mut r).is_ok());
    }

    #[test]
    fn snark_key_test_read_file() {
        let (header, key) = read_snark_key_file(test_fixtures::VERIFICATION_KEY).unwrap();
        assert_eq!(header.kind.r#type, crate::KeyType::WrapVerificationKey);
        assert_eq!(key.data.constraints, 131072);
    }
}
