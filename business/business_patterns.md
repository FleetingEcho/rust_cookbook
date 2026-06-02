# 业务代码设计模式

> Rust 的类型系统比大多数语言更强大，用好它可以让编译器帮你杜绝业务逻辑错误。
> 本文介绍在业务开发中最实用的几种模式。

---

## 一、Newtype：让类型不再混用

**问题**：所有 ID 都是 `i64`，函数接受 `user_id`，却可能传入 `order_id`，编译器不报错。

```rust
// ❌ 危险：三个参数都是 i64，传错了编译器不知道
async fn transfer_funds(
    from_user_id: i64,
    to_user_id:   i64,
    account_id:   i64,
    amount:       i64,
) -> Result<(), AppError> { ... }

// 调用方容易写错
transfer_funds(account_id, from_user_id, to_user_id, amount).await?;
```

```rust
// ✅ Newtype：每种 ID 是独立的类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cents(pub i64);  // 金额用整数分，避免浮点精度问题

// 现在参数类型不同，传错会直接编译报错
async fn transfer_funds(
    from_user: UserId,
    to_user:   UserId,
    account:   AccountId,
    amount:    Cents,
) -> Result<(), AppError> { ... }

// 使用
transfer_funds(
    UserId(1),
    UserId(2),
    AccountId(100),
    Cents(5000),    // 50.00 元
).await?;
```

### 为 Newtype 实现常用操作

```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub i64);

// Display：直接打印 ID 值
impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// From/Into：与底层类型互转
impl From<i64> for UserId {
    fn from(id: i64) -> Self { UserId(id) }
}
impl From<UserId> for i64 {
    fn from(id: UserId) -> i64 { id.0 }
}

// sqlx 支持：直接在 SQL 中使用（需要 sqlx feature）
// 方式一：传值时解包
sqlx::query!("SELECT * FROM users WHERE id = $1", user_id.0).fetch_one(&pool);
// 方式二：实现 sqlx::Type（更优雅，一劳永逸）

// Serde：序列化直接输出数字
impl Serialize for UserId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(self.0)
    }
}
impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(UserId(i64::deserialize(d)?))
    }
}
```

---

## 二、Type-State 模式：用类型强制状态流转

**问题**：订单有 Pending → Paid → Shipped → Delivered 的状态，如何防止状态非法跳转？

```rust
// ❌ 用字符串/枚举字段表示状态：运行时检查，容易遗漏
struct Order {
    id:     OrderId,
    status: String,    // "pending" / "paid" / "shipped"
}

impl Order {
    fn ship(&mut self) -> Result<(), String> {
        if self.status != "paid" {
            return Err("只有已支付的订单才能发货".into());
        }
        self.status = "shipped".to_string();
        Ok(())
    }
}
// ship 可以在任何 Order 上调用，错误只在运行时发现
```

```rust
// ✅ Type-State：状态编码进类型，编译器保证合法
use std::marker::PhantomData;

// 状态标记类型（零大小，只在类型层面存在）
pub struct Pending;
pub struct Paid;
pub struct Shipped;
pub struct Delivered;

// 订单结构体：S 是状态类型参数
pub struct Order<S> {
    pub id:         OrderId,
    pub user_id:    UserId,
    pub amount:     Cents,
    pub items:      Vec<OrderItem>,
    _state: PhantomData<S>,   // 不占内存，只携带类型信息
}

// 每种状态各自的额外数据
pub struct PendingOrder {
    pub order: Order<Pending>,
}

pub struct PaidOrder {
    pub order:      Order<Paid>,
    pub paid_at:    chrono::DateTime<chrono::Utc>,
    pub payment_id: String,
}

pub struct ShippedOrder {
    pub order:       Order<Shipped>,
    pub tracking_id: String,
    pub shipped_at:  chrono::DateTime<chrono::Utc>,
}

// 状态转换函数（只在特定状态上存在）
impl PendingOrder {
    pub fn new(user_id: UserId, items: Vec<OrderItem>) -> Self {
        let amount = items.iter().map(|i| i.price.0).sum();
        PendingOrder {
            order: Order {
                id: OrderId(uuid_to_i64()),
                user_id, amount: Cents(amount), items,
                _state: PhantomData,
            }
        }
    }

    // 只有 Pending 订单才能付款
    pub fn pay(self, payment_id: String) -> PaidOrder {
        PaidOrder {
            order: Order { _state: PhantomData, ..self.order },
            paid_at:    chrono::Utc::now(),
            payment_id,
        }
    }

    // 只有 Pending 订单才能取消
    pub fn cancel(self) -> CancelledOrder { todo!() }
}

impl PaidOrder {
    // 只有 Paid 订单才能发货
    pub fn ship(self, tracking_id: String) -> ShippedOrder {
        ShippedOrder {
            order: Order { _state: PhantomData, ..self.order },
            tracking_id,
            shipped_at: chrono::Utc::now(),
        }
    }
}

// 使用：非法转换在编译期就被拦截
let order = PendingOrder::new(UserId(1), items);
// order.ship(...)         // ❌ 编译错误：PendingOrder 没有 ship 方法
let paid  = order.pay("pay_abc123".to_string());
// paid.cancel()           // ❌ 编译错误：PaidOrder 没有 cancel 方法
let shipped = paid.ship("SF1234567".to_string());
```

