# 配置、日志与测试

---

## 第一部分：配置管理

```toml
[dependencies]
dotenvy = "0.15"
config  = "0.14"
serde   = { version = "1", features = ["derive"] }
```

### 1.1 环境变量与 .env 文件

```bash
# .env（开发环境，不提交 git）
DATABASE_URL=postgres://user:pass@localhost:5432/mydb
APP_PORT=3000
APP_JWT_SECRET=supersecretkey
APP_LOG_LEVEL=debug
```

```rust
// main.rs：最先调用，加载 .env
dotenvy::dotenv().ok();  // .env 不存在时不报错（生产环境没有 .env 文件）

// 读取单个环境变量
let db_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL 必须设置");

// 带默认值
let port: u16 = std::env::var("APP_PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("APP_PORT 必须是数字");
```

### 1.2 结构化配置（推荐方式）

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    // 数据库
    pub database_url:      String,
    pub database_max_conn: u32,

    // 服务
    pub port:              u16,
    pub host:              String,

    // 认证
    pub jwt_secret:        String,
    pub jwt_expiry_hours:  u64,

    // 日志
    pub log_level:         String,

    // 可选配置
    #[serde(default)]
    pub debug:             bool,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        // config crate：支持文件 + 环境变量，环境变量优先级更高
        let cfg = config::Config::builder()
            // 默认配置文件（所有环境共用）
            .add_source(
                config::File::with_name("config/default").required(false)
            )
            // 环境特定配置（APP_ENV=production → config/production.toml）
            .add_source(
                config::File::with_name(&format!(
                    "config/{}",
                    std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into())
                ))
                .required(false)
            )
            // 环境变量：APP_PORT → port，APP_JWT_SECRET → jwt_secret
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")        // APP_JWT_SECRET → jwt_secret
                    .ignore_empty(true)
            )
            .build()?;

        Ok(cfg.try_deserialize()?)
    }

    pub fn database_max_conn(&self) -> u32 {
        if self.database_max_conn == 0 { 10 } else { self.database_max_conn }
    }
}
```

```toml
# config/default.toml（版本控制里的默认值，不含敏感信息）
port              = 3000
host              = "0.0.0.0"
database_max_conn = 10
jwt_expiry_hours  = 24
log_level         = "info"
debug             = false
```

```toml
# config/dev.toml（开发环境覆盖）
log_level = "debug"
debug     = true
```

### 1.3 在应用中使用配置

```rust
// main.rs
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::from_env()
        .expect("配置加载失败");

    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_conn())
        .connect(&config.database_url)
        .await?;

    let state = AppState {
        db:     pool,
        config: Arc::new(config),
    };

    // 绑定地址
    let addr = format!("{}:{}", state.config.host, state.config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("服务启动: http://{addr}");

    axum::serve(listener, app.with_state(state)).await?;
    Ok(())
}
```

---

## 第二部分：日志与追踪

```toml
[dependencies]
tracing            = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tower-http         = { version = "0.5", features = ["trace"] }
```

### 2.1 初始化 tracing

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing(log_level: &str) {
    tracing_subscriber::registry()
        // 日志过滤（支持 RUST_LOG 环境变量）
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(log_level)))
        // 输出格式
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)         // 显示模块路径
                .with_thread_ids(false)
                .with_line_number(true)    // 显示行号
                // 生产环境用 JSON 格式，便于日志收集
                // .json()
        )
        .init();
}

// main.rs 中最先调用
fn main() {
    init_tracing(&config.log_level);
    // ...
}
```

### 2.2 日志级别与宏

```rust
use tracing::{trace, debug, info, warn, error, instrument};

// 五个级别（从详细到严重）：TRACE < DEBUG < INFO < WARN < ERROR

// 基本使用（支持结构化字段）
info!("服务启动成功");
info!(port = 3000, host = "0.0.0.0", "服务启动");  // 结构化字段

debug!(user_id = user.id, email = %user.email, "用户登录");
warn!(retry = attempt, max = 3, "重试请求");
error!(error = ?e, user_id = id, "查询用户失败");  // ? 用 Debug 格式

// 格式化消息（和 println! 类似，但有字段时尽量用结构化）
info!("处理了 {} 个订单", count);

// 条件日志（避免高频路径的格式化开销）
if tracing::enabled!(tracing::Level::DEBUG) {
    debug!("详细状态: {:#?}", complex_struct);
}
```

