use base64::prelude::*;
use iroh::{PublicKey, SecretKey, Signature, SignatureError};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(try_from = "String", into = "String")]
pub struct NetworkIdentity(SecretKey);

impl Type for NetworkIdentity {
    fn inline(type_map: &mut specta::TypeCollection, generics: specta::Generics) -> specta::datatype::DataType {
        String::inline(type_map, generics)
    }
}

impl From<SecretKey> for NetworkIdentity {
    fn from(value: SecretKey) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for NetworkIdentity {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let decoded = BASE64_URL_SAFE_NO_PAD.decode(value)?;
        Ok(Self(SecretKey::try_from(decoded.as_slice())?))
    }
}

impl Into<String> for NetworkIdentity {
    fn into(self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.0.to_bytes())
    }
}

impl From<NetworkIdentity> for SecretKey {
    fn from(value: NetworkIdentity) -> Self {
        value.private_key()
    }
}

impl Default for NetworkIdentity {
    fn default() -> Self {
        Self::generate()
    }
}

impl NetworkIdentity {
    pub fn generate() -> Self {
        Self(SecretKey::generate(&mut rand::rng()))
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.public()
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.0.sign(msg)
    }

    pub fn private_key(&self) -> SecretKey {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct PeerIdentity(PublicKey);

impl Type for PeerIdentity {
    fn inline(type_map: &mut specta::TypeCollection, generics: specta::Generics) -> specta::datatype::DataType {
        String::inline(type_map, generics)
    }
}

impl PeerIdentity {
    pub fn into_inner(self) -> PublicKey {
        self.0
    }

    pub fn short_format(&self) -> String {
        self.0.fmt_short().to_string()
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SignatureError> {
        self.0.verify(message, signature)
    }
}

impl TryFrom<String> for PeerIdentity {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let decoded = BASE64_URL_SAFE_NO_PAD.decode(value)?;
        Ok(Self(PublicKey::try_from(decoded.as_slice())?))
    }
}

impl Into<String> for PeerIdentity {
    fn into(self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.0.as_bytes())
    }
}

impl From<PublicKey> for PeerIdentity {
    fn from(value: PublicKey) -> Self {
        Self(value)
    }
}

impl From<PeerIdentity> for PublicKey {
    fn from(value: PeerIdentity) -> Self {
        value.into_inner()
    }
}
