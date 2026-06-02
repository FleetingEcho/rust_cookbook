# 后台任务：定时任务 / 延迟任务 / 任务队列

```toml
[dependencies]
tokio              = { version = "1", features = ["full"] }
tokio-cron-scheduler = "0.13"
serde              = { version = "1", features = ["derive"] }
serde_json         = "1"
anyhow             = "1"
tracing            = "0.1"
uuid               = { version = "1", features = ["v4"] }
```

---

## 一、纯 Tokio：简单定时循环

```rust
use tokio::time::{interval, sleep, Duration, MissedTickBehavior};

/// 固定间隔执行（最简单的方式）
pub async fn start_cleanup_task(pool: sqlx::PgPool) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(3600)); // 每小时
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip); // 错过就跳过，不补执行

        loop {
            ticker.tick().await;

            match cleanup_expired_sessions(&pool).await {
                Ok(n)  => tracing::info!(count = n, "清理过期 session 完成"),
                Err(e) => tracing::error!(error = ?e, "清理过期 session 失败"),
            }
        }
    });
}

async fn cleanup_expired_sessions(pool: &sqlx::PgPool) -> anyhow::Result<u64> {
    let result = sqlx::query!(
        "DELETE FROM sessions WHERE expires_at < NOW()"
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// 启动多个后台任务
pub async fn start_background_tasks(pool: sqlx::PgPool) {
    // 每小时清理过期 session
    start_cleanup_task(pool.clone()).await;

    // 每 5 分钟同步数据
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(300));
        loop {
            ticker.tick().await;
            if let Err(e) = sync_stats(&pool).await {
                tracing::error!(error = ?e, "同步统计失败");
            }
        }
    });
}
```

---

## 二、Cron 表达式定时任务

```rust
use tokio_cron_scheduler::{JobScheduler, Job};

pub async fn start_cron_scheduler() -> anyhow::Result<JobScheduler> {
    let sched = JobScheduler::new().await?;

    // 每天凌晨 2:00 执行（cron: 秒 分 时 日 月 周）
    sched.add(
        Job::new_async("0 0 2 * * *", |_uuid, _lock| {
            Box::pin(async move {
                tracing::info!("开始每日数据归档...");
                if let Err(e) = daily_archive().await {
                    tracing::error!(error = ?e, "每日归档失败");
                }
            })
        })?
    ).await?;

    // 每 30 分钟执行一次
    sched.add(
        Job::new_async("0 */30 * * * *", |_uuid, _lock| {
            Box::pin(async move {
                if let Err(e) = refresh_hot_cache().await {
                    tracing::error!(error = ?e, "刷新热缓存失败");
                }
            })
        })?
    ).await?;

    // 每周一 9:00 发送周报
    sched.add(
        Job::new_async("0 0 9 * * Mon", |_uuid, _lock| {
            Box::pin(async move {
                if let Err(e) = send_weekly_report().await {
                    tracing::error!(error = ?e, "发送周报失败");
                }
            })
        })?
    ).await?;

    sched.start().await?;
    Ok(sched)
}

// main.rs 中启动
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _sched = start_cron_scheduler().await?;
    // 持有 _sched，防止被 drop 导致 cron 停止
    Ok(())
}
```

### Cron 表达式速查

```
格式：秒 分 时 日 月 周

常用示例：
  "0 * * * * *"          每分钟整点（每分钟第 0 秒）
  "0 */5 * * * *"        每 5 分钟
  "0 0 * * * *"          每小时整点
  "0 0 2 * * *"          每天凌晨 2:00
  "0 0 9 * * Mon"        每周一 9:00
  "0 0 0 1 * *"          每月 1 日凌晨
  "0 30 8-17 * * Mon-Fri" 工作日 8:30-17:30 每小时执行
```

---

## 三、任务队列（基于内存 channel）

