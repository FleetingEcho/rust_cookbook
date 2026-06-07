# 可观测性：健康检查 / Prometheus 指标 / 结构化日志

```toml
[dependencies]
axum                      = "0.7"
tokio                     = { version = "1", features = ["full"] }
tracing                   = "0.1"
tracing-subscriber        = { version = "0.3", features = ["env-filter", "json"] }
metrics                   = "0.23"
metrics-exporter-prometheus = "0.15"
serde                     = { version = "1", features = ["derive"] }
serde_json                = "1"
```

---

## 一、健康检查端点

```rust
use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status:   &'static str,   // "ok" | "degraded" | "down"
    pub version:  &'static str,
    pub checks:   Vec<CheckResult>,
}

#[derive(Serialize)]
pub struct CheckResult {
    pub name:    &'static str,
    pub status:  &'static str,
    pub latency_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error:   Option<String>,
}

/// /health：快速检查，负载均衡/K8s liveness probe 用
pub async fn liveness_handler() -> StatusCode {
    StatusCode::OK   // 进程在就 200，不检查依赖
}

/// /ready：完整检查，K8s readiness probe 用
pub async fn readiness_handler(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<HealthResponse>) {
    let mut checks = Vec::new();
    let mut all_ok = true;

    // 检查数据库
    let db_check = check_database(&state.db).await;
    if db_check.status == "down" { all_ok = false; }
    checks.push(db_check);

    // 检查 Redis
    let redis_check = check_redis(&state.redis).await;
    if redis_check.status == "down" { all_ok = false; }
    checks.push(redis_check);

    let status = if all_ok { "ok" } else { "down" };
    let http_status = if all_ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };

    (http_status, Json(HealthResponse {
        status,
        version: env!("CARGO_PKG_VERSION"),
        checks,
    }))
}

async fn check_database(pool: &sqlx::PgPool) -> CheckResult {
    let start = std::time::Instant::now();
    let result = sqlx::query("SELECT 1").fetch_one(pool).await;
    let latency_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_)  => CheckResult { name: "database", status: "ok", latency_ms, error: None },
        Err(e) => CheckResult { name: "database", status: "down", latency_ms, error: Some(e.to_string()) },
    }
}

async fn check_redis(pool: &deadpool_redis::Pool) -> CheckResult {
    use deadpool_redis::redis::AsyncCommands;
    let start = std::time::Instant::now();
    let result: anyhow::Result<()> = async {
        let mut conn = pool.get().await?;
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(())
    }.await;
    let latency_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_)  => CheckResult { name: "redis", status: "ok", latency_ms, error: None },
        Err(e) => CheckResult { name: "redis", status: "down", latency_ms, error: Some(e.to_string()) },
    }
}

/// 注册健康检查路由
pub fn health_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health",  get(liveness_handler))
        .route("/ready",   get(readiness_handler))
        // /metrics 见下文
}
```

---

## 二、Prometheus 指标

### 2.1 初始化

```rust
use metrics_exporter_prometheus::PrometheusBuilder;

/// 启动时调用一次
pub fn init_metrics() -> anyhow::Result<()> {
    PrometheusBuilder::new()
        // 可以设置全局标签
        .add_global_label("service", "my-api")
        .add_global_label("env", std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into()))
        .install()?;   // 注册全局 recorder

    // 提前注册指标描述（可选，但可以让 /metrics 在首次请求前就有指标）
    metrics::describe_counter!(
        "http_requests_total",
        "HTTP 请求总数"
    );
    metrics::describe_histogram!(
        "http_request_duration_seconds",
        "HTTP 请求耗时（秒）"
    );
    metrics::describe_gauge!(
        "active_connections",
        "当前活跃连接数"
    );

    Ok(())
}
```

### 2.2 指标类型与使用

```rust
use metrics::{counter, gauge, histogram};

// ─── Counter：只增不减，用于计数 ───
counter!("http_requests_total",
    "method" => "GET",
    "path"   => "/users",
    "status" => "200",
).increment(1);

counter!("errors_total", "type" => "database").increment(1);

// ─── Gauge：可增可减，用于当前状态 ───
gauge!("active_connections").set(42.0);
gauge!("active_connections").increment(1.0);  // +1
gauge!("active_connections").decrement(1.0);  // -1
gauge!("queue_depth", "name" => "email").set(queue_len as f64);

// ─── Histogram：分布统计，用于延迟/大小 ───
histogram!("http_request_duration_seconds",
    "method" => "POST",
    "path"   => "/orders",
).record(0.045);  // 45ms = 0.045s

histogram!("db_query_duration_seconds",
    "query" => "find_user",
).record(elapsed.as_secs_f64());
```

### 2.3 HTTP 请求指标中间件

```rust
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn metrics_middleware(req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let path   = req.uri().path().to_string();

    // 规范化路径（防止路径参数导致指标爆炸）
    // /users/123 → /users/:id
    let normalized_path = normalize_path(&path);

    let start = Instant::now();

    // 活跃请求数 +1
    gauge!("http_requests_in_flight").increment(1.0);

    let resp = next.run(req).await;

    // 活跃请求数 -1
    gauge!("http_requests_in_flight").decrement(1.0);

    let status  = resp.status().as_u16().to_string();
    let elapsed = start.elapsed().as_secs_f64();

    // 请求计数
    counter!("http_requests_total",
        "method" => method.clone(),
        "path"   => normalized_path.clone(),
        "status" => status,
    ).increment(1);

    // 请求延迟
    histogram!("http_request_duration_seconds",
        "method" => method,
        "path"   => normalized_path,
    ).record(elapsed);

    resp
}

fn normalize_path(path: &str) -> String {
    // 简单实现：把纯数字段替换为 :id
    path.split('/')
        .map(|seg| if seg.chars().all(|c| c.is_ascii_digit()) { ":id" } else { seg })
        .collect::<Vec<_>>()
        .join("/")
}
```

