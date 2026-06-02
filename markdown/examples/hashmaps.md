# Rust vs TypeScript：HashMap 与集合

> **运行命令**：`cargo run -p learning_notes --example rts_hashmaps`

---

## TypeScript 参考版本

```ts
// Map<K, V>
const map = new Map<string, number>();
map.set("Alice", 90);
map.get("Alice");           // 90 | undefined
map.has("Alice");           // true
map.delete("Alice");
map.size;
for (const [k, v] of map) { ... }
[...map.keys()]
[...map.values()]
[...map.entries()]

// Record<string, number>（对象字面量）
const scores: Record<string, number> = { Alice: 90 };
scores["Alice"];
Object.keys(scores);
Object.values(scores);
Object.entries(scores);

// Set<T>
const set = new Set<number>([1, 2, 3]);
set.add(4);
set.has(3);
set.delete(2);
set.size;
[...new Set(arr)]           // 数组去重
```

---

## 一、HashMap<K, V>

**TS 对应**：`Map<K, V>` 或 `Record<K, V>`

**key** 需要实现 `Eq + Hash` trait。

```rust
use std::collections::{HashMap, HashSet, BTreeMap};

let mut scores: HashMap<String, i32> = HashMap::new();

// 插入 — TS: map.set("Alice", 90)
scores.insert(String::from("Alice"),   90);
scores.insert(String::from("Bob"),     85);
scores.insert(String::from("Charlie"), 92);

// 覆盖（重复插入同一 key 会覆盖旧值）
scores.insert(String::from("Bob"), 95);
println!("覆盖 Bob: {:?}", scores.get("Bob"));

// 读取：返回 Option<&V>，TS 的 map.get() 可能返回 undefined
if let Some(score) = scores.get("Alice") {
    println!("Alice: {score}");
}

// 读取并提供默认值 — TS: map.get("Dave") ?? 0
let dave_score = scores.get("Dave").copied().unwrap_or(0);

// 判断 key 是否存在 — TS: map.has("Bob")
println!("有 Bob 吗: {}", scores.contains_key("Bob"));

// 删除 — TS: map.delete("Bob")
scores.remove("Bob");

// 大小 — TS: map.size
println!("大小: {}", scores.len());

// 遍历 — TS: for (const [k, v] of map)
for (name, score) in &scores {
    println!("  {name}: {score}");
}

// 只遍历键 — TS: [...map.keys()]
let mut names: Vec<&String> = scores.keys().collect();

// 只遍历值 — TS: [...map.values()]
let vals: Vec<&i32> = scores.values().collect();
```

### Entry API（核心用法，TS 没有直接对应）

```rust
// 不存在则插入，存在则不变
scores.entry(String::from("Dave")).or_insert(0);
scores.entry(String::from("Alice")).or_insert(100); // Alice 已存在，不修改

// 基于 entry 更新已有值（计数器经典写法）
// TS: map.set("Alice", (map.get("Alice") ?? 0) + 10)
*scores.entry(String::from("Alice")).or_insert(0) += 10;

// 词频统计（entry 最典型的应用）
let text = "hello world hello rust rust rust world";
let mut word_count: HashMap<&str, i32> = HashMap::new();
for word in text.split_whitespace() {
    *word_count.entry(word).or_insert(0) += 1;
}
```

### 从 Vec 构建 HashMap

```rust
// TS: new Map(entries) 或 Object.fromEntries(entries)
let entries = vec![
    (String::from("a"), 1_i32),
    (String::from("b"), 2),
    (String::from("c"), 3),
];
let map_from_vec: HashMap<String, i32> = entries.into_iter().collect();
```

### 嵌套 HashMap

```rust
// TS: Map<string, Map<string, number>>
let mut nested: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
nested.entry("group1").or_default().insert("a", 1);
nested.entry("group1").or_default().insert("b", 2);
nested.entry("group2").or_default().insert("c", 3);
```

---

## 二、BTreeMap<K, V>

**TS** 没有直接对应；`BTreeMap` 保证 key 按排序顺序迭代。

`HashMap` 是无序的，`BTreeMap` 是有序的。

```rust
let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
btree.insert("banana", 2);
btree.insert("apple", 1);
btree.insert("cherry", 3);

// 迭代时按 key 字母序输出
for (k, v) in &btree {
    println!("  {k}: {v}");
}
```

---

## 三、HashSet<T>

**TS 对应**：`Set<T>`

```rust
let mut set: HashSet<i32> = HashSet::new();
set.insert(1);   // TS: set.add(1)
set.insert(2);
set.insert(3);
set.insert(2);   // 重复插入无效，set 保证元素唯一

println!("包含 3: {}", set.contains(&3));  // TS: set.has(3)
set.remove(&2);                             // TS: set.delete(2)
println!("大小: {}", set.len());            // TS: set.size
```

### 集合运算（TS 需要手动实现，Rust 内置）

```rust
let a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
let b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

// 并集 — TS: new Set([...a, ...b])
let mut union: Vec<i32> = a.union(&b).cloned().collect();

// 交集 — TS: new Set([...a].filter(x => b.has(x)))
let mut intersect: Vec<i32> = a.intersection(&b).cloned().collect();

// 差集 — TS: new Set([...a].filter(x => !b.has(x)))
let mut diff: Vec<i32> = a.difference(&b).cloned().collect();

// 对称差集
let mut sym_diff: Vec<i32> = a.symmetric_difference(&b).cloned().collect();

// 子集判断 — TS 需手动实现
let small: HashSet<i32> = [1, 2].iter().cloned().collect();
println!("是子集: {}", small.is_subset(&a));
```

### 数组去重

**TS**: `[...new Set(arr)]`

```rust
let with_dups = vec![1, 2, 2, 3, 3, 3, 4];
let mut deduped: Vec<i32> = with_dups.into_iter()
    .collect::<HashSet<_>>().into_iter().collect();
```
