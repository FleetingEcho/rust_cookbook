# HashMap 哈希表

## 1. HashMap 基础操作

```rust
use std::collections::HashMap;

fn hashmap_demo() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    println!("插入数据: {:?}", scores);

    if let Some(score) = scores.get("Alice") {
        println!("Alice 的分数: {}", score);
    } else {
        println!("Alice 的分数不存在");
    }

    scores.remove("Bob");
    println!("删除 Bob 后: {:?}", scores);

    println!("是否包含 Alice？{}", scores.contains_key("Alice"));
    println!("是否包含 Bob？{}", scores.contains_key("Bob"));

    scores.entry("Charlie").or_insert(88);
    scores.entry("Alice").or_insert(100); // Alice 已存在，不修改
    println!("使用 entry() 插入: {:?}", scores);

    for (key, value) in &scores {
        println!("{} 的分数是 {}", key, value);
    }

    let text = "hello rust hello world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    println!("单词计数: {:?}", word_count);
}

fn main() {
    hashmap_demo();
}
```

## 2. HashMap 键的要求

默认情况下，`HashMap<K, V>` 依赖于 `std::collections::hash_map::RandomState` 作为哈希算法，它要求：

- 键 `K` 必须实现 `Eq`（用于相等比较）。
- 键 `K` 必须实现 `Hash`（用于哈希计算）。
- 键 `K` 不能频繁变化（否则会导致哈希值失效）。

### 常见的可作为键的类型

| 类型 | 是否可用？ | 说明 |
|------|-----------|------|
| `i32`, `u32`, `i64`, `u64`, `usize` | ✅ | 整数类型，默认实现 `Eq` + `Hash` |
| `String` | ✅ | `String` 适合作为键 |
| `&str` | ✅ | `&str` 适合作为键（自动转换为 `String`） |
| `bool` | ✅ | `true`/`false` 作为键是可以的 |
| `char` | ✅ | 适合作为键 |
| `Vec<T>` | ❌ | `Vec<T>` 没有实现 `Hash`，不能作为键 |
| `HashMap<K, V>` | ❌ | `HashMap` 不能作为键 |
| 自定义结构体 | ⚠️ 需要实现 `Eq` + `Hash` | 需手动派生或实现 |

## 3. HashMap 值的要求

值 `V` 没有 `Hash` 约束，可以是任何类型。值 `V` 可以是 `Vec<T>`、`HashMap<K, V>`、`Option<T>`、自定义类型等。

### 常见的可作为值的类型

| 类型 | 是否可用？ | 说明 |
|------|-----------|------|
| `i32`, `u32`, `bool`, `f64`, `char` | ✅ | 任何基本类型都可以作为 `V` |
| `String` | ✅ | `String` 可以作为 `V` |
| `Vec<T>` | ✅ | `Vec<T>` 可以作为值 |
| `HashMap<K, V>` | ✅ | 允许嵌套 `HashMap` |
| `Option<T>` | ✅ | `Option<T>` 允许存储可选值 |
| 自定义结构体 | ✅ | `V` 没有限制，任何类型都可以作为值 |

---

## 📘 TypeScript 对比

Rust `HashMap<K, V>` ≈ TS `Map<K, V>` 或 `{ [key]: value }`。

| 操作 | Rust | TypeScript |
|------|------|-----------|
| 创建 | `HashMap::new()` | `new Map()` 或 `{}` |
| 插入 | `m.insert(k, v)` | `m.set(k, v)` 或 `obj[k]=v` |
| 读取 | `m.get(&k)` 返回 `Option<&V>` | `m.get(k)` 或 `obj[k]` |
| 删除 | `m.remove(&k)` | `m.delete(k)` 或 `delete obj[k]` |
| 遍历 | `for (k, v) in &m` | `for...of m.entries()` |
| 所有权 | 值被移动进 HashMap | 引用存储 |

> ⚠️ Rust `HashMap` 的 `get()` 返回 `Option<&V>`，不会 panic。而 TS 对象读取不存在的键返回 `undefined`，Rust 强制你处理"键不存在"的情况。

详细对照 → [rust_vs_typescript.rs §13 "集合"](../rust_vs_typescript.rs)
