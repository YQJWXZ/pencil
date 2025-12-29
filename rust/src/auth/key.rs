use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use rsa::pkcs1::{
    DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey,
};
use rsa::{RsaPrivateKey, RsaPublicKey};

#[derive(Clone)]
pub enum JwtKeys {
    Hs256 {
        secret: String,
    },
    Rs256 {
        private_pem: String,
        public_pem: String,
        kid: Option<String>,
    },
    // Future: EdDsa { private_pem: String, public_pem: String, kid: Option<String> },
}

impl JwtKeys {
    pub fn encoding_key(&self) -> EncodingKey {
        match self {
            JwtKeys::Hs256 { secret } => EncodingKey::from_secret(secret.as_bytes()),
            JwtKeys::Rs256 { private_pem, .. } => {
                let private_key = RsaPrivateKey::from_pkcs1_pem(private_pem)
                    .expect("invalid RSA private key PEM");
                let der = private_key
                    .to_pkcs1_der()
                    .expect("failed to convert to DER")
                    .to_bytes()
                    .to_vec();
                EncodingKey::from_rsa_der(&der)
            }
        }
    }

    pub fn decoding_key(&self) -> DecodingKey {
        match self {
            JwtKeys::Hs256 { secret } => DecodingKey::from_secret(secret.as_bytes()),
            JwtKeys::Rs256 { public_pem, .. } => {
                let public_key =
                    RsaPublicKey::from_pkcs1_pem(public_pem).expect("invalid RSA public key PEM");
                let der = public_key
                    .to_pkcs1_der()
                    .expect("failed to convert to DER")
                    .as_bytes()
                    .to_vec();
                DecodingKey::from_rsa_der(&der)
            }
        }
    }
}