---

## 三、Builder 模式：构造复杂对象

```rust
// 场景：创建用户时有很多可选字段，构造函数参数爆炸
// ❌ 参数太多，调用方需要记住顺序
fn create_user(
    username: &str, email: &str, password: &str,
    bio: Option<&str>, avatar: Option<&str>,
    role: Role, active: bool, max_sessions: u32,
) -> User { ... }
```

```rust
// ✅ Builder 模式
#[derive(Default)]
pub struct CreateUserRequest {
    pub username:     String,
    pub email:        String,
    pub password:     String,
    pub bio:          Option<String>,
    pub avatar_url:   Option<String>,
    pub role:         Role,
    pub active:       bool,
    pub max_sessions: u32,
}

pub struct CreateUserRequestBuilder {
    username:     Option<String>,
    email:        Option<String>,
    password:     Option<String>,
    bio:          Option<String>,
    avatar_url:   Option<String>,
    role:         Role,
    active:       bool,
    max_sessions: u32,
}

impl CreateUserRequestBuilder {
    pub fn new() -> Self {
        Self {
            username: None, email: None, password: None,
            bio: None, avatar_url: None,
            role: Role::User, active: true, max_sessions: 5,
        }
    }

    // 每个方法返回 Self，支持链式调用
    pub fn username(mut self, v: impl Into<String>) -> Self {
        self.username = Some(v.into()); self
    }
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.email = Some(v.into()); self
    }
    pub fn password(mut self, v: impl Into<String>) -> Self {
        self.password = Some(v.into()); self
    }
    pub fn bio(mut self, v: impl Into<String>) -> Self {
        self.bio = Some(v.into()); self
    }
    pub fn role(mut self, v: Role) -> Self { self.role = v; self }
    pub fn inactive(mut self) -> Self { self.active = false; self }

    // build：验证必填字段
    pub fn build(self) -> Result<CreateUserRequest, String> {
        Ok(CreateUserRequest {
            username:     self.username.ok_or("username 必填")?,
            email:        self.email.ok_or("email 必填")?,
            password:     self.password.ok_or("password 必填")?,
            bio:          self.bio,
            avatar_url:   self.avatar_url,
            role:         self.role,
            active:       self.active,
            max_sessions: self.max_sessions,
        })
    }
}

// 使用（可读性大幅提升）
let req = CreateUserRequestBuilder::new()
    .username("alice")
    .email("alice@example.com")
    .password("secret123")
    .role(Role::Admin)
    .bio("Rust 爱好者")
    .build()?;
```

> 实际项目中可以用 `bon` 或 `derive_builder` crate 自动生成 Builder，省去手写样板代码。

---

## 四、用 enum 建模业务状态机

```rust
// 用 enum 表达"只能是有限状态之一"的概念
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Processing { gateway_ref: String },
    Succeeded  { transaction_id: String, settled_at: chrono::DateTime<chrono::Utc> },
    Failed     { reason: String, retry_count: u32 },
    Refunded   { refund_id: String },
}

impl PaymentStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Succeeded { .. } | Self::Failed { retry_count, .. } if *retry_count >= 3 | Self::Refunded { .. })
    }

    pub fn can_retry(&self) -> bool {
        matches!(self, Self::Failed { retry_count, .. } if *retry_count < 3)
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Pending             => "等待处理",
            Self::Processing { .. }   => "处理中",
            Self::Succeeded  { .. }   => "支付成功",
            Self::Failed     { .. }   => "支付失败",
            Self::Refunded   { .. }   => "已退款",
        }
    }
}

// 状态转换（业务规则集中在这里）
pub fn transition_payment(
    current: PaymentStatus,
    event: PaymentEvent,
) -> Result<PaymentStatus, String> {
    match (current, event) {
        (PaymentStatus::Pending, PaymentEvent::StartProcessing { gateway_ref }) => {
            Ok(PaymentStatus::Processing { gateway_ref })
        }
        (PaymentStatus::Processing { .. }, PaymentEvent::Succeed { transaction_id }) => {
            Ok(PaymentStatus::Succeeded { transaction_id, settled_at: chrono::Utc::now() })
        }
        (PaymentStatus::Processing { .. }, PaymentEvent::Fail { reason, retry_count }) => {
            Ok(PaymentStatus::Failed { reason, retry_count })
        }
        (current, event) => {
            Err(format!("非法状态转换: {:?} + {:?}", current, event))
        }
    }
}
```

---

## 五、避免"坏类型"

### 5.1 避免 Option\<Option\<T\>\>

