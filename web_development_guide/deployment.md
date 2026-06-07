# 部署：Docker / docker-compose / CI

---

## 一、多阶段 Dockerfile（最终镜像 < 15MB）

```dockerfile
# ─────────────────────────────────────────
# 阶段 1：依赖缓存层（利用 Docker 层缓存加速构建）
# ─────────────────────────────────────────
FROM rust:1.81-slim AS deps

WORKDIR /app

# 只复制 Cargo 文件，先构建空项目缓存依赖
# 这样依赖层只在 Cargo.toml/Cargo.lock 变化时才重建
COPY Cargo.toml Cargo.lock ./

# 创建空 main.rs 触发依赖编译
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/my_app*   # 删掉空项目产物，保留依赖

# ─────────────────────────────────────────
# 阶段 2：构建
# ─────────────────────────────────────────
FROM deps AS builder

WORKDIR /app

# 复制源码（依赖已缓存，这层只有源码变化才重建）
COPY src ./src
COPY migrations ./migrations     # SQLx 迁移文件
COPY config ./config             # 默认配置文件

# 生产构建（开启所有优化）
ENV SQLX_OFFLINE=true            # 离线模式，不需要运行时数据库
RUN cargo build --release

# 压缩二进制（可选，需要 binutils）
RUN strip target/release/my_app

# ─────────────────────────────────────────
# 阶段 3：运行时镜像（最小化）
# ─────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# 安装运行时依赖（SSL 证书，如果要调用 HTTPS 接口）
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 非 root 用户运行（安全）
RUN useradd -m -u 1001 appuser
USER appuser

WORKDIR /app

# 从 builder 复制构建产物
COPY --from=builder /app/target/release/my_app   ./my_app
COPY --from=builder /app/migrations              ./migrations
COPY --from=builder /app/config                  ./config

EXPOSE 3000

# 健康检查（Docker 自动检查容器状态）
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget -qO- http://localhost:3000/health || exit 1

ENTRYPOINT ["./my_app"]
```

### 如果用 musl 实现静态链接（更小，无 OS 依赖）

```dockerfile
# 阶段 1：依赖
FROM rust:1.81-alpine AS deps
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/my_app*

# 阶段 2：构建
FROM deps AS builder
COPY src ./src
COPY migrations ./migrations
ENV SQLX_OFFLINE=true
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/my_app

# 阶段 3：从零开始（scratch）
FROM scratch AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my_app /my_app
COPY --from=builder /app/migrations /migrations
COPY --from=builder /app/config     /config
EXPOSE 3000
ENTRYPOINT ["/my_app"]
# 最终镜像只有一个二进制文件，约 5-10 MB
```

---

## 二、.dockerignore

```
target/
.git/
.gitignore
*.md
.env
.env.*
docker-compose*.yml
```

---

## 三、docker-compose（本地开发）

```yaml
# docker-compose.yml
version: "3.9"

services:
  app:
    build:
      context: .
      target: runtime          # 生产镜像；开发时可换 builder
    ports:
      - "3000:3000"
    environment:
      APP_ENV: dev
      DATABASE_URL: postgres://myapp:password@postgres:5432/myapp
      REDIS_URL: redis://redis:6379
      APP_JWT_SECRET: dev_jwt_secret_change_in_production
      APP_PORT: 3000
      RUST_LOG: debug,sqlx=warn
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "-qO-", "http://localhost:3000/health"]
      interval: 30s
      timeout: 5s
      retries: 3

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER:     myapp
      POSTGRES_PASSWORD: password
      POSTGRES_DB:       myapp
    ports:
      - "5432:5432"           # 暴露给本机调试用
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U myapp -d myapp"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    command: redis-server --maxmemory 256mb --maxmemory-policy allkeys-lru
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 3

volumes:
  postgres_data:
  redis_data:
```

```bash
# 常用命令
docker-compose up -d                   # 后台启动所有服务
docker-compose up -d postgres redis    # 只启动依赖，本机跑应用
docker-compose logs -f app             # 实时查看日志
docker-compose exec postgres psql -U myapp myapp  # 进入数据库
docker-compose down -v                 # 停止并删除数据卷（清空数据）
```

---

## 四、环境配置管理

```bash
# .env.example（提交到 git，不含真实值）
DATABASE_URL=postgres://user:password@localhost:5432/mydb
REDIS_URL=redis://localhost:6379
APP_PORT=3000
APP_JWT_SECRET=
APP_LOG_LEVEL=info
APP_ENV=dev
```

