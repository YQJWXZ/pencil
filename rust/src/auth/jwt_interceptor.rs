use crate::server::grpc::auth::context::ClaimContext;
use crate::server::grpc::auth::errors::AuthError;
use crate::server::grpc::auth::jwt_manager::{JwtConfig, JwtManager};
use crate::server::grpc::auth::keys::JwtKeys;
use crate::server::grpc::auth::whitelist::default_whitelist;
use std::collections::HashSet;
use std::sync::Arc;
use tonic::{service::Interceptor, Request, Status};
use tracing::{debug, info};

#[derive(Clone)]
pub struct AuthIpInterceptor {
    pub jwt: Arc<JwtManager>,
    pub whitelist: HashSet<&'static str>,
}

impl AuthIpInterceptor {
    pub fn new(jwt: Arc<JwtManager>) -> Self {
        Self {
            jwt,
            whitelist: default_whitelist(),
        }
    }

    pub fn from_jwt_config(config: JwtConfig, keys: JwtKeys) -> Self {
        let manager = JwtManager::new(config, keys);
        Self::new(Arc::new(manager))
    }
}

impl Interceptor for AuthIpInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(peer) = request.remote_addr() {
            let client = format!("{}:{}", peer.ip(), peer.port());
            let metadata = request.metadata_mut();
            metadata.insert("x-client", client.parse().unwrap());
            let method_str = metadata
                .get("method")
                .and_then(|m| m.to_str().ok())
                .unwrap_or("unknown");
            let source_str = metadata
                .get("source")
                .and_then(|s| s.to_str().ok())
                .unwrap_or("unknown");
            if method_str == "ping" || method_str == "heartbeat" {
                debug!(target: "grpc", "{} call {} from  {}", source_str, method_str, client);
            } else {
                info!(target: "grpc", "{} call {} from {}", source_str, method_str, client);
            }
        }

        let method = request
            .metadata()
            .get("method")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if self.whitelist.contains(method) {
            return Ok(request);
        }

        let auth_header = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::MissingAuthorization)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidAuthorizationFormat)?;

        let claims = self
            .jwt
            .validate_access(token)
            .map_err(|e| AuthError::ValidationFailed(e.to_string()))?;

        let context = ClaimContext::new(
            claims.sub,
            claims.name,
            claims.role,
            claims.acl,
            claims.jti,
            claims.iat,
            claims.exp,
        );
        request.extensions_mut().insert(context);
        Ok(request)
    }
}
