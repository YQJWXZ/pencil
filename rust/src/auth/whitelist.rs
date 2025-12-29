use std::collections::HashSet;

/// gRPC methods that don't require authentication
pub fn default_whitelist() -> HashSet<&'static str> {
    HashSet::from([
        "/user.UserService/Register",
        "/user.UserService/Login",
        "/user.UserService/RefreshToken",
    ])
}

/// Short method names (without package prefix) for convenience
pub fn short_whitelist() -> HashSet<&'static str> {
    HashSet::from([
        "Register",
        "Login",
        "RefreshToken",
    ])
}
