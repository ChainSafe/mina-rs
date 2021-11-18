// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::io::{BufRead, BufReader, Read};

mod header;
use header::{KeyFileHeader, FILE_ID};

mod error;
use error::{Error, Result};

mod verification_key;
use verification_key::VerificationKey;

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

    const TEST_KEYFILE: &[u8] = include_bytes!(
        "../vk-wrap-wrap-verification-key-blockchain-snark-d3623dbfa42f563e40cd5f2d032ad91f"
    );

    #[test]
    fn smoke_test_read_file() {
        let mut r = BufReader::new(TEST_KEYFILE);
        read_file_id(&mut r).unwrap();
        
        let (header, key) = read_snark_key_file(TEST_KEYFILE).unwrap();

        println!("{:?}", header);
        println!("{:#?}", key);
    }
}
