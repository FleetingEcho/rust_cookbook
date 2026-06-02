# 错误处理实战指南

> 业务应用的错误处理不只是"不 panic"，而是要做到：错误可区分、可转换、有上下文、能正确响应给客户端。

```toml
[dependencies]
thiserror = "1"
anyhow    = "1"
```

---

## 一、两个核心库的定位

```
thiserror → 定义"有结构"的错误类型（库代码、领域层、服务层）
            调用方可以 match 错误的具体变体

anyhow    → 在"顶层"收集任意错误（应用入口、main、glue code）
            不需要 match 具体错误，只需传播和打印
```

**分层原则：**
```
Domain / Repository 层  → thiserror 定义精确错误
Service 层             → thiserror（或透传 domain 错误）
Handler / main 层      → anyhow 收口，或转为 HTTP 响应
```

---

## 二、thiserror：定义结构化错误

```rust
use thiserror::Error;

// 定义错误枚举
#[derive(Error, Debug)]
pub enum AppError {
    // #[error("...")] 是 Display 实现，{0} 引用第一个字段，{field} 引用具名字段
    #[error("用户不存在: id={id}")]
    UserNotFound { id: u64 },

    #[error("邮箱已存在: {0}")]
    EmailAlreadyExists(String),

    #[error("密码错误")]
    InvalidPassword,

    // #[from] 自动生成 From<io::Error> for AppError
    // 让 ? 可以自动从 io::Error 转换
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("参数错误: {0}")]
    BadRequest(String),

    // 透传其他错误（包裹原始错误）
    #[error("内部错误: {0}")]
    Internal(#[from] anyhow::Error),
}

// 使用
fn find_user(id: u64) -> Result<User, AppError> {
    if id == 0 {
        return Err(AppError::UserNotFound { id });
    }
    Ok(User { id, name: "Alice".into() })
}

// ? 自动转换
fn read_config() -> Result<Config, AppError> {
    let content = std::fs::read_to_string("config.toml")?;  // io::Error → AppError::Io
    Ok(parse_config(&content))
}
```

### 嵌套错误分层

```rust
// 数据层错误
#[derive(Error, Debug)]
pub enum DbError {
    #[error("记录不存在")]
    NotFound,
    #[error("连接失败: {0}")]
    Connection(#[from] sqlx::Error),
}

// 服务层错误（包含数据层错误）
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("用户不存在")]
    UserNotFound,

    #[error("数据访问失败: {0}")]
    Db(#[from] DbError),   // 数据层错误自动转换进来

    #[error("业务规则违反: {0}")]
    BusinessRule(String),
}

// 服务层调用数据层，? 自动转换
fn get_user(id: u64) -> Result<User, ServiceError> {
    let user = db_find_user(id)?;   // DbError → ServiceError::Db
    Ok(user)
}
```

---

## 三、anyhow：顶层错误收口

```rust
use anyhow::{Context, Result, bail, ensure, anyhow};

// anyhow::Result<T> = Result<T, anyhow::Error>
// anyhow::Error 可以包装任何实现 std::error::Error 的类型

fn run() -> Result<()> {
    // context() / with_context()：给错误附加说明（最常用）
    let content = std::fs::read_to_string("data.txt")
        .context("读取数据文件失败")?;

    // with_context：懒求值，适合构造上下文比较耗时的情况
    let n: i32 = content.trim().parse()
        .with_context(|| format!("解析失败，原始内容: {:?}", content))?;

    // ensure!：条件断言，不满足则返回 Err
    ensure!(n > 0, "值必须为正数，实际为 {n}");
    ensure!(n < 1000, "值超出范围: {n}，最大允许 1000");

    // bail!：直接返回 Err（等价于 return Err(anyhow!("..."))）
    if n == 42 { bail!("不允许使用 42"); }

    // anyhow!：构造 anyhow::Error
    let err = anyhow!("发生了意料之外的错误，n={n}");

    Ok(())
}

// 打印完整错误链
fn main() {
    if let Err(e) = run() {
        eprintln!("错误: {e}");         // 只打印最外层
        eprintln!("详情: {e:?}");       // 打印完整调用链（带 source chain）
        // 或者逐层打印
        for (i, cause) in e.chain().enumerate() {
            eprintln!("  {i}: {cause}");
        }
    }
}
```

