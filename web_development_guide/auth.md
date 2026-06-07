# 认证体系：JWT + 密码哈希 + RBAC

```toml
[dependencies]
jsonwebtoken = "9"
argon2       = "0.5"
axum         = "0.7"
serde        = { version = "1", features = ["derive"] }
tokio        = { version = "1", features = ["full"] }
uuid         = { version = "1", features = ["v4"] }
chrono       = { version = "0.4", features = ["serde"] }
thiserror    = "1"
```

---

## 一、密码哈希（argon2）

```rust
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

/// 对密码进行哈希（注册时调用）
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

/// 验证密码（登录时调用）
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "my_secure_password_123";
        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
        // 相同密码每次哈希结果不同（salt 不同）
        assert_ne!(hash, hash_password(password).unwrap());
    }
}
```

---

## 二、JWT：签发与验证

```rust
use jsonwebtoken::{
    decode, encode,
    errors::{Error as JwtError, ErrorKind},
    DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

/// Access Token Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:   String,    // subject：用户 ID
    pub email: String,
    pub role:  Role,
    pub exp:   i64,       // 过期时间（Unix 时间戳）
    pub iat:   i64,       // 签发时间
    pub jti:   String,    // JWT ID（唯一，防重放）
}

/// Refresh Token Claims（信息更少，有效期更长）
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,      // 用户 ID
    pub jti: String,      // 对应 access token 的 jti，可用于吊销
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role { Admin, User, Guest }

pub struct JwtConfig {
    pub access_secret:  String,
    pub refresh_secret: String,
    pub access_ttl:     i64,   // 秒，通常 15 分钟 = 900
    pub refresh_ttl:    i64,   // 秒，通常 7 天 = 604800
}

/// 签发 access token
pub fn issue_access_token(
    user_id: &str,
    email:   &str,
    role:    Role,
    cfg:     &JwtConfig,
) -> Result<String, JwtError> {
    let now = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub:   user_id.to_string(),
        email: email.to_string(),
        role,
        iat:   now,
        exp:   now + cfg.access_ttl,
        jti:   uuid::Uuid::new_v4().to_string(),
    };
    encode(
        &Header::default(),                               // 默认 HS256
        &claims,
        &EncodingKey::from_secret(cfg.access_secret.as_bytes()),
    )
}

/// 签发 refresh token
pub fn issue_refresh_token(
    user_id: &str,
    cfg:     &JwtConfig,
) -> Result<String, JwtError> {
    let now = chrono::Utc::now().timestamp();
    let claims = RefreshClaims {
        sub: user_id.to_string(),
        jti: uuid::Uuid::new_v4().to_string(),
        iat: now,
        exp: now + cfg.refresh_ttl,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.refresh_secret.as_bytes()),
    )
}

/// 验证 access token，返回 Claims
pub fn verify_access_token(
    token: &str,
    cfg:   &JwtConfig,
) -> Result<Claims, JwtError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;   // 检查过期时间（默认已开启）

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.access_secret.as_bytes()),
        &validation,
    )?;
    Ok(data.claims)
}

/// 区分过期和无效两种错误
pub fn verify_access_token_detailed(
    token: &str,
    cfg:   &JwtConfig,
) -> Result<Claims, AuthError> {
    verify_access_token(token, cfg).map_err(|e| match e.kind() {
        ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _                          => AuthError::InvalidToken,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("token 已过期")]       TokenExpired,
    #[error("token 无效")]         InvalidToken,
    #[error("缺少认证信息")]       MissingCredentials,
    #[error("权限不足")]           Forbidden,
    #[error("密码错误")]           WrongPassword,
    #[error("用户不存在")]         UserNotFound,
}
```

---

## 三、axum 认证中间件

```rust
use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response, Json},
};
use std::sync::Arc;

/// 从 Authorization header 中提取并验证 token
pub async fn auth_middleware(
    State(cfg): State<Arc<JwtConfig>>,
    mut req:    Request,
    next:       Next,
) -> Response {
    // 取 Authorization: Bearer <token>
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t,
        None    => return auth_error(StatusCode::UNAUTHORIZED, "MISSING_TOKEN", "缺少认证令牌"),
    };

    match verify_access_token(token, &cfg) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);  // 注入到 extensions
            next.run(req).await
        }
        Err(e) => match e.kind() {
            ErrorKind::ExpiredSignature =>
                auth_error(StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED", "令牌已过期"),
            _ =>
                auth_error(StatusCode::UNAUTHORIZED, "INVALID_TOKEN", "无效的令牌"),
        }
    }
}

fn auth_error(status: StatusCode, code: &str, msg: &str) -> Response {
    (status, Json(serde_json::json!({
        "success": false,
        "error": { "code": code, "message": msg }
    }))).into_response()
}

/// 从 extension 中提取已验证的 Claims（用于 Handler）
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[axum::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Claims {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts.extensions
            .get::<Claims>()
            .cloned()
            .ok_or_else(|| auth_error(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "未认证"))
    }
}
```

### 路由上挂载中间件

