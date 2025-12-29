pub mod claim;
pub mod credential;
pub mod jwt_manager;
pub mod key;
pub mod key_manager;

pub use claim::{AccessClaims, RefreshClaims};
pub use credential::{PasswordHashAlgorithm, hash_password, verify_password};
pub use jwt_manager::{JwtConfig, JwtManager};
pub use key::JwtKeys;
pub use key_manager::RsaKeyManager;

use jsonwebtoken::Algorithm;
use serde_json::Value;
use std::str::FromStr;

// Helper function to build JwtManager from JSON config
pub fn build_jwt_manager(jwt_cfg: &Value) -> JwtManager {
    let issuer = jwt_cfg
        .get("issuer")
        .and_then(|v| v.as_str())
        .unwrap_or("tmplinker")
        .to_string();
    let audience = jwt_cfg
        .get("audience")
        .and_then(|v| v.as_str())
        .unwrap_or("tmplinker-clients")
        .to_string();
    let access_ttl_secs = jwt_cfg
        .get("access_ttl_secs")
        .and_then(|n| n.as_i64())
        .unwrap_or(15 * 60);
    let refresh_ttl_secs = jwt_cfg
        .get("refresh_ttl_secs")
        .and_then(|n| n.as_i64())
        .unwrap_or(7 * 24 * 3600);
    let skew_secs = jwt_cfg
        .get("skew_secs")
        .and_then(|n| n.as_i64())
        .unwrap_or(60);
    let algorithm = match jwt_cfg
        .get("alg")
        .and_then(|s| s.as_str())
        .unwrap_or("HS256")
    {
        "HS256" => Algorithm::HS256,
        "RS256" => Algorithm::RS256,
        other => {
            tracing::info!("Unsupported jwt alg {}, fallback to HS256", other);
            Algorithm::HS256
        }
    };
    let keys = match algorithm {
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            let secret = jwt_cfg
                .get("secret")
                .and_then(|s| s.as_str())
                .unwrap_or("mattlinker-default-secret")
                .to_string();
            JwtKeys::Hs256 { secret }
        }
        Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
            let km = crate::auth::key_manager::RsaKeyManager::from_config(jwt_cfg)
                .expect("invalid jwt RS256 config");
            km.ensure_keys().expect("failed to ensure RSA keys")
        }
        _ => {
            let secret = jwt_cfg
                .get("secret")
                .and_then(|s| s.as_str())
                .unwrap_or("mattlinker-default-secret")
                .to_string();
            JwtKeys::Hs256 { secret }
        }
    };

    let config = JwtConfig {
        issuer,
        audience,
        access_ttl_secs,
        refresh_ttl_secs,
        skew_secs,
        algorithm,
    };
    JwtManager::new(config, keys)
}

pub fn password_hash_algorithm_from_value(security_cfg: &Value) -> PasswordHashAlgorithm {
    let hash: &str = security_cfg
        .get("password")
        .and_then(|v| v.get("hash"))
        .and_then(|v| v.as_str())
        .unwrap_or("plain");
    PasswordHashAlgorithm::from_str(hash).unwrap()
}

pub fn build_jwt_manager_from_value(jwt_cfg: &Value) -> JwtManager {
    use jsonwebtoken::Algorithm;
    let issuer = jwt_cfg
        .get("issuer")
        .and_then(|v| v.as_str())
        .unwrap_or("linker")
        .to_string();
    let audience = jwt_cfg
        .get("audience")
        .and_then(|v| v.as_str())
        .unwrap_or("normal-user")
        .to_string();
    let access_ttl_secs = jwt_cfg
        .get("access_ttl_secs")
        .and_then(|n| n.as_i64())
        .unwrap_or(15 * 60);
    let refresh_ttl_secs = jwt_cfg
        .get("refresh_ttl_secs")
        .and_then(|n| n.as_i64())
        .unwrap_or(7 * 24 * 3600);
    let skew_secs = jwt_cfg
        .get("skew_secs")
        .and_then(|n| n.as_i64())
        .unwrap_or(60);
    let algorithm = match jwt_cfg
        .get("alg")
        .and_then(|s| s.as_str())
        .unwrap_or("HS256")
    {
        "HS256" => Algorithm::HS256,
        "RS256" => Algorithm::RS256,
        other => {
            tracing::info!("Unsupported jwt alg {}, fallback to HS256", other);
            Algorithm::HS256
        }
    };
    let keys = match algorithm {
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            let secret = jwt_cfg
                .get("secret")
                .and_then(|s| s.as_str())
                .unwrap_or("mattlinker-default-secret")
                .to_string();
            JwtKeys::Hs256 { secret }
        }
        Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
            let km = crate::auth::key_manager::RsaKeyManager::from_config(jwt_cfg)
                .expect("invalid jwt RS256 config");
            km.ensure_keys().expect("failed to ensure RSA keys")
        }
        _ => {
            let secret = jwt_cfg
                .get("secret")
                .and_then(|s| s.as_str())
                .unwrap_or("mattlinker-default-secret")
                .to_string();
            JwtKeys::Hs256 { secret }
        }
    };

    let config = JwtConfig {
        issuer,
        audience,
        access_ttl_secs,
        refresh_ttl_secs,
        skew_secs,
        algorithm,
    };
    JwtManager::new(config, keys)
}
