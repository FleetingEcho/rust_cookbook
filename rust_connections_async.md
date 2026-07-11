# Rust 知识串联（二）：异步与并发

> 前置阅读：`rust_connections.md`。那篇的公理是「值有唯一所有者，访问三式
> `T / &T / &mut T`」。本篇只做一件事：证明异步/并发里所有「玄学报错」——
> `future is not Send`、`does not live long enough`、`cannot be unpinned`——
> 都是同一套所有权规则在新场景下的自然推论，没有任何新魔法。
>
> 新场景只新在两点：**代码会暂停（.await），暂停后可能换一个线程继续跑。**
> 记住这两句，本篇所有内容都能自己推出来。

---

## 0. async fn 的真身：一个编译器生成的 enum 状态机

这是本篇的地基，先看透它：

```rust
async fn fetch_user(id: u64) -> User {
    let conn = connect().await;      // 暂停点 1
    let user = conn.query(id).await; // 暂停点 2
    user
}
```

编译器把它变成大致这样的东西：

```rust
// 你写的 async fn，实际返回一个实现了 Future 的匿名类型
fn fetch_user(id: u64) -> impl Future<Output = User>

// 这个匿名类型本质是个 enum 状态机——每个 .await 是一个状态：
enum FetchUserFuture {
    Start { id: u64 },
    WaitingConnect { id: u64, fut: ConnectFuture },       // 停在暂停点 1
    WaitingQuery { conn: Conn, fut: QueryFuture },        // 停在暂停点 2
    Done,
}
```

**恍然大悟点 #1**：`async fn` 不执行任何东西，它只是**构造一个 enum**。
每个变体保存「停在这个 .await 时，后面还要用到的所有局部变量」。
这一句话是本篇一半问题的答案，后面反复回来引用它。

三个直接推论：

1. **Future 是惰性的**。构造 enum 当然什么都不发生——
   不 `.await`、不 `spawn`，它就是一个躺着的值。
   （对比你熟悉的 TypeScript：`Promise` 创建即执行，Rust 的 Future 不是 Promise，
   更像一个还没调用的 generator。）
2. **需要执行器**。enum 自己不会动，要有人反复调用它的 `poll()` 推它前进——
   这就是 tokio 存在的理由。`#[tokio::main]` 只是「起一个 poll 循环」的糖。
3. **取消 = drop**。停止一个 future 不需要什么取消 API，把这个 enum 值
   drop 掉就行——这就是所有权体系里最普通的析构。`select!` 分支输了、
   `timeout` 超时了，都是把 future drop 掉而已。RAII 在异步世界依然是主角。

---

## 1. Send / Sync：不是新规则，是三元组的跨线程版

先给定义，再串联：

| trait | 含义 | 对应三元组 |
|-------|------|-----------|
| `Send` | 值可以**移动**到另一个线程 | `T` 的跨线程版 |
| `Sync` | 值可以被多个线程**同时借用**（`&T` 可跨线程） | `&T` 的跨线程版 |

**恍然大悟点 #2**：`Send/Sync` 就是「拿走」和「借来看」这两种访问方式
加上一个问题：「换个线程还安全吗？」它们不是你实现的，是**自动 trait**——
编译器看你的 struct 的每个字段：全是 Send，整体就是 Send；有一个不是，就不是。
和 `Drop` 的递归析构、move 的递归转移一个逻辑：**性质沿组合传播**。

哪些东西不是 Send/Sync？把上一篇的智能指针表拿来，答案自动浮现：

| 类型 | Send? | Sync? | 为什么（用已有知识推） |
|------|-------|-------|------------------------|
| `Rc<T>` | ❌ | ❌ | 引用计数是普通整数，两个线程同时 +1 会丢计数 |
| `Arc<T>` | ✅ | ✅ | 计数换成原子操作，就为这个 A（atomic）字 |
| `RefCell<T>` | ✅ | ❌ | 借用计数也是普通整数，同时 borrow 会乱 |
| `Mutex<T>` | ✅ | ✅ | 锁保证同时只有一个访问者 |
| `MutexGuard` | ❌ | — | 锁必须在加锁的线程解，guard 不能漂移到别的线程 |

