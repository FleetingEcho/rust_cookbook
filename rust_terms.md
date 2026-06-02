# Rust 常见术语中文对照表

## 核心概念

| 英文 | 中文 | 说明 |
|------|------|------|
| ownership | 所有权 | 每个值有且只有一个所有者，决定内存何时释放 |
| borrow / borrowing | 借用 | 在不转移所有权的情况下引用值 |
| lifetime | 生命周期 | 引用的有效作用域，用 `'a` 标注 |
| move | 移动 | 所有权从一个变量转移到另一个 |
| copy | 复制 | 实现 Copy trait 的类型赋值时复制而非移动 |
| clone | 克隆 | 显式深拷贝（调用 `.clone()`） |
| borrow checker | 借用检查器 | 编译器中强制执行所有权规则的组件 |
| drop | 销毁/析构 | 值离开作用域时自动调用，释放资源 |
| scope | 作用域 | 变量有效的代码块范围 |
| shadowing | 变量遮蔽 | 在同一作用域用 `let` 重新绑定同名变量 |

## 类型系统

| 英文 | 中文 | 说明 |
|------|------|------|
| trait | 特征/trait | 定义共享行为的接口（类似其他语言的接口/抽象类） |
| impl | 实现 | 为类型实现方法或 trait |
| generic | 泛型 | 参数化类型，如 `Vec<T>` |
| type alias | 类型别名 | 用 `type` 关键字给类型起别名 |
| newtype | 新类型 | 用单字段元组结构体包装现有类型 |
| associated type | 关联类型 | trait 中定义的占位类型 |
| type inference | 类型推断 | 编译器自动推断变量类型 |
| coercion | 类型强制转换 | 自动的隐式类型转换（如 `&String → &str`） |
| cast | 类型转换 | 用 `as` 关键字显式转换 |
| DST | 动态大小类型 | 编译时大小未知的类型，如 `str`、`[T]` |
| sized | 固定大小 | 编译时已知大小，可放在栈上 |
| unsized | 非固定大小 | 大小在编译时未知，只能通过引用使用 |
| ZST | 零大小类型 | 占用零字节内存的类型，如 `()` |

## 内存与指针

| 英文 | 中文 | 说明 |
|------|------|------|
| reference | 引用 | 借用值的指针，`&T` 或 `&mut T` |
| raw pointer | 裸指针 | 不受借用检查的指针，`*const T` 或 `*mut T` |
| smart pointer | 智能指针 | 携带额外能力的指针，如 Box、Rc、Arc |
| `Box<T>` | 堆分配指针 | 将值分配在堆上，拥有所有权 |
| `Rc<T>` | 引用计数指针 | 单线程下的共享所有权 |
| `Arc<T>` | 原子引用计数指针 | 多线程下的共享所有权 |
| `RefCell<T>` | 内部可变性容器 | 运行时检查借用规则（单线程） |
| `Cell<T>` | 可复制内部可变 | 通过复制提供内部可变性 |
| `Mutex<T>` | 互斥锁 | 多线程下的互斥访问 |
| `RwLock<T>` | 读写锁 | 允许多个读或一个写 |
| `Cow<T>` | 写时克隆 | 借用或拥有数据，按需克隆 |
| `Pin<T>` | 固定指针 | 防止值在内存中被移动（用于 async） |
| stack | 栈 | 自动管理的内存区域，存局部变量 |
| heap | 堆 | 动态分配的内存区域 |
| dangling pointer | 悬垂指针 | 指向已释放内存的指针（Rust 编译期禁止） |
| null safety | 空值安全 | Rust 用 `Option<T>` 代替空指针 |

## 数据结构

| 英文 | 中文 | 说明 |
|------|------|------|
| struct | 结构体 | 带命名字段的复合数据类型 |
| enum | 枚举 | 可携带数据的变体类型 |
| tuple | 元组 | 固定数量、可不同类型的有序集合，如 `(i32, &str)` |
| array | 数组 | 固定长度的同类型集合，存于栈 |
| slice | 切片 | 对数组或 Vec 的动态长度视图，`&[T]` |
| `Vec<T>` | 动态数组 | 可增长的堆分配数组 |
| `String` | 字符串 | 拥有所有权的 UTF-8 字符串 |
| `str` | 字符串切片 | 对 UTF-8 数据的引用，通常为 `&str` |
| `HashMap<K,V>` | 哈希映射 | 键值对集合 |
| `HashSet<T>` | 哈希集合 | 不重复元素集合 |
| `BTreeMap` / `BTreeSet` | 有序映射/集合 | 基于 B 树的有序键值/元素集合 |
| `VecDeque<T>` | 双端队列 | 两端均可高效增删的动态数组 |
| `LinkedList<T>` | 链表 | 双向链表（不常用） |
| `Option<T>` | 可选值 | 可能为空的值，`Some(T)` 或 `None` |
| `Result<T, E>` | 结果类型 | 可能出错的值，`Ok(T)` 或 `Err(E)` |

## 模式匹配与控制流

