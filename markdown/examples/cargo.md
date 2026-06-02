# Rust vs TypeScript: 包管理与项目结构

**工具对比：** `npm` / `package.json` → `cargo` / `Cargo.toml`

## TypeScript 版本

```json
// package.json
{
  "name": "my-app",
  "version": "1.0.0",
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "ts-node src/index.ts",
    "test": "jest"
  },
  "dependencies": {
    "express": "^4.18.0",
    "zod": "^3.22.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "@types/express": "^4.17.0",
    "jest": "^29.0.0"
  }
}
```

```bash
npm install           # 安装依赖（生成 node_modules）
npm run build         # 构建
npm test              # 测试
npx tsc --init        # 初始化 tsconfig
```

---

## 一、Cargo.toml 对应关系

```toml
# Cargo.toml — 对应 package.json
[package]
name = "my-app"           # 项目名
version = "0.1.0"         # 版本号
edition = "2021"          # Rust 版本（2015/2018/2021）
authors = ["Alice <alice@example.com>"]
description = "一个示例项目"

# 对应 dependencies（生产依赖）
[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = "0.12"

# 对应 devDependencies（开发依赖，只在 cargo test/bench 中使用）
[dev-dependencies]
criterion = "0.5"

# 构建脚本依赖
[build-dependencies]
cc = "1.0"
```

---

## 二、常用 Cargo 命令对照

| 操作 | npm | cargo |
|------|-----|-------|
| 初始化项目 | `npm init` | `cargo new my-app` |
| 初始化库 | — | `cargo new my-lib --lib` |
| 安装依赖 | `npm install` | `cargo build`（自动下载） |
| 添加依赖 | `npm install serde` | `cargo add serde` |
| 移除依赖 | `npm uninstall serde` | `cargo remove serde` |
| 构建 | `npm run build` | `cargo build` |
| 发布构建 | `npm run build` | `cargo build --release` |
| 运行 | `npm start` | `cargo run` |
| 测试 | `npm test` | `cargo test` |
| 格式化 | `prettier` | `cargo fmt` |
| 代码检查 | `eslint` | `cargo clippy` |
| 发布到 registry | `npm publish` | `cargo publish` |
| 查看依赖树 | `npm ls` | `cargo tree` |
| 更新依赖 | `npm update` | `cargo update` |
| 检查安全漏洞 | `npm audit` | `cargo audit`（需安装） |

---

## 三、版本号语法（SemVer）

```toml
[dependencies]
# 等价于 npm 的 ^ — 允许兼容性更新（主版本不变）
serde = "1"           # >= 1.0.0, < 2.0.0
serde = "1.0"         # >= 1.0.0, < 2.0.0（同上）
serde = "1.2.3"       # >= 1.2.3, < 2.0.0

# 精确版本（npm 的固定版本）
serde = "=1.2.3"

# 范围
serde = ">=1.0, <2.0"

# 通配符
serde = "1.*"
```

**Cargo.lock**：和 `package-lock.json` / `yarn.lock` 一样，锁定精确版本。
- 应用程序：**提交** Cargo.lock 到 git
- 库：**不提交** Cargo.lock（让调用者决定版本）

---

## 四、Features（条件依赖）

Features 是 Rust 独有的概念，比 npm 的 `optionalDependencies` 更强大。可以按需开启功能，减少编译时间和二进制大小。

```toml
[dependencies]
# 开启 serde 的 derive feature（启用 #[derive(Serialize)] 宏）
serde = { version = "1", features = ["derive"] }

# tokio 只开启需要的部分
tokio = { version = "1", features = ["rt", "net", "time"] }
# 或者全开（开发时方便）
tokio = { version = "1", features = ["full"] }

# 可选依赖：只在某些情况下需要
[dependencies]
serde_json = { version = "1", optional = true }

# 在自己的 Cargo.toml 中定义 features
[features]
default = []                          # 默认不开启任何额外 feature
json_support = ["dep:serde_json"]     # 使用 json_support feature 时才引入 serde_json
```

```bash
# 编译时开启特定 feature
cargo build --features "json_support"
cargo test --features "json_support,extra_logs"
```

---

## 五、项目结构

```
my-app/
├── Cargo.toml          # package.json
├── Cargo.lock          # package-lock.json
├── src/
│   ├── main.rs         # 二进制入口（对应 src/index.ts）
│   ├── lib.rs          # 库入口（如果同时提供库）
│   └── utils/
│       ├── mod.rs      # 模块声明（相当于 index.ts）
│       └── helper.rs
├── tests/
│   └── integration_test.rs  # 集成测试（cargo test 自动发现）
├── examples/
│   └── demo.rs         # 示例代码（cargo run --example demo）
├── benches/
│   └── benchmark.rs    # 性能测试（cargo bench）
└── build.rs            # 构建脚本（类似 webpack plugin）
```

---

## 六、Workspace（Monorepo）

对应 npm workspaces / turborepo：

```toml
# 根目录 Cargo.toml
[workspace]
members = [
    "crates/core",      # 核心库
    "crates/cli",       # CLI 工具
    "crates/server",    # Web 服务器
]
resolver = "2"

# 共享依赖版本（类似 npm workspaces 的 hoisting）
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

```toml
# crates/cli/Cargo.toml — 引用 workspace 中的版本
[dependencies]
serde = { workspace = true }          # 使用 workspace 定义的版本
tokio = { workspace = true }
core = { path = "../core" }           # 引用同 workspace 中的其他 crate
```

```bash
# Workspace 命令
cargo build -p cli          # 只构建 cli 这个 crate
cargo test -p core          # 只测试 core
cargo run -p cli -- --help  # 运行 cli 并传参数
cargo build --workspace     # 构建所有
```

---

## 七、本地 Crate 依赖

```toml
# 先用本地路径开发，发布后换成版本号
[dependencies]
my-utils = { path = "../my-utils" }

# 同时支持路径（本地开发）和版本（发布后）
my-utils = { path = "../my-utils", version = "0.1" }

# Git 依赖（对应 npm 的 git 依赖）
my-lib = { git = "https://github.com/user/my-lib", branch = "main" }
my-lib = { git = "https://github.com/user/my-lib", tag = "v0.2.0" }
my-lib = { git = "https://github.com/user/my-lib", rev = "abc1234" }
```

---

## 八、常用 Cargo 扩展工具

```bash
# 安装扩展工具（类似 npm 全局包）
cargo install cargo-watch    # 文件变化时自动重新运行（类似 nodemon）
cargo install cargo-edit     # 提供 cargo add/remove/upgrade
cargo install cargo-audit    # 安全漏洞扫描（类似 npm audit）
cargo install cargo-expand   # 展开宏，方便调试
cargo install cargo-flamegraph  # 生成火焰图，性能分析

# 使用示例
cargo watch -x run           # 相当于 nodemon
cargo watch -x test          # 文件变化时自动测试
```

---

## 关键差异速查

| 概念 | npm/TypeScript | Cargo/Rust |
|------|---------------|-----------|
| 配置文件 | `package.json` | `Cargo.toml` |
| 锁文件 | `package-lock.json` | `Cargo.lock` |
| 依赖存放 | `node_modules/` | `~/.cargo/registry/`（全局缓存） |
| 条件功能 | `optionalDependencies` | `features` |
| 构建输出 | `dist/` | `target/debug/` 或 `target/release/` |
| 代码检查 | `eslint` | `cargo clippy` |
| 格式化 | `prettier` | `cargo fmt` |
| Monorepo | workspaces | workspace |
| 热重载 | `nodemon` / `ts-node` | `cargo-watch` |
| 文档 | JSDoc / TypeDoc | `cargo doc` |