### anyhow 与 thiserror 混用

```rust
// 在 anyhow 的上下文里可以随时"向下转型"检查具体错误
fn handle_error(e: anyhow::Error) {
    if let Some(db_err) = e.downcast_ref::<DbError>() {
        // 是数据库错误，做特殊处理
        match db_err {
            DbError::NotFound => eprintln!("记录不存在"),
            DbError::Connection(_) => eprintln!("连接失败，稍后重试"),
        }
    } else {
        eprintln!("未知错误: {e}");
    }
}
```

---

## 四、HTTP 错误响应（以 axum 为例）

业务中最核心的需求：把内部错误类型转成统一的 JSON HTTP 响应。

```rust
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

// 统一 HTTP 错误结构
#[derive(Debug)]
pub struct ApiError {
    pub status:  StatusCode,
    pub code:    &'static str,   // 机器可读的错误码
    pub message: String,         // 人类可读的错误信息
}

// 实现 IntoResponse，让 ApiError 可以直接作为 Handler 的返回值
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "code":    self.code,
            "message": self.message,
        }));
        (self.status, body).into_response()
    }
}

// 把业务错误转成 HTTP 错误（核心映射）
impl From<AppError> for ApiError {
    fn from(e: AppError) -> Self {
        match e {
            AppError::UserNotFound { id } => ApiError {
                status:  StatusCode::NOT_FOUND,
                code:    "USER_NOT_FOUND",
                message: format!("用户 {id} 不存在"),
            },
            AppError::EmailAlreadyExists(email) => ApiError {
                status:  StatusCode::CONFLICT,
                code:    "EMAIL_EXISTS",
                message: format!("邮箱 {email} 已被注册"),
            },
            AppError::InvalidPassword => ApiError {
                status:  StatusCode::UNAUTHORIZED,
                code:    "INVALID_PASSWORD",
                message: "密码错误".into(),
            },
            AppError::BadRequest(msg) => ApiError {
                status:  StatusCode::BAD_REQUEST,
                code:    "BAD_REQUEST",
                message: msg,
            },
            // 数据库/IO 等内部错误：不暴露细节给客户端
            AppError::Database(e) => {
                tracing::error!("数据库错误: {e:?}");  // 记录内部日志
                ApiError {
                    status:  StatusCode::INTERNAL_SERVER_ERROR,
                    code:    "INTERNAL_ERROR",
                    message: "服务内部错误".into(),
                }
            },
            _ => {
                tracing::error!("未处理错误: {e:?}");
                ApiError {
                    status:  StatusCode::INTERNAL_SERVER_ERROR,
                    code:    "INTERNAL_ERROR",
                    message: "服务内部错误".into(),
                }
            }
        }
    }
}

// Handler 中使用：返回 Result<_, ApiError>
// axum 要求错误类型实现 IntoResponse
async fn get_user_handler(
    Path(user_id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = state.user_service.find_user(user_id)
        .await
        .map_err(ApiError::from)?;  // AppError → ApiError

    Ok(Json(UserResponse::from(user)))
}
```

### 更简洁：为 Result 实现 IntoResponse

```rust
// 定义类型别名，简化 Handler 签名
pub type ApiResult<T> = Result<T, ApiError>;

// Handler 使用
async fn create_user(
    Json(req): Json<CreateUserRequest>,
    State(state): State<AppState>,
) -> ApiResult<Json<UserResponse>> {
    let user = state.user_service
        .create_user(req)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(UserResponse::from(user)))
}
```

---

## 五、错误分层完整示例

