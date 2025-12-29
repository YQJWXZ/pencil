use crate::auth::claim::{AccessClaims, RefreshClaims};
use crate::auth::key::JwtKeys;
use crate::tools::utils::generate_uuid;
use jsonwebtoken::{Algorithm, Header, Validation, decode, encode};

#[derive(Clone, Debug)]
pub struct JwtConfig {
    pub issuer: String,
    pub audience: String,
    pub access_ttl_secs: i64,
    pub refresh_ttl_secs: i64,
    pub skew_secs: i64,
    pub algorithm: Algorithm,
}

#[derive(Clone)]
pub struct JwtManager {
    pub config: JwtConfig,
    pub keys: JwtKeys,
}

impl JwtManager {
    pub fn new(config: JwtConfig, keys: JwtKeys) -> Self {
        Self { config, keys }
    }

    pub fn sign_access(
        &self,
        user_id: &str,
        user_name: &str,
        role_name: &str,
        access_level: u32,
        now_epoch: i64,
    ) -> anyhow::Result<String> {
        let jti = generate_uuid();
        let claims = AccessClaims {
            sub: user_id.to_string(),
            name: user_name.to_string(),
            role: role_name.to_string(),
            acl: access_level,
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            iat: now_epoch,
            nbf: now_epoch,
            exp: now_epoch + self.config.access_ttl_secs,
            jti,
        };

        let mut header = Header::new(self.config.algorithm);
        if let JwtKeys::Rs256 { kid, .. } = &self.keys
            && let Some(k) = kid.clone()
        {
            header.kid = Some(k);
        }

        let token = encode(&header, &claims, &self.keys.encoding_key())?;
        Ok(token)
    }

    pub fn validate_access(&self, token: &str) -> anyhow::Result<AccessClaims> {
        let mut validation = Validation::new(self.config.algorithm);
        validation.set_audience(std::slice::from_ref(&self.config.issuer));
        validation.set_issuer(std::slice::from_ref(&self.config.issuer));
        validation.leeway = self.config.skew_secs as u64;

        let data = decode::<AccessClaims>(token, &self.keys.decoding_key(), &validation)?;
        Ok(data.claims)
    }

    pub fn validate_refresh(&self, token: &str) -> anyhow::Result<RefreshClaims> {
        let mut validation = Validation::new(self.config.algorithm);
        validation.set_audience(std::slice::from_ref(&self.config.issuer));
        validation.set_issuer(std::slice::from_ref(&self.config.issuer));
        validation.leeway = self.config.skew_secs as u64;

        let data = decode::<RefreshClaims>(token, &self.keys.decoding_key(), &validation)?;
        Ok(data.claims)
    }

    pub fn sign_refresh(
        &self,
        user_id: &str,
        user_name: &str,
        role_name: &str,
        access_level: u32,
        now_epoch: i64,
    ) -> anyhow::Result<String> {
        let jti = format!("r-{}", generate_uuid());
        let claims = RefreshClaims {
            sub: user_id.to_string(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            iat: now_epoch,
            nbf: now_epoch,
            exp: now_epoch + self.config.refresh_ttl_secs,
            jti,
            typ: "refresh".to_string(),
            name: user_name.to_string(),
            role: role_name.to_string(),
            acl: access_level,
        };
        let header = Header::new(self.config.algorithm);
        let token = encode(&header, &claims, &self.keys.encoding_key())?;
        Ok(token)
    }
}
