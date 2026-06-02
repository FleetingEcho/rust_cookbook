# HTTP 客户端：reqwest 实战

```toml
[dependencies]
reqwest  = { version = "0.12", features = ["json", "rustls-tls"] }
serde    = { version = "1", features = ["derive"] }
tokio    = { version = "1", features = ["full"] }
anyhow   = "1"
thiserror = "1"
```

> reqwest 0.12 基于 hyper 1.0，`default-features = false` + `rustls-tls` 可避免依赖 OpenSSL。

---

## 一、Client 配置

```rust
use reqwest::{Client, ClientBuilder, header};
use std::time::Duration;

/// 构建一个全局共享的 Client（应用启动时创建一次，Arc 共享）
pub fn build_http_client() -> anyhow::Result<Client> {
    let mut default_headers = header::HeaderMap::new();
    default_headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );
    default_headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("MyApp/1.0"),
    );

    let client = ClientBuilder::new()
        .default_headers(default_headers)
        .timeout(Duration::from_secs(30))         // 整个请求超时
        .connect_timeout(Duration::from_secs(5))  // 建连超时
        .pool_max_idle_per_host(10)               // 连接池每个 host 最多空闲连接
        .pool_idle_timeout(Duration::from_secs(90))
        .https_only(true)                         // 只允许 HTTPS（生产建议开启）
        .build()?;

    Ok(client)
}

// 放入 AppState
#[derive(Clone)]
pub struct AppState {
    pub http: Client,
    // ...
}
```

---

## 二、GET 请求

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GithubUser {
    login:      String,
    id:         u64,
    avatar_url: String,
    name:       Option<String>,
}

/// 基础 GET + JSON 响应
pub async fn get_github_user(client: &Client, username: &str) -> anyhow::Result<GithubUser> {
    let url = format!("https://api.github.com/users/{username}");

    let user = client
        .get(&url)
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .error_for_status()?      // 4xx/5xx 自动转 Err
        .json::<GithubUser>()
        .await?;

    Ok(user)
}

