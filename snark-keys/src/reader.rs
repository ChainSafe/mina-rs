// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::io::{BufRead, BufReader, Read};

use crate::header::{KeyFileHeader, FILE_ID};
use crate::error::{Error, Result};

use mina_rs_base::verification_key::VerificationKey;

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
    fn smoke_test_read_file() {
        let (_header, _key) = read_snark_key_file(test_fixtures::VERIFICATION_KEY).unwrap();
    }
}
