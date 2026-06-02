# 请求数据校验：validator 实战

```toml
[dependencies]
validator = { version = "0.18", features = ["derive"] }
axum      = "0.7"
serde     = { version = "1", features = ["derive"] }
thiserror = "1"
regex     = "1"         # 自定义正则校验时使用
once_cell = "1"         # 全局编译正则
```

---

## 一、基础用法

```rust
use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50, message = "用户名长度必须在 3-50 之间"))]
    pub username: String,

    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    #[validate(length(min = 8, message = "密码不能少于 8 位"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "两次密码不一致"))]
    pub password_confirm: String,

    #[validate(range(min = 1, max = 120, message = "年龄必须在 1-120 之间"))]
    #[serde(default)]
    pub age: Option<u8>,
}

// 手动调用（不用 axum 时）
let req = RegisterRequest { /* ... */ };
if let Err(errors) = req.validate() {
    println!("{errors:#?}");
}
```

---

## 二、所有内置校验规则

```rust
use validator::Validate;

#[derive(Validate)]
struct AllValidators {
    // ─── 字符串 ───
    #[validate(length(min = 1))]
    not_empty: String,

    #[validate(length(min = 3, max = 50))]
    bounded_len: String,

    #[validate(email)]
    email: String,

    #[validate(url)]
    website: String,

    #[validate(contains(pattern = "@"))]
    must_contain: String,

    #[validate(does_not_contain(pattern = "admin"))]
    no_admin: String,

    #[validate(regex(path = "USERNAME_RE"))]  // 见下方自定义正则
    username: String,

    // ─── 数字 ───
    #[validate(range(min = 0.0, max = 100.0))]
    score: f64,

    #[validate(range(min = 1))]
    positive: i32,

    // ─── 集合 ───
    #[validate(length(min = 1, max = 10))]
    #[validate(each(|item| validate_tag(item)))]  // 对每个元素校验
    tags: Vec<String>,

    // ─── 嵌套结构体 ───
    #[validate(nested)]
    address: Address,

    // ─── Option（有值时才校验）───
    #[validate(length(min = 10, max = 500))]
    bio: Option<String>,

    // ─── 自定义函数 ───
    #[validate(custom(function = "validate_phone"))]
    phone: String,
}

#[derive(Validate)]
struct Address {
    #[validate(length(min = 1, max = 100))]
    street: String,
    #[validate(length(min = 1, max = 50))]
    city:   String,
}
```

---

## 三、自定义校验函数与正则

```rust
use once_cell::sync::Lazy;
use regex::Regex;
use validator::ValidationError;

// 全局编译正则（只编译一次）
static USERNAME_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9_]{3,50}$").unwrap()
});

static PHONE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^1[3-9]\d{9}$").unwrap()
});

/// 自定义校验函数：返回 Ok(()) 或 Err(ValidationError)
fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if PHONE_RE.is_match(phone) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_phone");
        err.message = Some("手机号格式不正确（中国大陆 11 位）".into());
        Err(err)
    }
}

fn validate_username_not_reserved(username: &str) -> Result<(), ValidationError> {
    const RESERVED: &[&str] = &["admin", "root", "system", "test"];
    if RESERVED.contains(&username.to_lowercase().as_str()) {
        let mut err = ValidationError::new("reserved_username");
        err.message = Some("该用户名被系统保留，不可使用".into());
        Err(err)
    } else {
        Ok(())
    }
}

#[derive(Validate, Deserialize)]
struct CreateUser {
    #[validate(regex(path = "USERNAME_RE", message = "用户名只能包含字母、数字和下划线"))]
    #[validate(custom(function = "validate_username_not_reserved"))]
    username: String,

    #[validate(custom(function = "validate_phone"))]
    phone: String,
}
```

---

## 四、与 axum 集成：ValidatedJson 提取器

