use crate::auth::{
    credential::{PasswordHashAlgorithm, hash_password, verify_password},
    jwt_manager::JwtManager,
};
use crate::db::Pool;
use crate::repositories::user::UserRepository;
use crate::tools::proto::user::{
    LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, VerifyTokenRequest,
    VerifyTokenResponse,
};
use redis::AsyncCommands;
use tonic::{Request, Response, Status};

pub struct UserServiceImpl {
    user_repo: UserRepository,
    jwt_manager: JwtManager,
    redis_client: redis::Client,
    password_algo: PasswordHashAlgorithm,
}

impl UserServiceImpl {
    pub fn new(
        pool: Pool,
        jwt_manager: JwtManager,
        redis_client: redis::Client,
        password_algo: PasswordHashAlgorithm,
    ) -> Self {
        Self {
            user_repo: UserRepository::new(pool),
            jwt_manager,
            redis_client,
            password_algo,
        }
    }

    /// Cache access token in Redis
    async fn cache_access_token(&self, token: &str, user_id: i64, ttl: i64) -> Result<(), Status> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Status::internal(format!("Redis connection failed: {}", e)))?;

        let key = format!("access:{}", token);
        let _: () = conn
            .set_ex(&key, user_id, ttl as u64)
            .await
            .map_err(|e| Status::internal(format!("Failed to cache access token: {}", e)))?;

        Ok(())
    }

    /// Cache refresh token in Redis
    async fn cache_refresh_token(&self, token: &str, user_id: i64, ttl: i64) -> Result<(), Status> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Status::internal(format!("Redis connection failed: {}", e)))?;

        let key = format!("refresh:{}", token);
        let _: () = conn
            .set_ex(&key, user_id, ttl as u64)
            .await
            .map_err(|e| Status::internal(format!("Failed to cache refresh token: {}", e)))?;

        Ok(())
    }

    /// Check if access token exists in Redis
    async fn check_access_token(&self, token: &str) -> Result<bool, Status> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Status::internal(format!("Redis connection failed: {}", e)))?;

        let key = format!("access:{}", token);
        let exists: bool = conn
            .exists(&key)
            .await
            .map_err(|e| Status::internal(format!("Failed to check token: {}", e)))?;

        Ok(exists)
    }
}

#[tonic::async_trait]
impl crate::tools::proto::user::user_service_server::UserService for UserServiceImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();

        // Check if username exists
        if self
            .user_repo
            .find_by_username(&req.username)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?
            .is_some()
        {
            return Err(Status::already_exists("Username already exists"));
        }

        // Check if email exists
        if self
            .user_repo
            .find_by_email(&req.email)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?
            .is_some()
        {
            return Err(Status::already_exists("Email already exists"));
        }

        // Hash password
        let password_hash = hash_password(&req.password, self.password_algo);

        // Create user (default role: user, access_level: 0)
        let user = self
            .user_repo
            .create(&req.username, &req.email, &password_hash, "user")
            .await
            .map_err(|e| Status::internal(format!("Failed to create user: {}", e)))?;

        // Generate access token
        let now = chrono::Utc::now().timestamp();
        let access_token = self
            .jwt_manager
            .sign_access(&user.id.to_string(), &user.username, &user.role, 0, now)
            .map_err(|e| Status::internal(format!("Failed to generate access token: {}", e)))?;

        // Cache access token
        self.cache_access_token(
            &access_token,
            user.id,
            self.jwt_manager.config.access_ttl_secs,
        )
        .await?;

        Ok(Response::new(RegisterResponse {
            user: Some(user.into()),
            token: access_token,
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        // Find user by username
        let user = self
            .user_repo
            .find_by_username(&req.username)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?
            .ok_or_else(|| Status::not_found("User not found"))?;

        // Verify password
        let password_valid =
            verify_password(&req.password, &user.password_hash, self.password_algo);

        if !password_valid {
            return Err(Status::unauthenticated("Invalid password"));
        }

        // Generate access and refresh tokens
        let now = chrono::Utc::now().timestamp();
        let access_token = self
            .jwt_manager
            .sign_access(&user.id.to_string(), &user.username, &user.role, 0, now)
            .map_err(|e| Status::internal(format!("Failed to generate access token: {}", e)))?;

        let refresh_token = self
            .jwt_manager
            .sign_refresh(&user.id.to_string(), &user.username, &user.role, 0, now)
            .map_err(|e| Status::internal(format!("Failed to generate refresh token: {}", e)))?;

        // Cache tokens in Redis
        self.cache_access_token(
            &access_token,
            user.id,
            self.jwt_manager.config.access_ttl_secs,
        )
        .await?;
        self.cache_refresh_token(
            &refresh_token,
            user.id,
            self.jwt_manager.config.refresh_ttl_secs,
        )
        .await?;

        Ok(Response::new(LoginResponse {
            user: Some(user.into()),
            token: access_token,
            expires_in: self.jwt_manager.config.access_ttl_secs,
        }))
    }

    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenResponse>, Status> {
        let req = request.into_inner();

        // Check if token exists in Redis
        let exists = self.check_access_token(&req.token).await?;
        if !exists {
            return Ok(Response::new(VerifyTokenResponse {
                valid: false,
                user: None,
            }));
        }

        // Validate JWT token
        let claims = match self.jwt_manager.validate_access(&req.token) {
            Ok(claims) => claims,
            Err(_) => {
                return Ok(Response::new(VerifyTokenResponse {
                    valid: false,
                    user: None,
                }));
            }
        };

        // Find user by ID from claims
        let user_id: i64 = claims
            .sub
            .parse()
            .map_err(|_| Status::internal("Invalid user ID in token"))?;

        let user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        match user {
            Some(user) => Ok(Response::new(VerifyTokenResponse {
                valid: true,
                user: Some(user.into()),
            })),
            None => Ok(Response::new(VerifyTokenResponse {
                valid: false,
                user: None,
            })),
        }
    }
}