```rust
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};

/// 任务类型枚举（所有后台任务都在这里定义）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum BackgroundTask {
    SendEmail       { to: String, subject: String, body: String },
    GenerateReport  { user_id: i64, report_type: String },
    ProcessWebhook  { payload: serde_json::Value, source: String },
    ResizeImage     { image_url: String, sizes: Vec<u32> },
}

/// 任务队列（发送端）
#[derive(Clone)]
pub struct TaskQueue {
    tx: mpsc::Sender<BackgroundTask>,
}

impl TaskQueue {
    /// 创建任务队列，启动 N 个 worker
    pub fn start(worker_count: usize, pool: sqlx::PgPool) -> Self {
        let (tx, rx) = mpsc::channel::<BackgroundTask>(1000); // 缓冲 1000 个任务
        let rx = std::sync::Arc::new(tokio::sync::Mutex::new(rx));

        for worker_id in 0..worker_count {
            let rx    = rx.clone();
            let pool  = pool.clone();
            tokio::spawn(async move {
                tracing::info!(worker_id, "任务 worker 启动");
                loop {
                    let task = {
                        let mut guard = rx.lock().await;
                        guard.recv().await
                    };
                    match task {
                        Some(t) => {
                            if let Err(e) = process_task(&t, &pool).await {
                                tracing::error!(
                                    worker_id,
                                    task = ?t,
                                    error = ?e,
                                    "任务处理失败"
                                );
                            }
                        }
                        None => {
                            tracing::info!(worker_id, "队列已关闭，worker 退出");
                            break;
                        }
                    }
                }
            });
        }

        Self { tx }
    }

    /// 提交任务（非阻塞，队列满时返回 Err）
    pub async fn enqueue(&self, task: BackgroundTask) -> anyhow::Result<()> {
        self.tx.send(task).await
            .map_err(|_| anyhow::anyhow!("任务队列已关闭"))?;
        Ok(())
    }

    /// 提交任务（尝试一次，队列满就丢弃，适合非重要任务）
    pub fn try_enqueue(&self, task: BackgroundTask) {
        if let Err(e) = self.tx.try_send(task) {
            tracing::warn!(error = ?e, "任务队列已满，任务被丢弃");
        }
    }
}

/// 任务处理器（分发给具体实现）
async fn process_task(task: &BackgroundTask, pool: &sqlx::PgPool) -> anyhow::Result<()> {
    match task {
        BackgroundTask::SendEmail { to, subject, body } => {
            send_email(to, subject, body).await?;
        }
        BackgroundTask::GenerateReport { user_id, report_type } => {
            generate_report(*user_id, report_type, pool).await?;
        }
        BackgroundTask::ProcessWebhook { payload, source } => {
            process_webhook(payload, source).await?;
        }
        BackgroundTask::ResizeImage { image_url, sizes } => {
            resize_image(image_url, sizes).await?;
        }
    }
    Ok(())
}

// 在 axum Handler 中使用
async fn register_user(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<RegisterRequest>,
) -> impl IntoResponse {
    let user = state.user_service.create_user(&req).await?;

    // 异步发送欢迎邮件（不阻塞响应）
    state.task_queue.enqueue(BackgroundTask::SendEmail {
        to:      user.email.clone(),
        subject: "欢迎加入！".into(),
        body:    format!("Hi {}，欢迎注册！", user.username),
    }).await.ok();  // 入队失败不影响主流程

    Ok(Json(UserResponse::from(user)))
}
```

---

## 四、带重试的任务队列