### 2.4 /metrics 端点

```rust
use axum::response::IntoResponse;
use metrics_exporter_prometheus::PrometheusHandle;

pub async fn metrics_handler(
    State(handle): State<PrometheusHandle>,
) -> impl IntoResponse {
    handle.render()
}

// 注册路由时
let prometheus_handle = PrometheusBuilder::new()
    .install_recorder()?;

let app = Router::new()
    .route("/metrics", get(metrics_handler))
    .with_state(prometheus_handle);
```

---

## 三、结构化日志（tracing 进阶）

### 3.1 完整初始化（区分开发/生产）

```rust
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn init_tracing(is_production: bool) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            if is_production {
                EnvFilter::new("info")
            } else {
                // 开发时显示更多，但屏蔽噪声库
                EnvFilter::new("debug,hyper=warn,sqlx=warn,tower=warn")
            }
        });

    if is_production {
        // 生产：JSON 格式，便于日志收集（Loki/ELK）
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().json()
                .with_current_span(true)
                .with_span_list(false))
            .init();
    } else {
        // 开发：彩色文本，更易读
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_thread_ids(false))
            .init();
    }
}
```

### 3.2 最佳实践

```rust
use tracing::{info, warn, error, debug, instrument, Span};

// ✅ 结构化字段（不要字符串拼接）
info!(user_id = 42, action = "login", ip = %"1.2.3.4", "用户登录成功");
error!(order_id = 99, amount = 500, error = ?e, "支付失败");

// % 用 Display 格式，? 用 Debug 格式
debug!(path = %req.uri(), headers = ?req.headers());

// ─── instrument：函数自动创建 span ───
#[instrument(
    skip(pool, password),          // 跳过不想记录的字段（敏感信息）
    fields(user_id = tracing::field::Empty),  // 运行时填充
    err,                           // Err 时自动记录错误
)]
async fn create_user(
    pool:     &sqlx::PgPool,
    email:    &str,
    password: &str,
) -> Result<User, AppError> {
    let user = insert_user(pool, email, password).await?;

    // 运行时填充 span 字段
    Span::current().record("user_id", user.id);

    info!("用户创建成功");
    Ok(user)
}

// ─── 手动创建 span（覆盖 instrument 做不到的场景）───
pub async fn process_batch(items: Vec<Item>) {
    for (i, item) in items.iter().enumerate() {
        let span = tracing::info_span!(
            "process_item",
            item_id  = item.id,
            batch_idx = i,
        );

        async {
            if let Err(e) = handle_item(item).await {
                error!(error = ?e, "处理失败");
            }
        }
        .instrument(span)
        .await;
    }
}
```

### 3.3 请求 ID 追踪

```rust
use axum::{extract::Request, middleware::Next, response::Response};
use tracing::Instrument;
use uuid::Uuid;

pub async fn trace_request_middleware(req: Request, next: Next) -> Response {
    let request_id = req.headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method     = %req.method(),
        path       = %req.uri().path(),
    );

    let mut resp = next.run(req).instrument(span).await;

    // 把 request_id 写回响应 header
    resp.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    resp
}
```

---

## 四、完整路由组装

```rust
pub fn build_app(state: Arc<AppState>, prometheus_handle: PrometheusHandle) -> Router {
    Router::new()
        // 业务路由
        .nest("/api/v1", api_routes())
        // 可观测性路由（通常不鉴权，但可以限制内网访问）
        .route("/health",  get(liveness_handler))
        .route("/ready",   get(readiness_handler))
        .route("/metrics", get(metrics_handler).with_state(prometheus_handle))
        // 中间件（从内到外的顺序）
        .layer(axum::middleware::from_fn(metrics_middleware))
        .layer(axum::middleware::from_fn(trace_request_middleware))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}
```

---

## 五、常用监控指标清单

```
业务指标（按需添加）：
  http_requests_total{method,path,status}        请求总数（Counter）
  http_request_duration_seconds{method,path}     请求延迟（Histogram）
  http_requests_in_flight                        并发请求数（Gauge）
  db_query_duration_seconds{query}               DB 查询延迟（Histogram）
  db_pool_connections{state}                     DB 连接池状态（Gauge）
  cache_hits_total / cache_misses_total          缓存命中率（Counter）
  task_queue_depth{queue}                        任务队列深度（Gauge）
  errors_total{type}                             错误总数（Counter）
  auth_failures_total{reason}                    认证失败数（Counter）

Prometheus 常用查询：
  rate(http_requests_total[5m])                  5 分钟请求速率
  histogram_quantile(0.95, ...)                  P95 延迟
  sum by (status) (rate(http_requests_total[1m])) 各状态码速率
  errors_total / http_requests_total             错误率
```

---

## 速查表

```
健康检查：
  /health   → 只检查进程（快，liveness）
  /ready    → 检查所有依赖（readiness，影响流量切入）
  503       → 服务不可用时返回

指标：
  counter!("name", "label"=>"val").increment(n)   只增计数
  gauge!("name").set(v) / .increment(v)            当前状态值
  histogram!("name").record(seconds)               延迟/大小分布

日志：
  info!(key=val, "msg")         结构化字段（不要字符串拼接）
  error!(error=?e, "msg")       错误用 ? (Debug)
  %                             Display 格式
  ?                             Debug 格式
  #[instrument(skip(secret))]  自动 span，跳过敏感字段
  Span::current().record("k",v) 运行时补充字段

RUST_LOG=info,sqlx=warn         控制日志级别（模块粒度）
```
