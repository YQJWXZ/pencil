use crate::auth::key::JwtKeys;
use anyhow::{Context, Result, anyhow};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rsa::pkcs1::{DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use rsa::rand_core::OsRng;
use rsa::traits::PublicKeyParts;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub struct RsaKeyManager {
    private_key_path: PathBuf,
    public_key_path: PathBuf,
}

impl RsaKeyManager {
    pub fn from_config(jwt_cfg: &Value) -> Result<Self> {
        let priv_path = jwt_cfg
            .get("private_key_path")
            .and_then(|s| s.as_str())
            .unwrap_or("fixtures/jwt_dev_private.pem")
            .to_string();
        let pub_path = jwt_cfg
            .get("public_key_path")
            .and_then(|s| s.as_str())
            .unwrap_or("fixtures/jwt_dev_public.pem")
            .to_string();
        Ok(Self {
            private_key_path: PathBuf::from(priv_path),
            public_key_path: PathBuf::from(pub_path),
        })
    }

    /// 保证 RSA 密钥对存在，如果缺失，则生成 2048 位密钥对并写入 PEM 文件。
    pub fn ensure_keys(&self) -> Result<JwtKeys> {
        let private_exists = self.private_key_path.exists();
        let public_exists = self.public_key_path.exists();

        if !private_exists || !public_exists {
            self.generate_and_store_keys()?;
        }

        let private_pem = fs::read_to_string(&self.private_key_path)
            .with_context(|| format!("read private key {}", self.private_key_path.display()))?;
        let public_pem = fs::read_to_string(&self.public_key_path)
            .with_context(|| format!("read public key {}", self.public_key_path.display()))?;

        let kid = Some(compute_kid_from_public_pem(&public_pem)?);
        Ok(JwtKeys::Rs256 {
            private_pem,
            public_pem,
            kid,
        })
    }

    fn generate_and_store_keys(&self) -> Result<()> {
        if let Some(parent) = self.private_key_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).with_context(|| {
                format!("create dir for private key {}", parent.to_string_lossy())
            })?;
        }
        if let Some(parent) = self.public_key_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).with_context(|| {
                format!("create dir for public key {}", parent.to_string_lossy())
            })?;
        }

        let mut rng = OsRng;
        let private = RsaPrivateKey::new(&mut rng, 2048)
            .map_err(|e| anyhow!("RSA key generation failed: {}", e))?;
        let public = RsaPublicKey::from(&private);

        let private_pem = private
            .to_pkcs1_pem(LineEnding::LF)
            .map_err(|e| anyhow!("encode private key pem failed: {}", e))?
            .to_string();
        let public_pem = public
            .to_pkcs1_pem(LineEnding::LF)
            .map_err(|e| anyhow!("encode public key pem failed: {}", e))?
            .to_string();

        fs::write(&self.private_key_path, private_pem)
            .with_context(|| format!("write private key {}", self.private_key_path.display()))?;
        fs::write(&self.public_key_path, public_pem)
            .with_context(|| format!("write public key {}", self.public_key_path.display()))?;
        Ok(())
    }

    /// 构建 JWKS 格式字符串
    pub fn build_jwks(&self) -> Result<String> {
        let public_pem = fs::read_to_string(&self.public_key_path)
            .with_context(|| format!("read public key {}", self.public_key_path.display()))?;
        let jwk = build_jwk_from_public_pem(&public_pem)?;
        let jwks = serde_json::json!({ "keys": [jwk] });
        Ok(serde_json::to_string(&jwks)?)
    }
}

fn compute_kid_from_public_pem(public_pem: &str) -> Result<String> {
    let public = RsaPublicKey::from_pkcs1_pem(public_pem)
        .map_err(|e| anyhow!("parse public pem failed: {}", e))?;
    let der = public
        .to_pkcs1_der()
        .map_err(|e| anyhow!("encode public der failed: {}", e))?;
    let mut hasher = Sha256::new();
    hasher.update(der.as_ref());
    let digest = hasher.finalize();
    Ok(URL_SAFE_NO_PAD.encode(digest))
}

pub fn build_jwk_from_public_pem(public_pem: &str) -> Result<serde_json::Value> {
    let public = RsaPublicKey::from_pkcs1_pem(public_pem)
        .map_err(|e| anyhow!("parse public pem failed: {}", e))?;
    let n_bytes = public.n().to_bytes_be();
    let e_bytes = public.e().to_bytes_be();
    let n = URL_SAFE_NO_PAD.encode(n_bytes);
    let e = URL_SAFE_NO_PAD.encode(e_bytes);
    let kid = compute_kid_from_public_pem(public_pem)?;
    let jwk = serde_json::json!({
        "kty": "RSA",
        "use": "sig",
        "alg": "RS256",
        "kid": kid,
        "n": n,
        "e": e
    });
    Ok(jwk)
}