```rust
// ─── 第一层：数据访问层错误 ───
#[derive(Error, Debug)]
pub enum RepoError {
    #[error("记录不存在")]
    NotFound,
    #[error("唯一约束冲突: {0}")]
    UniqueViolation(String),
    #[error("数据库错误: {0}")]
    Sqlx(#[from] sqlx::Error),
}

// ─── 第二层：业务服务层错误 ───
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("用户不存在: {0}")]
    UserNotFound(u64),
    #[error("邮箱已注册: {0}")]
    EmailTaken(String),
    #[error("数据访问失败: {0}")]
    Repo(#[from] RepoError),
}

// ─── 第三层：HTTP API 层错误 ───
impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::UserNotFound(id) => ApiError {
                status: StatusCode::NOT_FOUND,
                code: "USER_NOT_FOUND",
                message: format!("用户 {id} 不存在"),
            },
            ServiceError::EmailTaken(email) => ApiError {
                status: StatusCode::CONFLICT,
                code: "EMAIL_TAKEN",
                message: format!("{email} 已被注册"),
            },
            ServiceError::Repo(RepoError::NotFound) => ApiError {
                status: StatusCode::NOT_FOUND,
                code: "NOT_FOUND",
                message: "资源不存在".into(),
            },
            e => {
                tracing::error!("服务层错误: {e:?}");
                ApiError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    code: "INTERNAL_ERROR",
                    message: "服务内部错误".into(),
                }
            }
        }
    }
}

// ─── 服务层实现 ───
pub async fn register_user(
    pool: &sqlx::PgPool,
    email: &str,
    password: &str,
) -> Result<User, ServiceError> {
    // RepoError::UniqueViolation → ServiceError::EmailTaken（手动转换）
    user_repo::create(pool, email, password)
        .await
        .map_err(|e| match e {
            RepoError::UniqueViolation(_) => ServiceError::EmailTaken(email.to_string()),
            other => ServiceError::Repo(other),   // 其他错误透传
        })
}
```

---

## 六、常见模式与坑

### 6.1 不要丢失错误上下文

```rust
// ❌ 错误信息太模糊，排查困难
fn process(path: &str) -> Result<(), AppError> {
    let data = std::fs::read(path)?;  // 报错只说 "No such file"，不知道是哪个路径
    Ok(())
}

// ✅ 附加上下文
fn process(path: &str) -> anyhow::Result<()> {
    let data = std::fs::read(path)
        .with_context(|| format!("读取文件失败: {path}"))?;
    Ok(())
}
```

### 6.2 内部错误不要暴露给客户端

```rust
// ❌ 把数据库错误直接返回给客户端，泄露内部实现
AppError::Database(e) => ApiError {
    message: e.to_string(),  // "relation \"users\" does not exist"
}

// ✅ 内部记录日志，对外只返回通用错误
AppError::Database(e) => {
    tracing::error!(error = ?e, "数据库查询失败");
    ApiError { message: "服务内部错误".into(), .. }
}
```

### 6.3 async 函数中 ? 的类型推断

```rust
// ? 要求错误类型实现 From<原错误> for 返回错误
// 如果 From 没实现，要手动 map_err

async fn handler() -> Result<String, AppError> {
    // sqlx::Error → AppError::Database（需要 #[from] sqlx::Error）
    let row = sqlx::query!("SELECT name FROM users WHERE id = 1")
        .fetch_one(&pool)
        .await?;

    // reqwest::Error 没有 From impl，需要手动转
    let resp = reqwest::get("http://example.com")
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(row.name)
}
```

### 6.4 避免 unwrap / expect 在生产代码中

```rust
// ❌ 任何 unwrap 都是潜在的 panic
let val = some_map.get("key").unwrap();

// ✅ 用 ok_or 转 Result
let val = some_map.get("key")
    .ok_or_else(|| AppError::BadRequest("缺少 key 字段".into()))?;

// ✅ 只在"理论上不可能失败"的地方用 expect，并说明原因
let re = regex::Regex::new(r"^\d+$").expect("硬编码的正则表达式，不可能失败");
```

---

## 速查表

```
thiserror 用于：
  #[derive(Error)]                定义错误枚举
  #[error("...")]                 实现 Display（支持 {0} {field} 插值）
  #[from]                         自动生成 From 实现，让 ? 可以转换

anyhow 用于：
  anyhow::Result<T>              Result<T, anyhow::Error>
  .context("msg")                附加上下文字符串
  .with_context(|| ...)          懒求值上下文
  bail!("msg")                   return Err(anyhow!("msg"))
  ensure!(cond, "msg")           条件不满足则 return Err
  e.downcast_ref::<T>()          从 anyhow::Error 取出具体类型

HTTP 错误处理：
  impl IntoResponse for MyError  让错误类型可直接作为 axum Handler 返回值
  impl From<ServiceError> for ApiError  错误层间转换
  tracing::error!()              内部错误记录日志，不暴露给客户端
```