**上一篇第 6 章的表格现在可以「证明」了**：为什么跨线程必须
`Rc→Arc`、`RefCell→Mutex`？不是背出来的搭配——是 `Rc` 和 `RefCell`
根本不是 Send/Sync，编译器直接拒绝，你只能换成原子/加锁版本。
那张表不是最佳实践，是唯一解。

---

## 2. 为什么 `tokio::spawn` 要求 `Send + 'static`

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,   // ← 这两个 bound 就是本章全部内容
```

用「新场景两句话」直接推导：

- **任务可能换线程执行** → 这个 future（那个 enum 状态机）会被搬到别的线程
  → 必须 `Send`。
- **任务可能比创建它的函数活得久**（spawn 完不 await，函数就返回了）
  → future 里不能藏着任何「借来的、会先死的东西」→ 必须 `'static`。

**恍然大悟点 #3**：`'static` 不是「活到程序结束」，而是
**「不借任何人的东西」——要么拥有所有权，要么只含 `'static` 引用**。
`String` 是 `'static` 的，`&'a str` 不是。所以这条 bound 的人话翻译是：
「交给我的任务，请把它需要的东西**都让它自己拥有**」。

这直接解释了两个你一定会遇到的日常模式：

```rust
// 为什么 spawn 前总要 clone？—— 让任务拥有自己的一份
let db = state.db.clone();               // Arc clone，只是计数 +1
tokio::spawn(async move {                // 为什么总是 async move？
    db.query(…).await;                   //   —— move = 把捕获从借用改为拿走
});
```

`async move` 的 `move` 和闭包的 `move` 是同一个字、同一个含义
（上一篇 1.2 节）：把捕获方式从 `&T` 改成 `T`。串起来了。

---

## 3. 最经典的报错：「future is not Send」

```
error: future cannot be sent between threads safely
note: future is not `Send` as this value is used across an await
```

现在你有全部零件，可以自己组装出这个报错的成因：

1. 第 0 章：状态机的每个变体保存「**跨过这个 .await 还活着的局部变量**」。
2. 第 1 章：struct 有一个字段不是 Send，整体就不是 Send。
3. 第 2 章：spawn 要求 future 是 Send。

所以：**任何非 Send 的值，只要活着跨过一个 `.await`，
它就成了状态机的字段，整个 future 就不是 Send，spawn 直接报错。**

```rust
tokio::spawn(async {
    let data = Rc::new(vec![1]);     // Rc 不是 Send
    something().await;               // ❌ data 活着跨过了 await → 进了状态机
    println!("{:?}", data);
});

tokio::spawn(async {
    {
        let data = Rc::new(vec![1]);
        println!("{:?}", data);
    }                                // ✅ data 在 await 前就死了 → 不进状态机
    something().await;
});
```

修法从原理直接推出，按优先级：

1. **缩短生命周期**：让非 Send 值在 `.await` 前死掉（加一层 `{}` 或提前 drop）；
2. **换等价的 Send 类型**：`Rc→Arc`、`RefCell→Mutex`（第 1 章的表）;
3. 真的绕不开，用 `tokio::task::spawn_local`（放弃换线程的能力换取免 Send）。

**恍然大悟点 #4**：这个报错不玄学——报错信息里那句
`used across an await` 就是在说「它进了状态机 enum」。
读懂第 0 章，这类报错永远能自己定位。

---

## 4. 同一个原理的第二次爆发：锁与 .await

```rust
let guard = state.lock().unwrap();   // std::sync::MutexGuard
do_something().await;                // ❌ guard 跨过 await
```

两个独立的问题在这里同时发生，值得分开看清：

- **编译期问题**：`MutexGuard` 不是 Send（锁必须在原线程解），
  跨过 await 进了状态机 → 第 3 章的报错，原样复现。
