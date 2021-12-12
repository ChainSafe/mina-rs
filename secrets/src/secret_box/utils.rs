use super::*;

const VERSION_CHECK_BYTE: u8 = 0x02;

pub(super) fn base58_decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
    let mut v = bs58::decode(input)
        .with_check(Some(VERSION_CHECK_BYTE))
        .into_vec()
        .map_err(Error::Base58DecodeError)?;
    // Remove version byte
    v.remove(0);
    Ok(v)
}

pub(super) fn base58_encode(input: impl AsRef<[u8]>) -> String {
    bs58::encode(input)
        .with_check_version(VERSION_CHECK_BYTE)
        .into_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const B58_ENCODED: &str =
        "C6akiMy3NarEYxkQ2ta4AweAz67Cq1Qn81VxdX5SQKUXebAEcrY97DRb5AdWxQdG9msbXWUqr";
    const HEX_ENCODED:&str = "5f742829b76350de2f965cb432967076e3fe8ee8d103cc8e4fd947e8ad09e9123a88c19e482810b034fc47f5a038864351";

    #[test]
    fn decode() {
        let bytes = base58_decode(B58_ENCODED).unwrap();
        assert_eq!(hex::encode(&bytes), HEX_ENCODED);
    }

    #[test]
    fn encode() {
        let bytes = hex::decode(HEX_ENCODED).unwrap();
        let encoded = base58_encode(&bytes);
        assert_eq!(encoded, B58_ENCODED);
    }
}
