#[derive(Debug, Clone)]
pub struct ClaimContext {
    pub user_id: String,
    pub user_name: String,
    pub role: String,
    pub access_level: u32,
    pub jti: String,
    pub issued_at: i64,
    pub expires_at: i64,
}

impl ClaimContext {
    pub fn new(
        user_id: String,
        user_name: String,
        role: String,
        access_level: u32,
        jti: String,
        issued_at: i64,
        expires_at: i64,
    ) -> Self {
        Self {
            user_id,
            user_name,
            role,
            access_level,
            jti,
            issued_at,
            expires_at,
        }
    }
}
