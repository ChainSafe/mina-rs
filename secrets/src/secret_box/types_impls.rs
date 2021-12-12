use super::*;
use argon2::{password_hash::SaltString, Argon2, ParamsBuilder, PasswordHasher};
use xsalsa20poly1305::{
    aead::{generic_array::GenericArray, Aead, NewAead},
    XSalsa20Poly1305,
};

impl TryFrom<SecretBoxJson> for SecretBox {
    type Error = Error;
    fn try_from(json: SecretBoxJson) -> Result<Self, Self::Error> {
        if json.box_primitive != SECRET_BOX_PRIMITIVE {
            return Err(Error::InvalidSecretBoxPrimitiveError);
        }
        if json.pw_primitive != PASSWORD_HASH_PRIMITIVE {
            return Err(Error::InvalidPasswordHashPrimitiveError);
        }

        let pwsalt = {
            let pwsalt_bytes = base58_decode(json.pwsalt)?;
            SaltString::b64_encode(pwsalt_bytes.as_slice())
                .map_err(|e| Error::PasswordHashError(format!("{}", e)))?
        };
        Ok(Self {
            box_primitive: json.box_primitive,
            pw_primitive: json.pw_primitive,
            nonce: base58_decode(json.nonce)?,
            pwsalt,
            pw_mem_limit_bytes: json.pwdiff[0],
            pw_ops_limit: json.pwdiff[1] as u32,
            ciphertext: base58_decode(json.ciphertext)?,
        })
    }
}

impl TryFrom<&str> for SecretBox {
    type Error = Error;
    fn try_from(json_str: &str) -> Result<Self, Self::Error> {
        let json: SecretBoxJson =
            serde_json::from_str(json_str.as_ref()).map_err(Error::JsonSerdeError)?;
        json.try_into()
    }
}

impl Into<SecretBoxJson> for SecretBox {
    fn into(self) -> SecretBoxJson {
        let pwsalt = {
            // 16 is sufficient here, alloc array on stack instead
            // let mut buf = vec![0; self.pwsalt.len()];
            let mut buf = [0; 16];
            let bytes = self.pwsalt.b64_decode(&mut buf).unwrap();
            base58_encode(bytes)
        };
        SecretBoxJson {
            box_primitive: self.box_primitive,
            pw_primitive: self.pw_primitive,
            nonce: base58_encode(self.nonce),
            pwsalt,
            pwdiff: [self.pw_mem_limit_bytes, self.pw_ops_limit as i64],
            ciphertext: base58_encode(self.ciphertext),
        }
    }
}

impl TryInto<String> for SecretBox {
    type Error = Error;
    fn try_into(self) -> Result<String, Error> {
        let json: SecretBoxJson = self.into();
        serde_json::to_string_pretty(&json).map_err(Error::JsonSerdeError)
    }
}

impl TryInto<serde_json::Value> for SecretBox {
    type Error = Error;
    fn try_into(self) -> Result<serde_json::Value, Error> {
        let s: String = self.try_into()?;
        serde_json::from_str(&s).map_err(Error::JsonSerdeError)
    }
}

impl SecretBox {
    pub(super) fn try_gen_secret(&self, password: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
        let hasher = (move || {
            let mut param_builder = ParamsBuilder::new();
            param_builder.m_cost((self.pw_mem_limit_bytes / 1024) as u32)?;
            param_builder.t_cost(self.pw_ops_limit)?;
            Ok(Argon2::new(
                argon2::Algorithm::Argon2i,
                argon2::Version::V0x13,
                param_builder.params()?,
            ))
        })()
        .map_err(|e: argon2::Error| Error::Argon2Error(format!("{}", e)))?;

        if let Some(hash) = hasher
            .hash_password(password.as_ref(), &self.pwsalt)
            .map_err(|e: argon2::password_hash::Error| Error::PasswordHashError(format!("{}", e)))?
            .hash
        {
            Ok(hash.as_bytes().clone().into())
        } else {
            Err(Error::Argon2Error("Empty hash output".into()))
        }
    }

    pub fn try_get_private_key(&self, password: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
        let secret = self.try_gen_secret(password)?;
        let key = GenericArray::from_slice(secret.as_slice());
        let cipher = XSalsa20Poly1305::new(key);
        let mut bytes = cipher
            .decrypt(
                GenericArray::from_slice(self.nonce.as_slice()),
                self.ciphertext.as_ref(),
            )
            .map_err(|e| Error::AeadError(format!("{}", e)))?;
        bytes.remove(0);
        Ok(bytes)
    }
}