| 英文 | 中文 | 说明 |
|------|------|------|
| pattern matching | 模式匹配 | 用 `match` 表达式解构值 |
| match | 匹配 | 多分支值匹配表达式 |
| if let | if let | 只匹配一个模式的简洁写法 |
| while let | while let | 满足模式时持续循环 |
| destructuring | 解构 | 从复合类型中提取字段 |
| guard | 匹配守卫 | match 分支中附加的条件（`if` 子句） |
| wildcard | 通配符 | `_` 表示匹配任意值且不绑定 |
| arm | 分支 | match 表达式中的一个 `pattern => expr` |
| binding | 绑定 | 将值绑定到变量名 |

## 错误处理

| 英文 | 中文 | 说明 |
|------|------|------|
| panic | 恐慌/panic | 不可恢复错误，终止当前线程 |
| unwrap | 解包 | 取出 Option/Result 的值，失败则 panic |
| expect | 期望 | 带自定义消息的 unwrap |
| `?` operator | ? 运算符 | 提前返回 Err 的语法糖 |
| propagate | 错误传播 | 将错误向上层调用者传递 |
| recoverable error | 可恢复错误 | 用 Result 处理的预期错误 |
| unrecoverable error | 不可恢复错误 | 用 panic! 处理的严重错误 |
| `From` / `Into` | 类型转换 trait | 用于错误类型转换，`?` 会自动调用 |
| thiserror | 错误派生库 | 常用库，用宏简化自定义错误类型 |
| anyhow | 通用错误库 | 灵活的错误处理库，适合应用代码 |

## 函数与闭包

| 英文 | 中文 | 说明 |
|------|------|------|
| closure | 闭包 | 能捕获环境的匿名函数，`\|x\| x + 1` |
| `FnOnce` | 消费闭包 | 只能调用一次，消费捕获的变量 |
| `FnMut` | 可变闭包 | 可多次调用，可变地捕获变量 |
| `Fn` | 不可变闭包 | 可多次调用，不可变地借用变量 |
| higher-order function | 高阶函数 | 接受或返回函数的函数 |
| iterator | 迭代器 | 实现 Iterator trait，逐个产生值 |
| lazy evaluation | 惰性求值 | 迭代器适配器只在消费时执行 |
| method | 方法 | 绑定到类型上的函数（第一参数为 `self`） |
| associated function | 关联函数 | 不接受 self 的类型函数，如 `String::new()` |
| diverging function | 发散函数 | 永不返回的函数，返回类型为 `!` |

## 并发与异步

| 英文 | 中文 | 说明 |
|------|------|------|
| concurrency | 并发 | 多个任务交替推进 |
| parallelism | 并行 | 多个任务同时执行 |
| thread | 线程 | 操作系统调度的执行单元 |
| async / await | 异步/等待 | 非阻塞并发语法 |
| `Future` | Future | 异步操作的抽象，描述未来完成的计算 |
| executor | 执行器 | 驱动 Future 运行的运行时（如 Tokio） |
| task | 任务 | 轻量级异步执行单元 |
| channel | 通道 | 线程/任务间传递消息的机制 |
| `Send` | 可跨线程移动 | 标记类型可安全转移到其他线程 |
| `Sync` | 可跨线程共享 | 标记类型可安全地被多个线程引用 |
| deadlock | 死锁 | 两个线程互相等待对方释放锁 |
| data race | 数据竞争 | 多线程不同步地读写同一内存（Rust 编译期阻止） |
| atomic | 原子操作 | 不可中断的基本操作，用于无锁编程 |

## 模块系统与包管理

| 英文 | 中文 | 说明 |
|------|------|------|
| crate | 包/crate | Rust 的编译单元（库或二进制） |
| module | 模块 | 用 `mod` 关键字组织代码的命名空间 |
| workspace | 工作空间 | 管理多个 crate 的 Cargo 项目 |
| Cargo | Cargo | Rust 的官方构建工具和包管理器 |
| Cargo.toml | Cargo 配置文件 | 项目元数据和依赖声明 |
| Cargo.lock | 锁定文件 | 精确记录依赖版本，保证可复现构建 |
| crates.io | crates.io | Rust 的官方包注册仓库 |
| dependency | 依赖 | 项目引用的外部 crate |
| feature flag | 特性开关 | 可选地启用 crate 的部分功能 |
| `pub` / `pub(crate)` | 公开/包内公开 | 控制符号的可见性 |
| use | 引入 | 将路径引入当前作用域 |
| path | 路径 | 访问模块中符号的定位方式，如 `std::io::Read` |
| prelude | 预导入 | 自动引入的常用符号集合 |

## 宏

| 英文 | 中文 | 说明 |
|------|------|------|
| macro | 宏 | 编译期代码生成机制 |
| declarative macro | 声明式宏 | 用 `macro_rules!` 定义的基于规则的宏 |
| procedural macro | 过程宏 | 操作 token 流的宏（derive/attr/function-like） |
| derive | 派生 | 自动实现 trait，如 `#[derive(Debug, Clone)]` |
| attribute macro | 属性宏 | 以 `#[...]` 形式标注的宏 |
| hygiene | 宏卫生 | 宏中的变量不会与外部命名冲突 |
| `macro_rules!` | 声明式宏定义 | 定义模式匹配式宏的关键字 |
| bang macro | 感叹号宏 | 以 `name!(...)` 调用的宏，如 `println!` |

