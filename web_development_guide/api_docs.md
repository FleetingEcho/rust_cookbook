# API 文档自动生成：utoipa + Swagger UI

```toml
[dependencies]
utoipa             = { version = "4", features = ["axum_extras", "chrono", "uuid"] }
utoipa-swagger-ui  = { version = "7", features = ["axum"] }
axum               = "0.7"
serde              = { version = "1", features = ["derive"] }
```

---

## 一、为数据结构添加 Schema

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 用户信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    /// 用户 ID
    pub id:         i64,
    /// 用户名（3-50 个字符）
    pub username:   String,
    /// 邮箱地址
    pub email:      String,
    /// 账号是否激活
    pub active:     bool,
    /// 注册时间（ISO 8601）
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 创建用户请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    /// 用户名，3-50 个字符，只能包含字母数字和下划线
    #[schema(example = "alice_123")]
    pub username: String,

    /// 邮箱地址
    #[schema(example = "alice@example.com")]
    pub email:    String,

    /// 密码，至少 8 位，包含大小写字母和数字
    #[schema(example = "Secure@Pass123")]
    pub password: String,
}

/// 分页响应包装
#[derive(Serialize, ToSchema)]
#[aliases(
    UserPageResponse  = PageResponse<UserResponse>,   // 为常用泛型生成别名
    OrderPageResponse = PageResponse<OrderResponse>,
)]
pub struct PageResponse<T: ToSchema> {
    /// 当前页数据
    pub items:    Vec<T>,
    /// 当前页码（从 1 开始）
    pub page:     u32,
    /// 每页条数
    pub per_page: u32,
    /// 总条数
    pub total:    u64,
}

/// 统一错误响应
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub success: bool,
    pub error:   ErrorDetail,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorDetail {
    /// 机器可读的错误码
    #[schema(example = "USER_NOT_FOUND")]
    pub code:    String,
    /// 人类可读的错误信息
    #[schema(example = "用户不存在")]
    pub message: String,
}
```

---

## 二、为 Handler 添加 OpenAPI 注解

```rust
use axum::{extract::{Path, Query, State}, response::Json};
use utoipa::{self, OpenApi};
use std::sync::Arc;

/// 获取用户详情
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = i64, Path, description = "用户 ID", example = 1)
    ),
    responses(
        (status = 200, description = "获取成功", body = UserResponse),
        (status = 404, description = "用户不存在", body = ErrorResponse),
        (status = 401, description = "未认证",     body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])   // 需要认证
    ),
    tag = "users"
)]
pub async fn get_user_handler(
    Path(id):     Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserResponse>, Json<ErrorResponse>> {
    todo!()
}

/// 创建用户
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "创建成功",       body = UserResponse),
        (status = 400, description = "参数错误",       body = ErrorResponse),
        (status = 409, description = "邮箱已被注册",   body = ErrorResponse),
        (status = 422, description = "校验失败",       body = ErrorResponse),
    ),
    tag = "users"
)]
pub async fn create_user_handler(
    State(state): State<Arc<AppState>>,
    Json(req):    Json<CreateUserRequest>,
) -> Result<(axum::http::StatusCode, Json<UserResponse>), Json<ErrorResponse>> {
    todo!()
}

/// 用户列表（分页）
#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(
        ("page"     = Option<u32>, Query, description = "页码（默认 1）",    example = 1),
        ("per_page" = Option<u32>, Query, description = "每页条数（默认 20）", example = 20),
        ("keyword"  = Option<String>, Query, description = "搜索关键字"),
    ),
    responses(
        (status = 200, description = "获取成功", body = UserPageResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "users"
)]
pub async fn list_users_handler() -> Json<PageResponse<UserResponse>> {
    todo!()
}