### 2.3 instrument：自动追踪函数调用

```rust
// #[instrument] 自动为函数创建 span，记录入参
#[tracing::instrument(
    skip(pool, password),          // 跳过敏感字段
    fields(user_id = %user_id),    // 额外字段
)]
async fn authenticate(
    pool: &PgPool,
    username: &str,
    password: &str,     // skip 了，不会被记录
    user_id: u64,
) -> Result<User, AppError> {
    info!("开始认证");    // 自动带 span 上下文

    let user = find_user(pool, username).await
        .map_err(|e| {
            error!(error = ?e, "查找用户失败");
            e
        })?;

    info!("认证成功");
    Ok(user)
}
```

### 2.4 axum 集成请求日志

```rust
use tower_http::trace::TraceLayer;
use tracing::Span;

let app = Router::new()
    .route("/users", get(list_users))
    .layer(
        TraceLayer::new_for_http()
            // 自定义 span：记录哪些信息
            .make_span_with(|req: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method    = %req.method(),
                    uri       = %req.uri(),
                    req_id    = %uuid::Uuid::new_v4(),
                )
            })
            // 请求开始
            .on_request(|req: &axum::http::Request<_>, _span: &Span| {
                tracing::info!("收到请求");
            })
            // 请求结束
            .on_response(|resp: &axum::http::Response<_>, latency: std::time::Duration, _span: &Span| {
                tracing::info!(
                    status  = resp.status().as_u16(),
                    latency = ?latency,
                    "请求完成"
                );
            })
    );
```

### 2.5 日志级别控制（运行时）

```bash
# 环境变量控制日志级别
RUST_LOG=info                          # 全局 info
RUST_LOG=debug                         # 全局 debug
RUST_LOG=myapp=debug,sqlx=warn         # 模块级别控制
RUST_LOG=myapp::handlers=trace         # 精确到子模块
```

---

## 第三部分：测试

### 3.1 单元测试

```rust
// 同文件底部，或 src/xxx/tests.rs

#[cfg(test)]
mod tests {
    use super::*;    // 引入当前模块的所有内容

    // 同步测试
    #[test]
    fn test_password_hash() {
        let hash = hash_password("secret123");
        assert!(verify_password("secret123", &hash));
        assert!(!verify_password("wrong", &hash));
    }

    // 异步测试（需要 tokio）
    #[tokio::test]
    async fn test_user_service() {
        let service = UserService::new_mock();
        let result = service.create_user("alice", "a@b.com", "pass123").await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "alice");
    }

    // 期望 panic
    #[test]
    #[should_panic(expected = "用户名不能为空")]
    fn test_empty_username_panics() {
        validate_username("").unwrap();
    }

    // 错误测试（推荐用 Result 而不是 panic）
    #[test]
    fn test_invalid_email() -> Result<(), Box<dyn std::error::Error>> {
        let result = validate_email("not-an-email");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "邮箱格式无效");
        Ok(())
    }
}
```

### 3.2 数据库集成测试（事务回滚隔离）

```rust
// tests/user_repo_test.rs（集成测试在 tests/ 目录）
use sqlx::PgPool;

// 用测试数据库 URL
async fn setup_test_pool() -> PgPool {
    let url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:pass@localhost/mydb_test".to_string());
    PgPool::connect(&url).await.expect("连接测试数据库失败")
}

// 每个测试用事务包裹，结束后回滚，互不影响
#[sqlx::test]   // sqlx 提供的测试宏，自动创建和清理测试数据库
async fn test_create_user(pool: PgPool) {
    let repo = UserRepository::new(pool);

    let user = repo.create("testuser", "test@example.com", "hash123")
        .await
        .expect("创建用户失败");

    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert!(user.id > 0);
}

#[sqlx::test]
async fn test_find_by_email(pool: PgPool) {
    let repo = UserRepository::new(pool.clone());

    // 先插入数据
    repo.create("findme", "findme@test.com", "hash")
        .await.unwrap();

    // 再查找
    let found = repo.find_by_email("findme@test.com").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().username, "findme");

    // 查不存在的
    let not_found = repo.find_by_email("nobody@test.com").await.unwrap();
    assert!(not_found.is_none());
}
```