- **运行期问题（更隐蔽）**：`.await` 是把线程让出去跑别的任务；
  如果别的任务也要这把锁，而当前任务停着不放——**死锁**。
  同一个线程既拿着锁，又在等待队列里排队。

修法优先级（和第 3 章同构）：

```rust
// 1. 最好：缩小锁的作用域，await 前放锁 —— 「缩短生命周期」
let value = { state.lock().unwrap().clone() };   // guard 在这行结束就死了
do_something(value).await;

// 2. 次之：换 tokio::sync::Mutex —— 「换等价的异步版类型」
//    它的 guard 是 Send 的，lock().await 等锁时也会让出线程
let guard = state.lock().await;
do_something(&guard).await;                       // 合法，但锁被拿着更久
```

**恍然大悟点 #5**：`std::Mutex` vs `tokio::Mutex` 的选择不用背：
锁内没有 `.await` → 用 std 的（更快）；锁必须跨 `.await` → 才用 tokio 的。
判断标准就一条：guard 会不会成为状态机的字段。

同理还有一个运行期陷阱同属此章：**在 async 里做阻塞调用**
（`std::thread::sleep`、同步 IO、重 CPU 计算）。async 是协作式调度——
你不 `.await`，就永远不让出线程，同线程的其他任务全部饿死。
修法：`tokio::time::sleep(...).await` 替代线程 sleep，
重计算/同步 IO 丢给 `tokio::task::spawn_blocking`。

---

## 5. Pin：状态机自引用的善后（知道为什么存在即可）

第 0 章的状态机有个隐患：变体里可能同时存着 `conn` 和「借用了 conn 的
future」——**结构体的一个字段指向另一个字段**（自引用）。
而 Rust 的 move 是浅拷贝内存：一 move，指向旧地址的内部引用就悬空了。

所以规则是：**开始 poll 之后，这个状态机不许再被 move**。
`Pin` 就是把这个承诺写成类型：`Pin<&mut F>` 意思是「我保证 F 钉死不动了」。

日常你几乎不手写 Pin，只需要认识它的两个出没地点：

- `Box::pin(fut)`：future 太大或类型写不出时钉到堆上
  （`async fn` 返回的匿名类型没名字，存 struct 字段时常用
  `Pin<Box<dyn Future<Output = T> + Send>>`——每个词现在你都认识了）；
- `tokio::pin!(fut)`：在 `select!` 循环里反复 poll 同一个 future 时钉在栈上。

**恍然大悟点 #6**：Pin 不是异步专属的新概念，它是「move = 浅拷贝内存」
这条老规则撞上「状态机可能自引用」之后，必然要打的补丁。

---

## 6. Channel 选型：又是那几个老问题

上一篇智能指针用三个正交问题定位，channel 同样如此：
**几个生产者？几个消费者？消费者错过消息要不要紧？**

| channel | 生产者→消费者 | 语义 | 典型场景 |
|---------|--------------|------|----------|
| `tokio::sync::mpsc` | 多 → 1 | 队列，每条消息恰好被消费一次 | 任务队列、actor 收件箱 |
| `oneshot` | 1 → 1 | 只发一次 | 请求-应答、拿任务结果 |
| `broadcast` | 多 → 多 | 每个消费者都收到每条 | 聊天室、事件广播 |
| `watch` | 1 → 多 | 只保留最新值，中间的会跳过 | 配置热更新、状态订阅 |

串联点一：**channel 是所有权的搬运工**。`send(msg)` 把 msg 的所有权
move 给接收方——这就是为什么消息传递天然没有数据竞争，
「不要用共享内存来通信，用通信来共享内存」在 Rust 里是类型系统保证的。

串联点二：**`Sender` clone、`Receiver` 不能 clone（mpsc）**，
正是「多生产者单消费者」写进了类型——数一数 `Arc` 和 `&mut` 的味道。