```rust
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TaskItem {
    pub id:          String,
    pub task:        BackgroundTask,
    pub attempt:     u32,
    pub max_attempts: u32,
    pub next_run_at: tokio::time::Instant,
}

impl TaskItem {
    pub fn new(task: BackgroundTask) -> Self {
        Self {
            id:           uuid::Uuid::new_v4().to_string(),
            task,
            attempt:      0,
            max_attempts: 3,
            next_run_at:  tokio::time::Instant::now(),
        }
    }

    pub fn next_retry_delay(&self) -> Duration {
        // 指数退避：500ms, 2s, 8s
        let base = 500_u64;
        let delay = base * 2_u64.pow(self.attempt);
        Duration::from_millis(delay.min(30_000))
    }
}

pub async fn run_worker_with_retry(
    mut rx:   mpsc::Receiver<TaskItem>,
    pool:     sqlx::PgPool,
    retry_tx: mpsc::Sender<TaskItem>,
) {
    while let Some(mut item) = rx.recv().await {
        // 等到指定时间才执行
        let now = tokio::time::Instant::now();
        if item.next_run_at > now {
            tokio::time::sleep_until(item.next_run_at).await;
        }

        item.attempt += 1;
        tracing::info!(
            task_id = %item.id,
            attempt = item.attempt,
            "执行任务"
        );

        match process_task(&item.task, &pool).await {
            Ok(_) => {
                tracing::info!(task_id = %item.id, "任务完成");
            }
            Err(e) if item.attempt < item.max_attempts => {
                let delay = item.next_retry_delay();
                tracing::warn!(
                    task_id = %item.id,
                    attempt = item.attempt,
                    delay   = ?delay,
                    error   = ?e,
                    "任务失败，将重试"
                );
                item.next_run_at = tokio::time::Instant::now() + delay;
                let _ = retry_tx.send(item).await;
            }
            Err(e) => {
                tracing::error!(
                    task_id = %item.id,
                    error   = ?e,
                    "任务达到最大重试次数，放弃"
                );
                // 可以写入数据库 dead letter queue
            }
        }
    }
}
```

---

## 五、优雅关闭

```rust
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

pub struct BackgroundRunner {
    cancel: CancellationToken,
}

impl BackgroundRunner {
    pub fn new() -> Self {
        Self { cancel: CancellationToken::new() }
    }

    /// 启动后台任务（携带取消令牌）
    pub fn spawn<F, Fut>(&self, name: &'static str, f: F)
    where
        F: FnOnce(CancellationToken) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let token = self.cancel.clone();
        tokio::spawn(async move {
            tracing::info!(task = name, "后台任务启动");
            f(token).await;
            tracing::info!(task = name, "后台任务退出");
        });
    }

    /// 发送停止信号（会等待当前任务处理完当前批次）
    pub fn shutdown(&self) {
        tracing::info!("发送关闭信号给所有后台任务");
        self.cancel.cancel();
    }
}

// 使用示例
let runner = BackgroundRunner::new();

runner.spawn("email-sender", |cancel| async move {
    let mut ticker = interval(Duration::from_secs(10));
    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                tracing::info!("邮件发送任务收到关闭信号");
                break;
            }
            _ = ticker.tick() => {
                process_pending_emails().await;
            }
        }
    }
});

// 收到 SIGTERM 时
tokio::signal::ctrl_c().await?;
runner.shutdown();
sleep(Duration::from_secs(10)).await;  // 给任务时间完成当前工作
```

---

## 速查表

```
简单定时：
  interval(Duration)            固定间隔
  .set_missed_tick_behavior(Skip)  跳过错过的 tick

Cron（tokio-cron-scheduler）：
  JobScheduler::new().await     创建调度器
  Job::new_async("cron", f)     创建 cron 任务
  sched.add(job).await          注册任务
  sched.start().await           启动调度

任务队列：
  mpsc::channel(capacity)       创建有界 channel
  tx.send(task).await           入队（满时等待）
  tx.try_send(task)             入队（满时丢弃）
  rx.recv().await               取任务（None 表示队列关闭）

优雅关闭：
  CancellationToken::new()      创建取消令牌
  token.cancel()                触发取消
  token.cancelled().await       等待取消
  select! { _ = token.cancelled() => break, _ = work() => {} }  任务内响应取消
```