```rust
use axum::{Router, routing::{get, post}, middleware};

pub fn build_router(cfg: Arc<JwtConfig>) -> Router {
    // 公开路由（无需认证）
    let public = Router::new()
        .route("/auth/login",   post(login_handler))
        .route("/auth/refresh", post(refresh_handler))
        .route("/auth/register", post(register_handler));

    // 受保护路由（需要认证）
    let protected = Router::new()
        .route("/me",           get(me_handler))
        .route("/users",        get(list_users_handler))
        .route_layer(middleware::from_fn_with_state(
            cfg.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public)
        .merge(protected)
}

/// Handler 中直接使用提取的 Claims
async fn me_handler(claims: Claims) -> impl IntoResponse {
    Json(serde_json::json!({
        "user_id": claims.sub,
        "email":   claims.email,
        "role":    claims.role,
    }))
}
```

---

## 四、登录完整流程

```rust
use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    email:    String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token:  String,
    refresh_token: String,
    expires_in:    i64,     // access token 有效秒数
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(req):    Json<LoginRequest>,
) -> Result<Json<LoginResponse>, impl IntoResponse> {
    // 1. 查用户
    let user = state.user_repo
        .find_by_email(&req.email)
        .await
        .map_err(|_| auth_error(StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS", "邮箱或密码错误"))?
        .ok_or_else(|| auth_error(StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS", "邮箱或密码错误"))?;

    // 2. 验证密码
    if !verify_password(&req.password, &user.password_hash)
        .map_err(|_| auth_error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "服务内部错误"))?
    {
        return Err(auth_error(StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS", "邮箱或密码错误"));
    }

    // 3. 签发 token
    let access_token = issue_access_token(
        &user.id.to_string(),
        &user.email,
        user.role.clone(),
        &state.jwt_cfg,
    ).map_err(|_| auth_error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "签发令牌失败"))?;

    let refresh_token = issue_refresh_token(
        &user.id.to_string(),
        &state.jwt_cfg,
    ).map_err(|_| auth_error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "签发令牌失败"))?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        expires_in: state.jwt_cfg.access_ttl,
    }))
}
```

---

## 五、RBAC 权限控制

```rust
/// 角色权限守卫：只允许特定角色访问
pub fn require_role(allowed: Vec<Role>) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Clone {
    move |req: Request, next: Next| {
        let allowed = allowed.clone();
        Box::pin(async move {
            let claims = req.extensions().get::<Claims>().cloned();
            match claims {
                Some(c) if allowed.contains(&c.role) => next.run(req).await,
                Some(_) => auth_error(StatusCode::FORBIDDEN, "FORBIDDEN", "权限不足"),
                None    => auth_error(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "未认证"),
            }
        })
    }
}

// 在路由上使用
let admin_routes = Router::new()
    .route("/admin/users", get(admin_list_users))
    .route_layer(middleware::from_fn(require_role(vec![Role::Admin])));
```

---

## 六、Refresh Token 流程

```rust
#[derive(Deserialize)]
pub struct RefreshRequest { refresh_token: String }

pub async fn refresh_handler(
    State(state): State<Arc<AppState>>,
    Json(req):    Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, Response> {
    // 1. 验证 refresh token
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let data = decode::<RefreshClaims>(
        &req.refresh_token,
        &DecodingKey::from_secret(state.jwt_cfg.refresh_secret.as_bytes()),
        &validation,
    ).map_err(|_| auth_error(StatusCode::UNAUTHORIZED, "INVALID_REFRESH_TOKEN", "刷新令牌无效"))?;

    let user_id = &data.claims.sub;

    // 2. 查用户（确认账号仍然有效）
    let user = state.user_repo
        .find_by_id(user_id.parse().unwrap())
        .await
        .ok()
        .flatten()
        .ok_or_else(|| auth_error(StatusCode::UNAUTHORIZED, "USER_NOT_FOUND", "用户不存在"))?;

    // 3. 签发新 access token
    let access_token = issue_access_token(
        user_id, &user.email, user.role.clone(), &state.jwt_cfg
    ).map_err(|_| auth_error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "签发失败"))?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token: req.refresh_token,  // refresh token 本身不轮换（也可以轮换）
        expires_in:    state.jwt_cfg.access_ttl,
    }))
}
```

---

## 速查表

```
argon2：
  SaltString::generate(&mut OsRng)        生成随机 salt
  Argon2::default().hash_password(pw, salt)   哈希密码
  Argon2::default().verify_password(pw, hash) 验证密码

jsonwebtoken：
  encode(&Header::default(), &claims, &EncodingKey::from_secret(s))  签发
  decode::<Claims>(token, &DecodingKey::from_secret(s), &Validation::default())  验证
  e.kind() == ErrorKind::ExpiredSignature  区分过期 vs 无效

axum 中间件：
  req.extensions_mut().insert(claims)      注入到 request
  req.extensions().get::<Claims>()         后续 handler 取出
  impl FromRequestParts for Claims         实现自定义提取器
  .route_layer(middleware::from_fn(...))   只对本路由组生效
```
