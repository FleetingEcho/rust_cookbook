# Rust 学习路线图 —— 下一阶段计划

> 现状：语言概念已系统学完（rust_concepts / type_system / 100 exercises 全部完成，
> rustlings 接近尾声），已有两个能跑的 CRUD API（cookbook-api、issue_tracker_api）。
> 下一阶段的目标不是学更多概念，而是补上 **「教学项目」到「生产项目」之间的差距**。

---

## 总体思路

```
Phase 0  收尾          （1-2 天）   把已开的坑填完
Phase 1  生产化改造     （2-3 周）   把 issue_tracker_api 升级成作品集级项目
Phase 2  并发深水区     （2-3 周）   做一个真正考验并发的项目
Phase 3  走出自己的仓库 （长期）     读源码、贡献开源、底层练习
```

优先级：**Phase 1 价值最高**。它能用掉 `web_development_guide/` 里几乎所有
还没实践过的文档（auth / redis / background_jobs / deployment / observability），
并且产出一个可以放进简历的项目。

---

## Phase 0：收尾（1-2 天）

- [ ] rustlings 剩余练习做完（当前停在 `as_ref_mut`）
- [ ] cookbook-api Step 8 测试补齐（plan.md 里唯一没勾的项）：
  - [ ] Markdown 解析器单元测试（边界情况：标题变体、无用量食材、内部链接）
  - [ ] `axum-test` 集成测试：搜索 / 分页 / 详情 / 404
- [ ] 全 workspace 跑一遍 `cargo clippy --workspace -- -D warnings`，清零告警

---

## Phase 1：issue_tracker_api 生产化改造（核心，2-3 周）

把现有 issue_tracker_api 从「能跑的 CRUD」升级为「可部署的生产服务」。
每一步都对应一篇已有但还没实践的指南文档。

### 1.1 认证与授权（对应 `auth.md`）★ 最高优先级

- [ ] 用户注册/登录端点，密码用 `argon2` 哈希
- [ ] JWT 签发与验证（`jsonwebtoken`），access token + refresh token
- [ ] 认证中间件：axum extractor 方式提取当前用户
- [ ] RBAC：至少两种角色（admin / member），admin 才能删 issue
- [ ] 资源归属检查：只有作者能编辑自己的 comment

### 1.2 迁移到 PostgreSQL（对应 `database_sqlx.md` 未覆盖部分）

- [ ] Docker Compose 起 Postgres，替换 SQLite
- [ ] 重写 migrations（SERIAL/UUID、TIMESTAMPTZ、外键行为差异）
- [ ] 用上 Postgres 特有能力：`RETURNING`、事务隔离级别、`ILIKE` 或 FTS
- [ ] 连接池参数调优（max_connections、acquire_timeout）并理解含义

### 1.3 集成测试体系（对应 `config_logging_testing.md`）

- [ ] 每个测试用独立的临时数据库（或 testcontainers 起 Postgres）
- [ ] `tower::ServiceExt::oneshot` 或 `axum-test` 覆盖主要 handler
- [ ] 测试认证流程：无 token / 过期 token / 权限不足
- [ ] 目标：CI 里 `cargo test` 全绿，覆盖核心业务路径

### 1.4 Redis（对应 `redis.md`）

- [ ] 热点端点加缓存（如 issue 列表），带失效策略（写操作时清缓存）
- [ ] 基于 Redis 的限流中间件（固定窗口或滑动窗口）
- [ ] refresh token / session 存 Redis 并支持主动吊销

### 1.5 后台任务（对应 `background_jobs.md`）

- [ ] 简单版：`tokio::spawn` + `mpsc` channel 的进程内 worker
  （场景：issue 被 @ 时发「通知」，用写日志/写表模拟发邮件）
- [ ] 加重试和优雅关闭（drain channel 后再退出）

### 1.6 可观测性（对应 `observability.md`）

- [ ] `tracing` 结构化日志：每个请求带 request_id，串起整条链路
- [ ] `/metrics` 端点（`metrics` + `metrics-exporter-prometheus`）
- [ ] 慢查询日志：sqlx 查询超过阈值时 warn

### 1.7 部署（对应 `deployment.md`）★ 一定要做到有公网 URL

- [ ] 多阶段 Dockerfile（builder + `distroless`/`debian-slim`，镜像 < 100MB）
- [ ] docker-compose：api + postgres + redis 一键起
- [ ] GitHub Actions CI：fmt check + clippy -D warnings + test
- [ ] 部署到 Fly.io / Railway / 任意免费平台，拿到可访问的 URL
- [ ] 健康检查、优雅关闭（`graceful_shutdown` + 信号处理）验证通过

**Phase 1 完成标准**：一个公网可访问、带认证、有测试、有 CI、有监控端点的
API 服务。这就是一个合格的作品集项目。

---

## Phase 2：并发深水区（2-3 周）

CRUD API 几乎不碰 Rust 真正难的部分：并发下的所有权。三选一（推荐第一个）：

### 选项 A：WebSocket 聊天服务（推荐）

- axum WebSocket + `tokio::sync::broadcast` 多房间广播
- 共享状态：`Arc<RwLock<HashMap<RoomId, Room>>>` vs actor 模式（channel per room），两种都实现、对比
- 连接生命周期：心跳、断线清理、优雅关闭
- 考验点：`select!` 循环、任务取消、背压

### 选项 B：带持久化的任务队列

- Redis（或 Postgres `SELECT ... FOR UPDATE SKIP LOCKED`）做队列
- worker pool、失败重试（指数退避）、死信队列、优雅 drain
- 考验点：`JoinSet`、`Semaphore` 限并发、任务幂等

### 选项 C：并发网络爬虫

- `reqwest` + `Semaphore` 有界并发、URL 去重、按域名限速
- 考验点：`JoinSet` 动态任务、共享去重集合、错误聚合

---

## Phase 3：走出自己的仓库（长期，与 Phase 1/2 并行）

### 3.1 读生产级源码

- [ ] 精读 `tower::Service` trait 及一两个 Layer 实现（Rust API 设计的教科书）
- [ ] 读 `thiserror` 或 `axum` extractor 的宏/trait 实现，理解「为什么这样设计」
- 方法：每周挑一个「这玩意儿怎么实现的？」问题，顺着源码找答案，记进 markdown/

### 3.2 开源贡献

- [ ] 在常用 crate（axum / sqlx / utoipa / rustlings…）里找 `good-first-issue`
- [ ] 先从文档改进、测试补充这类低门槛 PR 开始
- 目的：接受 maintainer 的 code review，这是学 idiomatic Rust 最快的路径

### 3.3 底层/系统方向（可选，按兴趣）

- [ ] CodeCrafters「Build your own Redis」：字节级协议解析 + 并发服务器
- [ ] 或用 Rust 写解释器（Crafting Interpreters / *Writing An Interpreter In Go* 的 Rust 版）
- 价值：接触 Web 开发碰不到的领域 —— 手写 parser、协议、无 GC 的数据结构

---

## 刻意不做的事

- ❌ 再写新的概念笔记 / 整理更多 markdown —— 文档已经足够多，缺的是实践
- ❌ 同时开多个新项目 —— 集中把 issue_tracker_api 做深，好过三个半成品
- ❌ 过早钻 unsafe / 宏 / no_std —— 等 Phase 3 读源码时自然会遇到，带着问题学

## 进度记录

| 日期 | 完成内容 |
|------|----------|
| 2026-07-10 | 制定本计划 |
