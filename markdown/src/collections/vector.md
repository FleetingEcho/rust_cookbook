# Vector 动态数组

## 1. 创建 Vector

有多种方式可以创建 `Vec`：

```rust
let v: Vec<i32> = Vec::new(); // 必须指明类型

let mut v = Vec::new(); // 可以不指定类型
v.push(1);

let v = vec![1, 2, 3]; // 使用宏来创建数组
```

## 2. 更新 Vector

```rust
let mut v = Vec::new();
v.push(1);
```

Vector 与其元素共存亡。跟结构体一样，Vector 类型在超出作用域范围后，会被自动删除。

## 3. 读取元素

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("第三个元素是 {}", third);

match v.get(2) {
    Some(third) => println!("第三个元素是 {third}"),
    None => println!("去你的第三个元素，根本没有！"),
}
```

## 4. 借用检查

当持有 Vector 的引用时，不能同时对其进行可变操作：

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // ✅ 获取 v 的不可变引用

v.push(6); // ❌ 这里对 v 进行了可变借用

println!("The first element is: {first}"); // ❌ 这里仍然使用了 first
```

**修复方式——克隆值而非持有引用：**

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = v[0].clone(); // ✅ 获取值，而不是引用

v.push(6);

println!("The first element is: {first}");
```

## 5. 存储不同类型的元素

使用 `enum` 使 Vector 可以存储"不同类型"的值：

```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()), // IPv4 变体
        IpAddr::V6("::1".to_string()),       // IPv6 变体
    ];

    for ip in v {
        show_addr(ip);
    }
}

fn show_addr(ip: IpAddr) {
    match ip {
        IpAddr::V4(addr) => println!("IPv4: {}", addr),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}
```

Vector 要求其所有元素具有相同的类型。使用 `enum` 定义一个涵盖所有可能类型的公共类型，是实现混合存储的惯用方式。

---

## 📘 TypeScript 对比

Rust `Vec<T>` ≈ TS `T[]` 数组。

**Rust：**

```rust
let mut v = vec![1, 2, 3];
v.push(4);
```

**TypeScript：**

```ts
let v: number[] = [1, 2, 3];
v.push(4);
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 类型 | `Vec<T>` 强类型 | `any[]` 或泛型 `T[]` |
| 增长 | 堆上动态扩容 | 数组自动扩容 |
| 借用检查 | 持有引用时不能 push | 无此限制 |
| 内存 | 连续堆分配 | V8 引擎管理 |

> ⚠️ Rust 的借用检查器要求在持有 Vector 元素引用时不能修改 Vector，而 TypeScript 没有此限制。

详细对照 → [rust_vs_typescript.rs §13 "集合"](../rust_vs_typescript.rs)
