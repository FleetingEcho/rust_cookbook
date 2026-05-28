// serde 是 Rust 生态中最常用的序列化/反序列化框架。
// serde 本身只定义接口，serde_json / serde_yaml / bincode 等提供具体格式。
//
// Cargo.toml 依赖（本项目已配置）：
//   serde = { version = "1", features = ["derive"] }
//   serde_json = "1"

use serde::{Deserialize, Serialize};

// ── 基础：derive Serialize / Deserialize ──────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

pub fn basic_json() {
    let user = User {
        id: 1,
        name: "Teng".to_string(),
        email: "teng@example.com".to_string(),
    };

    // 序列化：Rust 结构体 → JSON 字符串
    let json = serde_json::to_string(&user).unwrap();
    println!("序列化: {json}");

    // 美化输出（带缩进）
    let pretty = serde_json::to_string_pretty(&user).unwrap();
    println!("美化:\n{pretty}");

    // 反序列化：JSON 字符串 → Rust 结构体
    let json_str = r#"{"id":2,"name":"Alice","email":"alice@example.com"}"#;
    let parsed: User = serde_json::from_str(json_str).unwrap();
    println!("反序列化: {parsed:?}");
}

// ── 字段重命名 ────────────────────────────────────────────────────────────────

// rename_all：批量把所有字段从 snake_case 转成 camelCase（序列化时）
// 用在结构体属性上，不是字段属性
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub user_id: u64,       // JSON 里变成 "userId"
    pub created_at: String, // JSON 里变成 "createdAt"
}

pub fn show_rename_all() {
    let r = ApiResponse { user_id: 1, created_at: "2024-01-01".into() };
    let json = serde_json::to_string(&r).unwrap();
    println!("rename_all: {json}");
    // {"userId":1,"createdAt":"2024-01-01"}
}

// 单字段重命名示例
#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    #[serde(rename = "issueType")]  // JSON 键名是 issueType
    pub issue_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

pub fn show_rename() {
    let issue = Issue {
        id: 1,
        issue_type: "bug".to_string(),
        created_at: "2024-01-01".to_string(),
    };
    let json = serde_json::to_string(&issue).unwrap();
    println!("重命名后: {json}");
    // {"id":1,"issueType":"bug","createdAt":"2024-01-01"}
}

// ── 跳过字段 ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    #[serde(skip)]          // 序列化和反序列化都跳过这个字段
    pub internal_cache: u32,
    #[serde(skip_serializing_if = "Option::is_none")] // None 时不输出这个键
    pub description: Option<String>,
}

pub fn show_skip() {
    let cfg = Config {
        host: "localhost".to_string(),
        internal_cache: 42,
        description: None,
    };
    let json = serde_json::to_string(&cfg).unwrap();
    // internal_cache 和 description（None）都不会出现在 JSON 里
    println!("跳过字段: {json}");
}

// ── 默认值 ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub name: String,
    #[serde(default)]               // 反序列化时 JSON 中缺失则用 Default::default()
    pub timeout: u64,
    #[serde(default = "default_retries")] // 用自定义函数提供默认值
    pub retries: u32,
}

fn default_retries() -> u32 {
    3
}

pub fn show_default() {
    // JSON 里没有 timeout 和 retries
    let json = r#"{"name":"my-service"}"#;
    let s: Settings = serde_json::from_str(json).unwrap();
    println!("默认值: timeout={}, retries={}", s.timeout, s.retries);
    // timeout=0, retries=3
}

// ── 枚举序列化 ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")] // 枚举变体也可以重命名
pub enum Status {
    Open,
    InProgress,
    Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub status: Status,
}

pub fn show_enum() {
    let task = Task {
        title: "Fix bug".to_string(),
        status: Status::InProgress,
    };
    let json = serde_json::to_string(&task).unwrap();
    println!("枚举: {json}"); // {"title":"Fix bug","status":"in_progress"}

    // 反序列化回来
    let t: Task = serde_json::from_str(&json).unwrap();
    println!("还原: {t:?}");
}

// ── serde_json::Value：动态 JSON ──────────────────────────────────────────────

pub fn show_dynamic_json() {
    // 当不知道 JSON 结构时，可以先解析成 Value。
    let raw = r#"{"name":"Rust","version":1,"tags":["fast","safe"]}"#;
    let val: serde_json::Value = serde_json::from_str(raw).unwrap();

    // 用下标访问字段
    println!("name: {}", val["name"]);
    println!("version: {}", val["version"]);
    println!("first tag: {}", val["tags"][0]);

    // json! 宏构造 Value
    let obj = serde_json::json!({
        "key": "value",
        "numbers": [1, 2, 3]
    });
    println!("动态构造: {obj}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_user() {
        let user = User { id: 1, name: "A".into(), email: "a@b.com".into() };
        let json = serde_json::to_string(&user).unwrap();
        let back: User = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, 1);
        assert_eq!(back.name, "A");
    }

    #[test]
    fn default_fills_missing_fields() {
        let s: Settings = serde_json::from_str(r#"{"name":"x"}"#).unwrap();
        assert_eq!(s.timeout, 0);
        assert_eq!(s.retries, 3);
    }

    #[test]
    fn skip_none_option() {
        let cfg = Config { host: "h".into(), internal_cache: 0, description: None };
        let json = serde_json::to_string(&cfg).unwrap();
        assert!(!json.contains("description"));
        assert!(!json.contains("internal_cache"));
    }
}
