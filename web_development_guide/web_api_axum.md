# Web API 开发：axum 实战

```toml
[dependencies]
axum       = "0.7"
tokio      = { version = "1", features = ["full"] }
tower      = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "compression-gzip"] }
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

## 一、最小可运行示例

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",         get(root_handler))
        .route("/health",   get(health_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("监听 http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str { "Hello, World!" }
async fn health_handler() -> &'static str { "ok" }
```

---

## 二、路由组织

```rust
use axum::{routing::{get, post, put, delete, patch}, Router};

// 扁平路由
let app = Router::new()
    .route("/users",        get(list_users).post(create_user))
    .route("/users/:id",    get(get_user).put(update_user).delete(delete_user))
    .route("/users/:id/orders", get(get_user_orders));

// 嵌套路由（按模块拆分，推荐）
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

fn order_routes() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_orders).post(create_order))
        .route("/:id", get(get_order))
}

let app = Router::new()
    .nest("/api/v1/users",  user_routes())
    .nest("/api/v1/orders", order_routes());
```

---

## 三、提取器（Extractor）

提取器是 axum 从请求中拿数据的方式，**按顺序**作为 Handler 函数参数：

### 3.1 Path：路径参数

```rust
use axum::extract::Path;

// 单个参数
async fn get_user(Path(user_id): Path<u64>) -> String {
    format!("用户 {user_id}")
}

// 多个参数：用元组
async fn get_user_order(
    Path((user_id, order_id)): Path<(u64, u64)>,
) -> String {
    format!("用户{user_id}的订单{order_id}")
}

// 命名参数：用结构体（需要 Deserialize）
#[derive(Deserialize)]
struct UserPath { user_id: u64, order_id: u64 }

async fn get_detail(Path(p): Path<UserPath>) -> String {
    format!("{} {}", p.user_id, p.order_id)
}
```

### 3.2 Query：查询参数

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct ListQuery {
    page:     Option<u32>,
    per_page: Option<u32>,
    keyword:  Option<String>,
    #[serde(default)]
    active:   bool,
}

// /users?page=2&per_page=20&keyword=alice
async fn list_users(Query(q): Query<ListQuery>) -> String {
    format!("page={:?} kw={:?}", q.page, q.keyword)
}
```

### 3.3 Json：请求体

```rust
use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email:    String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse { id: u64, username: String, email: String }

async fn create_user(
    Json(req): Json<CreateUserRequest>,
) -> Json<UserResponse> {
    // Json 提取失败（格式错误、字段缺失）时自动返回 400
    Json(UserResponse { id: 1, username: req.username, email: req.email })
}
```

### 3.4 Header：请求头

```rust
use axum::extract::TypedHeader;
use axum::headers::{Authorization, Bearer};  // axum-extra crate

// Cargo.toml: axum-extra = { version = "0.9", features = ["typed-header"] }
async fn protected(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> String {
    format!("token: {}", auth.token())
}

// 或者直接取原始 header
use axum::http::HeaderMap;
async fn with_headers(headers: HeaderMap) -> String {
    let ua = headers.get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    format!("UA: {ua}")
}
```

### 3.5 State：共享应用状态

```rust
use axum::extract::State;

// 定义应用状态（必须 Clone，通常内部用 Arc）
#[derive(Clone)]
struct AppState {
    db:     sqlx::PgPool,
    config: Arc<AppConfig>,
}

// 注册状态到路由
let state = AppState { db: pool, config: Arc::new(config) };
let app = Router::new()
    .route("/users", get(list_users))
    .with_state(state);   // ← 绑定状态

// Handler 中提取
async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(users)
}
```

---

## 四、统一响应结构

```rust
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::Serialize;

// 统一 API 响应格式
#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data:    Option<T>,
    error:   Option<ErrorDetail>,
}

#[derive(Serialize)]
struct ErrorDetail {
    code:    String,
    message: String,
}

// 成功响应
fn ok<T: Serialize>(data: T) -> impl IntoResponse {
    Json(ApiResponse { success: true, data: Some(data), error: None })
}

// 失败响应
fn err(status: StatusCode, code: &str, msg: &str) -> impl IntoResponse {
    let body = Json(ApiResponse::<()> {
        success: false,
        data:    None,
        error:   Some(ErrorDetail { code: code.to_string(), message: msg.to_string() }),
    });
    (status, body)
}

// 分页响应
#[derive(Serialize)]
struct PageResponse<T> {
    items:    Vec<T>,
    page:     u32,
    per_page: u32,
    total:    u64,
}
```

---

## 五、错误处理集成

```rust
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};

// 业务错误 → HTTP 响应（参见 error_handling_in_practice.md）
#[derive(Debug)]
pub struct ApiError {
    pub status:  StatusCode,
    pub code:    &'static str,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "success": false,
            "error": {
                "code":    self.code,
                "message": self.message,
            }
        }));
        (self.status, body).into_response()
    }
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;

// Handler 使用
async fn get_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> ApiResult<UserResponse> {
    let user = state.db.find_user(id).await
        .map_err(|e| ApiError {
            status:  StatusCode::NOT_FOUND,
            code:    "USER_NOT_FOUND",
            message: format!("用户 {id} 不存在"),
        })?;

    Ok(Json(UserResponse::from(user)))
}
```

---

## 六、中间件

### 6.1 内置中间件（tower-http）

```rust
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    compression::CompressionLayer,
};