```rust
// ❌ 难以理解和使用
struct UserProfile {
    bio: Option<Option<String>>,  // None=没改, Some(None)=清空, Some(Some(v))=更新
}

// ✅ 用显式枚举表达意图
#[derive(Deserialize)]
#[serde(untagged)]
pub enum Patch<T> {
    Missing,           // 字段未传（保持原值）
    Null,              // 字段传了 null（清空）
    Value(T),          // 字段有值（更新）
}

impl<T> Default for Patch<T> {
    fn default() -> Self { Patch::Missing }
}

#[derive(Deserialize)]
struct UpdateProfileRequest {
    #[serde(default)]
    bio:        Patch<String>,
    #[serde(default)]
    avatar_url: Patch<String>,
}

// 使用
match req.bio {
    Patch::Missing  => { /* 不改 */ }
    Patch::Null     => { user.bio = None; }
    Patch::Value(v) => { user.bio = Some(v); }
}
```

### 5.2 避免 bool 参数爆炸

```rust
// ❌ 调用方不知道三个 bool 各是什么意思
send_email(user, true, false, true);

// ✅ 用结构体或 builder
send_email(user, EmailOptions {
    send_copy:    true,
    track_open:   false,
    high_priority: true,
});
```

### 5.3 用类型表达不变量

```rust
// ❌ 任何 String 都能传，但实际要求不能为空
fn create_user(username: String) { ... }

// ✅ 用验证过的类型保证不变量
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(s: impl Into<String>) -> Result<Self, &'static str> {
        let s = s.into();
        if s.trim().is_empty() {
            Err("字符串不能为空")
        } else {
            Ok(NonEmptyString(s))
        }
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

// 一旦拿到 NonEmptyString，就无需在函数内部再判空
fn create_user(username: NonEmptyString) { ... }
```

---

## 六、领域对象设计原则

```rust
// 原则：让非法状态在类型层面不可表达

// ❌ 设计：User 同时含有多种状态的字段，大部分时候都是 None
struct User {
    id:              i64,
    username:        String,
    email:           String,
    email_verified:  bool,
    // 只有 email_verified=true 时才有意义
    verified_at:     Option<chrono::DateTime<Utc>>,
    // 只有被封禁时才有意义
    banned:          bool,
    ban_reason:      Option<String>,
    banned_until:    Option<chrono::DateTime<Utc>>,
}

// ✅ 设计：用 enum 把状态和相关数据绑在一起
pub struct User {
    pub id:       UserId,
    pub username: Username,
    pub email:    Email,
    pub status:   UserStatus,
}

#[derive(Debug, Clone)]
pub enum UserStatus {
    PendingVerification { token: String, expires_at: chrono::DateTime<Utc> },
    Active { verified_at: chrono::DateTime<Utc> },
    Banned { reason: String, until: Option<chrono::DateTime<Utc>> },
    Deleted { deleted_at: chrono::DateTime<Utc> },
}

impl UserStatus {
    pub fn is_active(&self) -> bool { matches!(self, Self::Active { .. }) }
    pub fn can_login(&self) -> bool {
        match self {
            Self::Active { .. } => true,
            Self::Banned { until: Some(t), .. } => chrono::Utc::now() > *t,
            _ => false,
        }
    }
}
```

---

## 七、Repository 模式（接口与实现分离）

```rust
// 定义 Repository trait（接口）
// 好处：Service 层依赖 trait，测试时可 mock，生产时换不同实现
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn save(&self, user: &User) -> Result<User, AppError>;
    async fn delete(&self, id: UserId) -> Result<(), AppError>;
}

// 真实实现（PostgreSQL）
pub struct PgUserRepository { pool: PgPool }

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            UserRow,
            "SELECT * FROM users WHERE id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map(|row| row.map(User::from))
        .map_err(AppError::from)
    }

    async fn save(&self, user: &User) -> Result<User, AppError> {
        // upsert
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> { todo!() }
    async fn delete(&self, id: UserId) -> Result<(), AppError> { todo!() }
}

// Service 层：依赖 trait，不依赖具体实现
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self { Self { repo } }

    pub async fn get_user(&self, id: UserId) -> Result<User, AppError> {
        self.repo.find_by_id(id)
            .await?
            .ok_or(AppError::UserNotFound { id: id.0 })
    }
}

// 组装（main.rs）
let repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository { pool: pool.clone() });
let service = UserService::new(repo);

// 测试时
// let mock_repo: Arc<dyn UserRepository> = Arc::new(MockUserRepository::new());
// let service = UserService::new(mock_repo);
```

---

## 速查表

```
Newtype(T)                    让类型不可混用，ID 用 Newtype 定义
PhantomData<S>                在类型层面携带状态，不占内存
Order<Pending> / Order<Paid>  Type-State：非法方法直接不存在
Builder::new().x().build()    构造复杂对象，明确必填/可选
enum Status { A, B(Data) }    状态机：状态和相关数据绑定
Patch<T>：Missing/Null/Value  PATCH API 的三种字段情况
NonEmptyString(String)        用类型表达不变量，不是 String 谁都能传
Arc<dyn Repository>           接口与实现分离，便于测试 mock
```