```bash
# .env（不提交 git，真实开发值）
DATABASE_URL=postgres://myapp:password@localhost:5432/myapp_dev
REDIS_URL=redis://localhost:6379
APP_PORT=3000
APP_JWT_SECRET=dev_secret_key_at_least_32_chars
APP_LOG_LEVEL=debug
APP_ENV=dev
```

```bash
# .gitignore
.env
.env.local
.env.production
```

---

## 五、GitHub Actions CI/CD

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  SQLX_OFFLINE: true            # 不连接数据库构建

jobs:
  # ─── 代码检查 ───
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: 安装 Rust 工具链
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: 缓存依赖
        uses: Swatinem/rust-cache@v2

      - name: 格式检查
        run: cargo fmt --check

      - name: Clippy 静态分析
        run: cargo clippy -- -D warnings    # 警告视为错误

  # ─── 测试 ───
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_USER:     test
          POSTGRES_PASSWORD: test
          POSTGRES_DB:       test
        ports: ["5432:5432"]
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7-alpine
        ports: ["6379:6379"]
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: 安装 sqlx-cli
        run: cargo install sqlx-cli --no-default-features --features postgres

      - name: 执行数据库迁移
        run: sqlx migrate run
        env:
          DATABASE_URL: postgres://test:test@localhost:5432/test

      - name: 运行测试
        run: cargo test
        env:
          DATABASE_URL: postgres://test:test@localhost:5432/test
          TEST_DATABASE_URL: postgres://test:test@localhost:5432/test
          REDIS_URL: redis://localhost:6379
          APP_JWT_SECRET: test_secret_at_least_32_chars

  # ─── 构建并推送 Docker 镜像 ───
  docker:
    needs: [check, test]
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'     # 只在 main 分支构建
    steps:
      - uses: actions/checkout@v4

      - name: 登录 Docker Registry
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: 构建并推送
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            ghcr.io/${{ github.repository }}:latest
            ghcr.io/${{ github.repository }}:${{ github.sha }}
          cache-from: type=gha
          cache-to:   type=gha,mode=max
```

---

## 六、Cargo.toml 生产优化

```toml
[profile.release]
opt-level    = 3          # 最高优化级别（默认就是 3）
lto          = "thin"     # 链接期优化（thin 比 fat 快，效果接近）
codegen-units = 1         # 单编译单元（更好的优化，编译更慢）
strip        = "symbols"  # 剥离调试符号（减小二进制大小）
panic        = "abort"    # panic 直接中止（不展开栈，更小）

# 依赖库用较低优化，应用代码用最高优化（权衡编译速度和性能）
[profile.release.package."*"]
opt-level = 2
```

---

## 七、环境变量完整清单模板

```bash
# ── 服务配置 ──
APP_ENV=production          # dev | staging | production
APP_HOST=0.0.0.0
APP_PORT=3000

# ── 数据库 ──
DATABASE_URL=postgres://user:pass@host:5432/dbname
DATABASE_MAX_CONNECTIONS=20
DATABASE_MIN_CONNECTIONS=2

# ── Redis ──
REDIS_URL=redis://:password@host:6379/0

# ── 认证 ──
APP_JWT_SECRET=至少32位随机字符串
APP_JWT_ACCESS_TTL=900         # 秒（15 分钟）
APP_JWT_REFRESH_TTL=604800     # 秒（7 天）

# ── 日志 ──
RUST_LOG=info,sqlx=warn
RUST_LOG_FORMAT=json           # text | json

# ── 功能开关 ──
APP_METRICS_ENABLED=true
APP_RATE_LIMIT_ENABLED=true
APP_RATE_LIMIT_MAX=100         # 每分钟最大请求数
```

---

## 速查表

```
构建：
  SQLX_OFFLINE=true cargo build --release     离线模式（CI 必用）
  cargo build --release --target x86_64-unknown-linux-musl  静态链接

Docker：
  FROM rust:1.81-slim AS builder              构建阶段（带编译器）
  FROM debian:bookworm-slim AS runtime        运行阶段（无编译器）
  COPY --from=builder /app/target/...         跨阶段复制
  strip target/release/my_app                 剥离符号，缩小体积
  USER appuser                                非 root 运行

docker-compose：
  depends_on: { service: { condition: service_healthy } }  等待依赖健康
  healthcheck: { test, interval, timeout, retries }        配置健康检查

CI/CD：
  SQLX_OFFLINE=true                           避免 CI 连接数据库
  Swatinem/rust-cache@v2                      缓存 Cargo 依赖
  cargo fmt --check                           格式检查
  cargo clippy -- -D warnings                 Lint（警告变错误）
```