let app = Router::new()
    .route("/users", get(list_users))
    // 请求追踪日志
    .layer(TraceLayer::new_for_http())
    // 响应压缩
    .layer(CompressionLayer::new())
    // CORS
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );
```

### 6.2 自定义中间件：请求 ID

```rust
use axum::{middleware::{self, Next}, extract::Request, response::Response};
use uuid::Uuid;

async fn request_id_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    let id = Uuid::new_v4().to_string();
    req.headers_mut().insert(
        "x-request-id",
        id.parse().unwrap(),
    );
    let mut resp = next.run(req).await;
    resp.headers_mut().insert("x-request-id", id.parse().unwrap());
    resp
}

let app = Router::new()
    .route("/", get(handler))
    .layer(middleware::from_fn(request_id_middleware));
```

### 6.3 自定义中间件：JWT 认证

```rust
use axum::{middleware::Next, extract::{Request, State}};

async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = req.headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError {
            status:  StatusCode::UNAUTHORIZED,
            code:    "MISSING_TOKEN",
            message: "缺少认证令牌".into(),
        })?;

    let claims = verify_jwt(token, &state.config.jwt_secret)
        .map_err(|_| ApiError {
            status:  StatusCode::UNAUTHORIZED,
            code:    "INVALID_TOKEN",
            message: "无效的认证令牌".into(),
        })?;

    // 把解析好的 claims 传给后续 Handler
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

// 在需要认证的路由上挂载
let protected = Router::new()
    .route("/profile", get(get_profile))
    .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));
```

### 6.4 Handler 中提取 Extension

```rust
use axum::Extension;

// 从 extensions 中取出中间件注入的数据
async fn get_profile(
    Extension(claims): Extension<JwtClaims>,
    State(state): State<AppState>,
) -> ApiResult<ProfileResponse> {
    let user = state.db.find_user(claims.user_id).await
        .map_err(ApiError::from)?;
    Ok(Json(ProfileResponse::from(user)))
}
```

---

## 七、表单与文件上传

```rust
// Cargo.toml: axum = { version = "0.7", features = ["multipart"] }
use axum::extract::Multipart;

async fn upload_avatar(
    Path(user_id): Path<u64>,
    mut multipart: Multipart,
) -> ApiResult<UploadResponse> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name      = field.name().unwrap_or("").to_string();
        let file_name = field.file_name().map(|s| s.to_string());
        let data      = field.bytes().await.unwrap();

        if name == "avatar" {
            // 保存文件
            let path = format!("uploads/{user_id}.jpg");
            tokio::fs::write(&path, &data).await
                .map_err(|e| ApiError {
                    status:  StatusCode::INTERNAL_SERVER_ERROR,
                    code:    "UPLOAD_FAILED",
                    message: e.to_string(),
                })?;
        }
    }
    Ok(Json(UploadResponse { url: format!("/uploads/{user_id}.jpg") }))
}
```

---

## 八、完整的业务 Handler 示例

```rust
// POST /api/v1/users
async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> ApiResult<CreateUserResponse> {
    // 1. 参数校验
    if req.username.is_empty() {
        return Err(ApiError {
            status:  StatusCode::BAD_REQUEST,
            code:    "INVALID_USERNAME",
            message: "用户名不能为空".into(),
        });
    }

    // 2. 业务逻辑
    let user = state.user_service
        .create_user(&req.username, &req.email, &req.password)
        .await
        .map_err(ApiError::from)?;  // ServiceError → ApiError

    // 3. 返回响应
    Ok(Json(CreateUserResponse {
        id:         user.id,
        username:   user.username,
        email:      user.email,
        created_at: user.created_at.to_string(),
    }))
}
```

---

## 九、应用组织结构（推荐）

```
src/
├── main.rs              入口：启动 tokio，初始化 DB/配置，启动 axum
├── app.rs               路由组装、中间件注册
├── state.rs             AppState 定义
├── error.rs             ApiError、ApiResult 定义
│
├── handlers/            HTTP Handler（只处理 HTTP 层）
│   ├── mod.rs
│   ├── user.rs          用户相关 handler
│   └── order.rs
│
├── services/            业务逻辑层
│   ├── mod.rs
│   ├── user_service.rs
│   └── order_service.rs
│
├── repositories/        数据访问层
│   ├── mod.rs
│   └── user_repo.rs
│
├── models/              数据模型
│   ├── user.rs          User struct（DB 行）
│   └── dto.rs           CreateUserRequest / UserResponse 等 DTO
│
└── config.rs            配置结构体
```

---

## 速查表

```
// 路由方法
get / post / put / delete / patch     HTTP 动词路由
.nest("/prefix", router)              路由嵌套
.with_state(state)                    绑定共享状态
.layer(middleware)                    添加中间件

// 提取器（Handler 参数）
Path(x): Path<T>                      路径参数（:param）
Query(q): Query<T>                    查询字符串 (?key=val)
Json(body): Json<T>                   JSON 请求体（自动 400）
State(s): State<T>                    共享状态
Extension(e): Extension<T>            中间件注入的扩展数据
HeaderMap                             所有请求头

// 响应类型
impl IntoResponse                     可以直接 return 的类型
Json(data)                            JSON 响应（200）
(StatusCode, Json(data))              自定义状态码
(StatusCode, headers, body)           完全自定义

// 中间件
middleware::from_fn(fn)               函数中间件
middleware::from_fn_with_state(s, fn) 带状态的函数中间件
.route_layer(layer)                   只对本路由生效的中间件
```