串联点三：actor 模式（一个任务独占状态 + mpsc 收件箱）和
`Arc<Mutex<T>>` 是同一个问题的两个解：**共享可变状态**。
锁 = 大家排队进房间改；actor = 只有管家能进房间，大家递纸条。
你 plan.md Phase 2 的聊天服务器要求两种都写，写完这个对比就有体感了。

---

## 7. 结构化收尾：spawn 家族与取消

| 工具 | 干什么 | 关键点 |
|------|--------|--------|
| `tokio::spawn` | 丢出去独立跑 | 返回 JoinHandle；**drop handle 任务照跑**（detach） |
| `JoinSet` | 管一批任务 | drop JoinSet = **取消所有任务**；天然限并发配 Semaphore |
| `select!` | 几个 future 赛跑 | 输的分支被 drop = 被取消（第 0 章推论 3） |
| `join!` | 几个都要完成 | 并发等待，全完才继续 |

**恍然大悟点 #7**：Rust 异步没有独立的「取消」概念——
取消就是 drop，drop 就走 RAII 析构。所以「优雅关闭」的通用套路是：
关闭信号（`watch` 或 `CancellationToken`）+ `select!`（任务里同时等
「正事」和「关闭信号」）+ drop 触发清理。三个你已认识的零件拼出来的。

反过来这也是个坑：future 在任何 `.await` 处都可能被取消（drop），
所以「先写文件、再更新索引」这种两步操作若中间有 await，
要考虑只完成一半的情况——这叫**取消安全**，面试和生产都会遇到。

---

## 8. 总图

```
            ┌────────────────────────────────────────┐
            │ 新场景两句话：代码会暂停(.await)；      │
            │ 暂停后可能换线程继续跑                  │
            └───────────────────┬────────────────────┘
                                ▼
     async fn = enum 状态机；变体字段 = 跨 await 存活的局部变量
       │ 惰性、需 executor、取消 = drop（RAII 延续）
       │
       ├── 换线程 ⇒ Send/Sync（三元组 T/&T 的跨线程版，自动 trait 沿字段传播）
       │      └─ Rc/RefCell 不过关 ⇒ Arc/Mutex 是唯一解（上一篇表格的证明）
       │
       ├── 任务比创建者活得久 ⇒ 'static（= 不借别人东西 ⇒ clone + async move）
       │
       ├── 非 Send 值跨 await 进状态机 ⇒ "future is not Send"
       │      └─ 同构问题：MutexGuard 跨 await ⇒ 编译错/死锁 ⇒ 缩短guard或换tokio锁
       │
       ├── 状态机自引用 + move 浅拷贝 ⇒ Pin（Box::pin / tokio::pin!）
       │
       ├── 跨任务传数据 ⇒ channel（send = move 所有权；四种channel按生产/消费数选）
       │
       └── 一批任务的生死 ⇒ spawn/JoinSet/select!（取消=drop ⇒ 取消安全）
```

## 9. 自测

1. 为什么 Rust 的 Future 不 `.await` 就什么都不发生，而 JS 的 Promise 创建即执行？
2. `Rc<T>` 为什么不是 Send？把 Rc 换成 Arc 到底换掉了什么？
3. `tokio::spawn` 的 `'static` bound 的人话翻译是什么？为什么 spawn 前总要 clone？
4. 「future is not Send」的报错里 `used across an await` 这半句在说什么？
5. 一个非 Send 的值出现在 async fn 里，什么情况下**不会**导致报错？
6. `std::Mutex` 和 `tokio::Mutex` 怎么选？判断标准是哪一条？
7. 在 async 任务里调 `std::thread::sleep` 会发生什么？为什么？
8. `select!` 里输掉的那个分支去哪了？这和 RAII 有什么关系？
9. actor + mpsc 和 `Arc<Mutex<T>>` 在解决同一个什么问题？各自的比喻是什么？
10. 什么叫取消安全？为什么「任何 .await 处都可能被取消」？

答不上来的题号 → 对应章节：0 / 1 / 2 / 3 / 3 / 4 / 4 / 7 / 6 / 7。
