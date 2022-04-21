use super::*;
use argon2::password_hash::SaltString;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(super) struct SecretBoxJson {
    pub box_primitive: String,
    pub pw_primitive: String,
    pub nonce: String,
    pub pwsalt: String,
    pub pwdiff: [i64; 2],
    pub ciphertext: String,
}

#[derive(Clone, Debug)]
pub struct SecretBox {
    pub(super) box_primitive: String,
    pub(super) pw_primitive: String,
    pub(super) nonce: Vec<u8>,
    pub(super) pwsalt: SaltString,
    pub(super) pw_mem_limit_bytes: i64,
    pub(super) pw_ops_limit: u32,
    pub(super) ciphertext: Vec<u8>,
}
