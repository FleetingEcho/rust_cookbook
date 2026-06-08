# Rust 设计模式 × TypeScript 对比

每个模式一对文件，左边 Rust 右边 TS，`cargo run --bin <name>` 直接看输出。

---

## 如何运行

```bash
cargo run --bin builder
cargo run --bin observer
# 以此类推
```

---

## 创建型

### Builder — 建造者
[builder.rs](builder.rs) · [builder.ts](builder.ts)

**工程用处：** 构造参数超过 3-4 个的对象时必用。最典型的场景是 HTTP 客户端配置（reqwest、axios）、数据库查询构造（QueryBuilder）、测试夹具（Test Fixture）生成。比起一个参数列表很长的构造函数，Builder 让调用方只设置关心的字段，其余走默认值，可读性大幅提升。Rust 标准库的 `std::process::Command` 就是经典 Builder。

---

### Singleton — 单例
[singleton.rs](singleton.rs) · [singleton.ts](singleton.ts)

**工程用处：** 全局配置、数据库连接池、日志系统。这些东西初始化一次就够，重复创建浪费资源甚至导致状态不一致。Rust 用 `OnceLock` / `LazyLock` 实现，天然线程安全；TS 的静态变量在 Node.js 单进程里没问题，但在 Worker 线程中要小心。

---

### Factory — 工厂
[factory.rs](factory.rs) · [factory.ts](factory.ts)

**工程用处：** 根据配置或运行时参数决定用哪种实现，调用方不关心具体类型。常见于日志后端（输出到 console / 文件 / 远程服务）、存储驱动（本地磁盘 / S3 / 内存）、支付渠道（支付宝 / 微信 / Stripe）。依赖注入框架底层大量使用工厂模式。

---

## 结构型

### Newtype — 新类型
[newtype.rs](newtype.rs) · [newtype.ts](newtype.ts)

**工程用处：** 防止"同类型但不同含义"的值被误传。`UserId` 和 `OrderId` 都是字符串，但不应该互换——Newtype 让编译器帮你检查。在金融系统里尤其重要：`USD`、`CNY`、`BTC` 都是 `f64`，但混用是灾难。Rust 的 Newtype 是零成本的，编译后和裸类型完全一样。

---

### Decorator — 装饰器
[decorator.rs](decorator.rs) · [decorator.ts](decorator.ts)

**工程用处：** 在不修改原有代码的前提下叠加功能。HTTP 中间件是最典型的装饰器：认证、限流、日志、压缩各自独立，按需组合。I/O 流的包装也是（读缓冲、加密、压缩层层套）。TS 里 `@decorator` 语法糖本质也是这个模式。

---

### Adapter — 适配器
[adapter.rs](adapter.rs) · [adapter.ts](adapter.ts)

**工程用处：** 接入第三方库或遗留系统时，对方的接口和你的系统不匹配，但你又不能改对方代码。常见于支付 SDK 对接、旧版 API 迁移、跨语言 FFI 封装。也是依赖倒置原则的实现手段——业务代码依赖自己定义的接口，Adapter 负责适配外部实现。

---

## 行为型

### Strategy — 策略
[strategy.rs](strategy.rs) · [strategy.ts](strategy.ts)

**工程用处：** 同一操作有多种算法，运行时或配置时决定用哪个。排序算法、压缩算法、路由策略、定价策略（普通用户 / VIP / 企业）都是典型场景。比 `if/else` 或 `switch` 更容易扩展，新增一种策略不需要改已有代码，符合开闭原则。

---

### Observer — 观察者
[observer.rs](observer.rs) · [observer.ts](observer.ts)

**工程用处：** 事件系统的核心模式。前端的 DOM 事件、Node.js 的 `EventEmitter`、后端的消息总线（用户注册后触发发邮件、写日志、更新统计）都是观察者。解耦生产者和消费者，新增消费者不需要修改触发方代码。微服务里的领域事件（Domain Event）也是这个思路。

---

### State — 状态机
[state.rs](state.rs) · [state.ts](state.ts)

**工程用处：** 对象行为强依赖当前状态时使用。订单状态（待支付→已支付→已发货→已收货）、TCP 连接状态、审批流、游戏角色状态（站立/跑动/跳跃/死亡）都是典型场景。用状态机替代一堆 `if status === 'xxx'` 的判断，让非法转换在代码结构上就无法发生，而不是靠运行时 throw。

---

### Iterator — 迭代器
[iterator.rs](iterator.rs) · [iterator.ts](iterator.ts)

**工程用处：** 几乎无处不在。自定义集合类型、分页查询（每次 `next()` 拉一页）、流式处理大文件（避免一次性加载进内存）、数据管道都依赖迭代器。Rust 的 Iterator 是惰性的，`filter().map().take()` 整个链在 `collect()` 之前不执行任何计算，适合处理大数据集。

---

### Command — 命令
[command.rs](command.rs) · [command.ts](command.ts)

**工程用处：** 把操作变成可传递的对象，从而支持撤销/重做（文本编辑器、图形编辑器）、操作队列（任务队列、批处理）、操作日志（数据库 WAL、事件溯源 Event Sourcing）。前端表单的"撤销上一步"、CI/CD 的操作回滚都是这个模式。

---

## Rust 特有

### RAII — 资源获取即初始化
[raii.rs](raii.rs) · [raii.ts](raii.ts)

**工程用处：** Rust 中资源管理的基础机制，不是选择用不用，而是时刻都在用。数据库连接池的 `acquire()`、Mutex 的 `lock()`、文件句柄——所有这些拿到手的都是 Guard，离开作用域自动归还/释放，不需要 `finally`，也不可能忘记。在 TS 里需要手写 `try/finally` 或 `using` 才能达到同等效果。

---

### Typestate — 类型状态
[typestate.rs](typestate.rs) · [typestate.ts](typestate.ts)

**工程用处：** 把"状态"编码进类型，让非法的状态转换在编译期就无法表达。未连接的客户端根本没有 `send()` 方法，而不是调用时 throw。常见于协议实现（TCP 握手流程）、构建器的必填字段校验（`Builder<WithUrl>` 才能调用 `build()`）、硬件驱动（引脚在输入模式没有写方法）。是 Rust 类型系统最有特色的用法之一，TS 无法完整复现。