## Trait 与多态

| 英文 | 中文 | 说明 |
|------|------|------|
| trait object | trait 对象 | 动态分发的 trait 引用，如 `&dyn Trait` |
| dynamic dispatch | 动态分发 | 运行时通过 vtable 确定调用哪个实现 |
| static dispatch | 静态分发 | 编译期单态化，零运行时开销 |
| monomorphization | 单态化 | 为每个泛型具体类型生成独立代码 |
| vtable | 虚函数表 | 实现动态分发的函数指针表 |
| `dyn Trait` | 动态 trait | 用于 trait 对象的关键字 |
| `impl Trait` | 不透明类型 | 参数/返回位置的不透明类型，静态分发 |
| blanket impl | 全面实现 | 对所有满足条件的类型统一实现 trait |
| orphan rule | 孤儿规则 | trait 或类型至少有一个须在当前 crate 中定义 |
| coherence | 一致性 | 保证 trait 实现不产生冲突的规则 |
| supertraits | 超特征 | 要求实现某 trait 前先实现另一 trait |
| default method | 默认方法 | trait 中提供默认实现的方法 |

## 编译器与工具链

| 英文 | 中文 | 说明 |
|------|------|------|
| rustc | Rust 编译器 | Rust 的官方编译器 |
| rustup | 工具链管理器 | 管理 Rust 版本和目标平台的工具 |
| clippy | 代码检查工具 | 提供额外 lint 和改进建议的工具 |
| rustfmt | 代码格式化工具 | 自动格式化 Rust 代码 |
| rust-analyzer | 语言服务器 | 编辑器的代码补全/跳转等语言支持 |
| MIR | 中间表示 | 编译器内部的中级中间语言 |
| HIR | 高级中间表示 | 编译器内部的高级中间语言 |
| LLVM | LLVM 后端 | Rust 编译器使用的底层代码生成框架 |
| target triple | 目标三元组 | 描述编译目标平台的字符串，如 `x86_64-unknown-linux-gnu` |
| cross compilation | 交叉编译 | 在一种平台上编译出另一种平台的程序 |
| nightly | 每日构建 | 含实验性特性的 Rust 版本 |
| stable / beta | 稳定/测试版 | 正式发布的 Rust 版本 |
| edition | 版次 | Rust 语言版本，如 2015/2018/2021 |
| lint | 静态检查 | 编译器或 Clippy 对代码的警告和建议 |
| unsafe | 不安全代码 | 允许绕过部分安全检查的代码块 |
| FFI | 外部函数接口 | 与 C 等语言互操作的机制 |
| ABI | 应用二进制接口 | 函数调用约定，如 `extern "C"` |

## 常用标准 Trait

| 英文 | 中文 | 说明 |
|------|------|------|
| `Debug` | 调试格式 | 用 `{:?}` 打印，可 `#[derive(Debug)]` |
| `Display` | 展示格式 | 用 `{}` 打印，需手动实现 |
| `Clone` | 克隆 | 显式深拷贝 |
| `Copy` | 复制 | 按位复制，无需显式 `.clone()` |
| `PartialEq` / `Eq` | 等值比较 | 实现 `==` 运算符 |
| `PartialOrd` / `Ord` | 排序比较 | 实现 `<` `>` 等运算符 |
| `Hash` | 哈希 | 用于 HashMap/HashSet 的键 |
| `Default` | 默认值 | 提供类型的默认值 `T::default()` |
| `From` / `Into` | 类型转换 | 不会失败的类型转换 |
| `TryFrom` / `TryInto` | 可失败转换 | 可能失败的类型转换，返回 Result |
| `AsRef` / `AsMut` | 引用转换 | 廉价的引用到引用转换 |
| `Deref` / `DerefMut` | 解引用 | 自定义 `*` 运算符，支持自动解引用 |
| `Iterator` | 迭代器 | 提供 `.next()` 方法的惰性序列 |
| `IntoIterator` | 转迭代器 | 可转化为迭代器的类型 |
| `FromIterator` | 从迭代器构建 | 用 `.collect()` 构建集合 |
| `Index` / `IndexMut` | 索引 | 支持 `[]` 运算符 |
| `Add` / `Sub` / `Mul`… | 运算符重载 | 实现算术运算符 |
| `Drop` | 析构 | 离开作用域时自动调用的清理逻辑 |
| `Send` | 可跨线程移动 | 标记类型可安全转移到其他线程 |
| `Sync` | 可跨线程共享 | 标记类型可安全地被多个线程引用 |
| `Sized` | 固定大小 | 编译时大小已知的类型（默认隐含） |
| `Unpin` | 可移动 | 不需要被 Pin 固定的类型 |
| `Future` | 异步任务 | 代表一个将来完成的异步计算 |
| `Read` / `Write` | 读写 | 标准 I/O 读写接口 |
| `BufRead` | 缓冲读取 | 提供按行读取等缓冲 I/O 能力 |