### 3.3 HTTP 接口测试（axum）

```rust
// tests/api_test.rs
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use tower::ServiceExt;   // oneshot

async fn build_test_app() -> Router {
    let pool = setup_test_pool().await;
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let state = AppState { db: pool, config: Arc::new(test_config()) };
    crate::app::build_router(state)
}

#[tokio::test]
async fn test_create_user_api() {
    let app = build_test_app().await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/users")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"username":"alice","email":"a@b.com","password":"secret123"}"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 解析响应体
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    assert!(json["data"]["id"].as_u64().unwrap() > 0);
    assert_eq!(json["data"]["username"], "alice");
}

#[tokio::test]
async fn test_unauthorized_access() {
    let app = build_test_app().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/profile")
        // 不带 Authorization header
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_with_auth_token() {
    let app = build_test_app().await;
    let token = generate_test_jwt(1);   // 测试用 token

    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/profile")
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

### 3.4 Mock：用 trait 隔离依赖

```rust
// 定义 Repository trait（便于测试时 mock）
#[cfg_attr(test, mockall::automock)]
pub trait UserRepo: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, username: &str, email: &str) -> Result<User, sqlx::Error>;
}

// 测试中使用 mock
// Cargo.toml: mockall = "0.12" (仅 dev-dependencies)
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_service_user_not_found() {
        let mut mock_repo = MockUserRepo::new();

        // 设置期望：当 find_by_id(999) 被调用时返回 Ok(None)
        mock_repo
            .expect_find_by_id()
            .with(eq(999_i64))
            .times(1)
            .returning(|_| Box::pin(async { Ok(None) }));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service.get_user(999).await;

        assert!(matches!(result, Err(ServiceError::UserNotFound(999))));
    }
}
```

### 3.5 测试辅助工具

```rust
// tests/helpers.rs 或 src/test_utils.rs（只在测试时编译）
#[cfg(test)]
pub mod test_helpers {
    use super::*;

    // 测试配置
    pub fn test_config() -> AppConfig {
        AppConfig {
            database_url:      std::env::var("TEST_DATABASE_URL")
                                   .unwrap_or_else(|_| "postgres://...".into()),
            port:              0,   // 随机端口
            host:              "127.0.0.1".into(),
            jwt_secret:        "test_secret_key".into(),
            jwt_expiry_hours:  1,
            log_level:         "warn".into(),  // 测试中减少日志噪音
            ..Default::default()
        }
    }

    // 创建测试用户（fixture）
    pub async fn create_test_user(pool: &PgPool) -> User {
        sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) \
             VALUES ($1, $2, $3) RETURNING *",
            format!("testuser_{}", uuid::Uuid::new_v4().simple()),
            format!("test_{}@example.com", uuid::Uuid::new_v4().simple()),
            "test_hash",
        )
        .fetch_one(pool)
        .await
        .expect("创建测试用户失败")
    }

    // 生成测试 JWT
    pub fn generate_test_jwt(user_id: i64) -> String {
        create_jwt(user_id, "test_secret_key", 3600).unwrap()
    }
}
```

### 3.6 常用测试命令

```bash
cargo test                          # 运行所有测试
cargo test user                     # 只运行名称含 "user" 的测试
cargo test -- --nocapture           # 显示 println! 输出
cargo test -- --test-threads=1      # 串行运行（避免 DB 竞争）
cargo test --test api_test          # 只运行集成测试文件
cargo nextest run                   # nextest（更快的测试运行器）
```

---

## 速查表

```
// 配置
dotenvy::dotenv().ok()              加载 .env 文件
std::env::var("KEY")                读取环境变量 → Result<String>
config::Config::builder()...        分层配置（文件 + 环境变量）

// 日志
tracing::info!(field=val, "msg")    结构化日志
tracing::error!(error=?e, "msg")    用 Debug 格式记录错误
#[tracing::instrument(skip(...))]   函数自动追踪，跳过指定参数
RUST_LOG=module=level               环境变量控制日志级别

// 测试
#[test]                             同步测试
#[tokio::test]                      异步测试
#[sqlx::test]                       数据库测试（自动管理测试库）
app.oneshot(request).await          axum 单次请求测试，不启动服务器
#[cfg_attr(test, mockall::automock)]自动生成 mock 实现
cargo test -- --nocapture           显示测试中的打印输出
```
