use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,  // user_id
    pub name: String, // user_name
    pub role: String, // role (e.g., Admin)
    pub acl: u32,     // access level bitmap
    pub iss: String,  // issuer
    pub aud: String,  // audience
    pub iat: i64,     // issued at (epoch seconds)
    pub nbf: i64,     // not before (epoch seconds)
    pub exp: i64,     // expires at (epoch seconds)
    pub jti: String,  // token id
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String, // user_id
    pub iss: String,
    pub aud: String,
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub jti: String,
    pub typ: String,  // "refresh"
    pub name: String, // user_name (for re-issue without store)
    pub role: String, // role string
    pub acl: u32,     // access level
}