```rust
use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response, Json},
};
use serde::de::DeserializeOwned;
use validator::Validate;
use serde_json::json;

/// 自定义提取器：先反序列化 JSON，再校验，失败返回 422 + 错误详情
pub struct ValidatedJson<T>(pub T);

#[axum::async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. 先用 axum 的 Json 提取器反序列化
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| ValidationErrorResponse::parse_error(e.to_string()))?;

        // 2. 再校验
        value.validate()
            .map_err(ValidationErrorResponse::from_validation_errors)?;

        Ok(ValidatedJson(value))
    }
}

/// 统一的校验错误响应
pub struct ValidationErrorResponse {
    pub status:  StatusCode,
    pub code:    &'static str,
    pub message: String,
    pub fields:  serde_json::Value,
}

impl ValidationErrorResponse {
    fn parse_error(msg: String) -> Self {
        Self {
            status:  StatusCode::BAD_REQUEST,
            code:    "PARSE_ERROR",
            message: msg,
            fields:  json!({}),
        }
    }

    fn from_validation_errors(errors: validator::ValidationErrors) -> Self {
        // 把 ValidationErrors 转成清晰的 JSON 格式
        let mut fields = serde_json::Map::new();

        for (field, errs) in errors.field_errors() {
            let messages: Vec<String> = errs.iter()
                .map(|e| e.message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("校验失败: {}", e.code)))
                .collect();
            fields.insert(field.to_string(), json!(messages));
        }

        Self {
            status:  StatusCode::UNPROCESSABLE_ENTITY,
            code:    "VALIDATION_ERROR",
            message: "请求参数校验失败".into(),
            fields:  serde_json::Value::Object(fields),
        }
    }
}

impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({
                "success": false,
                "error": {
                    "code":    self.code,
                    "message": self.message,
                    "fields":  self.fields,
                }
            }))
        ).into_response()
    }
}
```

### Handler 中使用

```rust
// Handler 直接用 ValidatedJson，校验失败自动返回 422
async fn register(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<RegisterRequest>,
) -> impl IntoResponse {
    // 走到这里说明校验已通过
    state.user_service.register(req).await
}

// 错误响应示例：
// HTTP 422
// {
//   "success": false,
//   "error": {
//     "code": "VALIDATION_ERROR",
//     "message": "请求参数校验失败",
//     "fields": {
//       "email":    ["邮箱格式不正确"],
//       "password": ["密码不能少于 8 位"]
//     }
//   }
// }
```

---

## 五、跨字段校验（结构体级别）

```rust
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_date_range", skip_on_field_errors = true))]
pub struct DateRangeQuery {
    #[validate(length(min = 10, max = 10))]  // YYYY-MM-DD
    pub start_date: String,

    #[validate(length(min = 10, max = 10))]
    pub end_date: String,

    #[validate(range(min = 1, max = 365))]
    #[serde(default = "default_max_days")]
    pub max_days: u32,
}

fn default_max_days() -> u32 { 90 }

/// 跨字段校验：start_date 必须早于 end_date
fn validate_date_range(query: &DateRangeQuery) -> Result<(), ValidationError> {
    if query.start_date > query.end_date {
        let mut err = ValidationError::new("invalid_date_range");
        err.message = Some("开始日期不能晚于结束日期".into());
        return Err(err);
    }
    Ok(())
}
```

---

## 六、常见场景速查

```rust
// ─ 非空字符串 ─
#[validate(length(min = 1, message = "不能为空"))]
name: String,

// ─ 中文手机号 ─
#[validate(regex(path = "PHONE_RE", message = "手机号格式错误"))]
phone: String,

// ─ 强密码（至少 8 位，包含大小写和数字）─
#[validate(custom(function = "validate_strong_password"))]
password: String,

fn validate_strong_password(pw: &str) -> Result<(), ValidationError> {
    if pw.len() < 8
        || !pw.chars().any(|c| c.is_uppercase())
        || !pw.chars().any(|c| c.is_lowercase())
        || !pw.chars().any(|c| c.is_ascii_digit())
    {
        let mut e = ValidationError::new("weak_password");
        e.message = Some("密码须包含大小写字母和数字，且不少于 8 位".into());
        return Err(e);
    }
    Ok(())
}

// ─ 分页参数 ─
#[derive(Deserialize, Validate)]
pub struct PageQuery {
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u32,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}
fn default_page()     -> u32 { 1  }
fn default_per_page() -> u32 { 20 }

// ─ ID 列表 ─
#[derive(Deserialize, Validate)]
pub struct BatchDeleteRequest {
    #[validate(length(min = 1, max = 100, message = "ids 不能为空且最多 100 个"))]
    pub ids: Vec<i64>,
}
```

---

## 速查表

```
内置规则：
  length(min=, max=)          字符串/集合长度
  range(min=, max=)           数值范围
  email                       邮箱格式
  url                         URL 格式
  contains(pattern=)          必须包含
  does_not_contain(pattern=)  不能包含
  regex(path = "REGEX_VAR")   正则（Lazy<Regex>）
  must_match(other = "field") 两字段相同（如确认密码）
  custom(function = "fn")     自定义函数

属性：
  message = "..."             自定义错误信息
  skip_on_field_errors = true 字段有错时跳过结构体校验

标注：
  #[validate(nested)]         校验嵌套结构体
  #[validate(schema(function = "fn"))]  结构体级别跨字段校验

axum 集成：
  ValidatedJson<T>            自定义提取器，校验失败 → 422
  StatusCode::UNPROCESSABLE_ENTITY (422)  校验失败的标准状态码
```