/// 带查询参数
pub async fn search_repos(
    client:  &Client,
    keyword: &str,
    page:    u32,
) -> anyhow::Result<serde_json::Value> {
    client
        .get("https://api.github.com/search/repositories")
        .query(&[
            ("q",        keyword),
            ("sort",     "stars"),
            ("order",    "desc"),
            ("per_page", "20"),
            ("page",     &page.to_string()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(Into::into)
}
```

---

## 三、POST / PUT / PATCH 请求

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct CreateIssueRequest {
    title: String,
    body:  Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    labels: Vec<String>,
}

#[derive(Deserialize)]
struct IssueResponse {
    id:         u64,
    number:     u32,
    title:      String,
    html_url:   String,
}

/// POST JSON body
pub async fn create_issue(
    client: &Client,
    token:  &str,
    owner:  &str,
    repo:   &str,
    title:  &str,
) -> anyhow::Result<IssueResponse> {
    let body = CreateIssueRequest {
        title:  title.to_string(),
        body:   Some("Created via API".into()),
        labels: vec!["bug".into()],
    };

    let resp = client
        .post(format!("https://api.github.com/repos/{owner}/{repo}/issues"))
        .bearer_auth(token)                    // Authorization: Bearer <token>
        .json(&body)                           // 自动设置 Content-Type: application/json
        .send()
        .await?
        .error_for_status()?
        .json::<IssueResponse>()
        .await?;

    Ok(resp)
}

/// 发送表单（application/x-www-form-urlencoded）
pub async fn oauth_token_exchange(
    client:       &Client,
    client_id:    &str,
    client_secret: &str,
    code:         &str,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("https://github.com/login/oauth/access_token")
        .header(header::ACCEPT, "application/json")
        .form(&[
            ("client_id",     client_id),
            ("client_secret", client_secret),
            ("code",          code),
        ])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .map_err(Into::into)
}
```

---

## 四、错误处理与状态码区分

```rust
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("资源不存在: {0}")]
    NotFound(String),

    #[error("认证失败")]
    Unauthorized,

    #[error("请求过于频繁，请 {retry_after} 秒后重试")]
    RateLimited { retry_after: u64 },

    #[error("服务器错误: {status}")]
    ServerError { status: u16, body: String },

    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
}

/// 处理响应，区分不同 HTTP 错误
pub async fn handle_response<T: for<'de> serde::Deserialize<'de>>(
    resp: reqwest::Response,
) -> Result<T, ApiClientError> {
    let status = resp.status();

    if status.is_success() {
        return resp.json::<T>().await.map_err(ApiClientError::Network);
    }

    match status {
        StatusCode::NOT_FOUND => {
            Err(ApiClientError::NotFound(resp.url().to_string()))
        }
        StatusCode::UNAUTHORIZED => {
            Err(ApiClientError::Unauthorized)
        }
        StatusCode::TOO_MANY_REQUESTS => {
            let retry_after = resp.headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);
            Err(ApiClientError::RateLimited { retry_after })
        }
        s if s.is_server_error() => {
            let body = resp.text().await.unwrap_or_default();
            Err(ApiClientError::ServerError { status: s.as_u16(), body })
        }
        s => {
            let body = resp.text().await.unwrap_or_default();
            Err(ApiClientError::ServerError { status: s.as_u16(), body })
        }
    }
}
```

---

## 五、重试（指数退避）

```rust
use std::time::Duration;

pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay:   Duration,
    pub max_delay:    Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay:   Duration::from_millis(500),
            max_delay:    Duration::from_secs(10),
        }
    }
}

/// 对 5xx 错误和网络错误进行重试，4xx 不重试
pub async fn retry<F, Fut, T, E>(
    cfg: &RetryConfig,
    mut f: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = cfg.base_delay;

    for attempt in 1..=cfg.max_attempts {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) if attempt == cfg.max_attempts => {
                tracing::error!("第 {attempt} 次重试失败（已达最大次数）: {e}");
                return Err(e);
            }
            Err(e) => {
                tracing::warn!("第 {attempt} 次尝试失败，{delay:?} 后重试: {e}");
                tokio::time::sleep(delay).await;
                // 指数退避：delay *= 2，但不超过 max_delay
                delay = (delay * 2).min(cfg.max_delay);
            }
        }
    }
    unreachable!()
}

// 使用
let result = retry(&RetryConfig::default(), || async {
    client
        .get("https://api.example.com/data")
        .send()
        .await?
        .error_for_status()?
        .json::<MyData>()
        .await
}).await?;
```

---

## 六、并发请求

```rust
use futures::future;

/// 并发请求多个 URL，全部完成才返回
pub async fn fetch_all(
    client: &Client,
    urls:   Vec<String>,
) -> Vec<anyhow::Result<serde_json::Value>> {
    let futs = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            client.get(&url)
                .send()
                .await?
                .error_for_status()?
                .json::<serde_json::Value>()
                .await
                .map_err(anyhow::Error::from)
        }
    });

    future::join_all(futs).await
}

/// 并发请求，限制并发数（防止打垮对方服务）
pub async fn fetch_with_concurrency_limit(
    client:      &Client,
    urls:        Vec<String>,
    concurrency: usize,
) -> Vec<anyhow::Result<serde_json::Value>> {
    use futures::stream::{self, StreamExt};

    stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            async move {
                client.get(&url)
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<serde_json::Value>()
                    .await
                    .map_err(anyhow::Error::from)
            }
        })
        .buffer_unordered(concurrency)   // 最多 N 个并发
        .collect()
        .await
}
```

---

## 七、封装第三方 API 客户端

```rust
/// 推荐做法：封装为 struct，持有 Client 和 base_url
#[derive(Clone)]
pub struct PaymentClient {
    client:   Client,
    base_url: String,
    api_key:  String,
}

impl PaymentClient {
    pub fn new(base_url: &str, api_key: &str) -> anyhow::Result<Self> {
        Ok(Self {
            client:   build_http_client()?,
            base_url: base_url.to_string(),
            api_key:  api_key.to_string(),
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
    ) -> anyhow::Result<T> {
        self.client
            .get(self.url(path))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await
            .map_err(Into::into)
    }

    async fn post<B: Serialize, T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        self.client
            .post(self.url(path))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await
            .map_err(Into::into)
    }

    pub async fn create_charge(&self, req: &CreateChargeRequest) -> anyhow::Result<ChargeResponse> {
        self.post("/charges", req).await
    }

    pub async fn get_charge(&self, charge_id: &str) -> anyhow::Result<ChargeResponse> {
        self.get(&format!("/charges/{charge_id}")).await
    }
}
```

---

## 速查表

```
Client 创建：
  ClientBuilder::new()
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(5))
    .default_headers(headers)
    .build()?

请求方法：
  client.get(url)
  client.post(url)
  client.put(url)
  client.patch(url)
  client.delete(url)

请求修饰：
  .query(&[("key", "val")])          查询参数
  .json(&body)                       JSON body（自动设 Content-Type）
  .form(&[("key", "val")])           表单 body
  .bearer_auth(token)                Authorization: Bearer <token>
  .basic_auth(user, Some(pass))      Basic Auth
  .header(key, val)                  自定义 header
  .timeout(Duration)                 单次请求超时（覆盖全局）

响应处理：
  .send().await?                     发送请求
  .error_for_status()?               4xx/5xx 转 Err
  .json::<T>().await?                反序列化 JSON 响应
  .text().await?                     原始文本
  .bytes().await?                    原始字节
  .status()                          StatusCode
  .headers()                         HeaderMap
```
