use argon2::password_hash::rand_core::{self, RngCore};
use base64::prelude::*;
use deref_derive::Deref;

#[derive(serde::Serialize, serde::Deserialize, Deref, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(transparent)]
pub struct UserId(pub i64);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deref, Clone, Copy, PartialEq, Eq, Debug)]
pub struct LoginToken(pub [u8; 20]);

impl LoginToken {
    pub fn generate() -> Self {
        let mut contents = [0_u8; 20];
        rand_core::OsRng.fill_bytes(contents.as_mut_slice());
        Self(contents)
    }
}

impl serde::Serialize for LoginToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_URL_SAFE
            .encode(self.0.as_slice())
            .serialize(serializer)
    }
}

impl<'a> serde::Deserialize<'a> for LoginToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let str = String::deserialize(deserializer)?;
        let bytes = BASE64_URL_SAFE
            .decode(str)
            .map_err(|e| serde::de::Error::custom(e.to_string()))?
            .try_into()
            .map_err(|v: Vec<u8>| {
                serde::de::Error::custom(format!(
                    "Invalid login token length: {}, should be 20.",
                    v.len()
                ))
            })?;
        Ok(Self(bytes))
    }
}