/// 删除用户
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    params(
        ("id" = i64, Path, description = "用户 ID")
    ),
    responses(
        (status = 204, description = "删除成功"),
        (status = 404, description = "用户不存在", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "users"
)]
pub async fn delete_user_handler(Path(_id): Path<i64>) -> axum::http::StatusCode {
    axum::http::StatusCode::NO_CONTENT
}
```

---

## 三、定义 OpenAPI 文档结构

```rust
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    // 注册所有 handler
    paths(
        get_user_handler,
        create_user_handler,
        list_users_handler,
        delete_user_handler,
        // 认证相关
        login_handler,
        refresh_token_handler,
        // 订单相关
        create_order_handler,
        get_order_handler,
    ),

    // 注册所有数据结构
    components(
        schemas(
            UserResponse,
            CreateUserRequest,
            PageResponse<UserResponse>,
            ErrorResponse,
            ErrorDetail,
            LoginRequest,
            LoginResponse,
        )
    ),

    // 全局标签（用于分组 API）
    tags(
        (name = "auth",   description = "认证与授权"),
        (name = "users",  description = "用户管理"),
        (name = "orders", description = "订单管理"),
    ),

    // 全局安全方案
    modifiers(&SecurityAddon),

    // 文档基本信息
    info(
        title       = "My API",
        version     = "1.0.0",
        description = "完整的 API 文档",
        contact(
            name  = "开发团队",
            email = "dev@example.com"
        )
    ),

    // 服务器列表
    servers(
        (url = "http://localhost:3000", description = "本地开发"),
        (url = "https://api.example.com", description = "生产环境"),
    )
)]
pub struct ApiDoc;

/// 添加 Bearer Token 安全方案
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build()
                ),
            );
        }
    }
}
```

---

## 四、注册 Swagger UI 路由

```rust
use utoipa_swagger_ui::SwaggerUi;
use axum::Router;

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        // 业务路由
        .nest("/api/v1", api_routes(state.clone()))

        // Swagger UI：访问 /docs
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )

        .with_state(state)
}
```

访问 `http://localhost:3000/docs` 即可看到交互式 API 文档。

---

## 五、枚举类型的 Schema

```rust
/// 订单状态
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// 待支付
    Pending,
    /// 已支付
    Paid,
    /// 已发货
    Shipped,
    /// 已完成
    Delivered,
    /// 已取消
    Cancelled,
}

/// 通知类型（携带不同数据）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotificationType {
    Email { recipient: String },
    Sms   { phone:     String },
    Push  { device_id: String },
}
```

---

## 六、为整个模块批量注册

```rust
// handlers/user.rs
pub mod user_handlers {
    use super::*;

    /// 所有用户相关的 path 函数
    pub fn paths() -> utoipa::openapi::path::Paths {
        // 用 ApiDoc 局部生成
        use utoipa::OpenApi;
        #[derive(OpenApi)]
        #[openapi(paths(get_user_handler, create_user_handler, list_users_handler))]
        struct UserApiDoc;
        UserApiDoc::openapi().paths
    }
}

// 主 ApiDoc 中合并
// （utoipa 4.x 通过 nest 功能合并多个子文档）
```

---

## 七、在响应中描述 Header

```rust
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (
            status = 200,
            description = "登录成功",
            body = LoginResponse,
            headers(
                ("x-request-id" = String, description = "请求 ID"),
                ("x-ratelimit-remaining" = i32, description = "剩余请求次数"),
            )
        ),
        (status = 401, description = "认证失败", body = ErrorResponse),
    ),
    tag = "auth"
)]
pub async fn login_handler(Json(req): Json<LoginRequest>) -> Json<LoginResponse> {
    todo!()
}
```

---

## 速查表

```
依赖：
  utoipa = { version = "4", features = ["axum_extras"] }
  utoipa-swagger-ui = { version = "7", features = ["axum"] }

数据结构：
  #[derive(ToSchema)]                          标记可生成 Schema
  #[schema(example = "value")]                 指定示例值
  /// 注释                                     作为字段描述

Handler 注解：
  #[utoipa::path(get/post/put/delete/patch, path="...", ...)]
  params(("name" = Type, Path/Query, description = "..."))
  request_body = RequestType                   请求体
  responses((status=200, description="", body=ResponseType))
  security(("bearer_auth" = []))               需要认证
  tag = "group_name"                           分组

OpenAPI 文档：
  #[derive(OpenApi)]
  #[openapi(paths(...), components(schemas(...)), tags(...), modifiers(...))]
  struct ApiDoc;

安全方案：
  impl utoipa::Modify for SecurityAddon        添加 Bearer/API Key 方案

Swagger UI：
  SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi())
  访问 http://localhost:3000/docs             交互式文档页面
```
