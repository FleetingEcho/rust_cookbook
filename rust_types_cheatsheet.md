# Rust 数据类型方法速查手册

> 覆盖范围：`String` · `&str` · `Tuple` · `[T; N]` · `&[T]` · `Vec<T>` · `HashMap<K,V>` · `HashSet<T>` · `Option<T>` · `Result<T,E>` · `Iterator`
> 末尾附：**各类型之间的转换速查表**

---

## 一、String（堆分配·可增长·UTF-8）

### 创建
```rust
String::new()                    // 空字符串
String::from("hello")            // 从字符串字面量构建
"hello".to_string()              // &str → String（同上，更惯用）
String::with_capacity(10)        // 预分配容量，避免频繁 realloc
format!("{}_{}", a, b)          // 格式化拼接，返回新 String
```

### 追加 / 修改
```rust
s.push_str(" world")             // 追加 &str（不消耗参数）
s.push('!')                      // 追加单个 char
s1 + &s2                         // 拼接（消耗 s1，s2 只需借用）
s.insert(0, 'H')                 // 在字节位置 0 插入 char（注意 UTF-8 边界）
s.insert_str(0, "Hi")            // 在字节位置插入 &str
s.replace("foo", "bar")          // 替换所有匹配，返回新 String
s.replacen("foo", "bar", 2)      // 最多替换 n 次
s.truncate(5)                    // 截断到前 5 字节（原地，注意 UTF-8 边界）
s.clear()                        // 清空，保留已分配容量
s.retain(|c| c.is_alphanumeric()) // 就地保留满足条件的字符
```

### 查询
```rust
s.len()                          // 字节长度（非字符数！）
s.is_empty()                     // 是否为空
s.contains("lo")                 // 包含子串？
s.starts_with("he")              // 前缀匹配
s.ends_with("ld")                // 后缀匹配
s.find("lo")                     // 返回 Option<usize>（字节位置）
s.rfind("l")                     // 从右查找，返回 Option<usize>
s.chars().count()                // 字符数（O(n)，遍历计数）
```

### 迭代 / 切片
```rust
s.chars()                        // 按 Unicode 字符迭代（char）
s.bytes()                        // 按 u8 迭代
s.lines()                        // 按行迭代（自动处理 \n 和 \r\n）
s.split(',')                     // 按分隔符切分，返回迭代器
s.split_whitespace()             // 按空白切分，自动跳过多余空白
s.trim()                         // 去首尾空白（返回 &str）
s.trim_start() / s.trim_end()   // 只去一侧
s.to_uppercase()                 // → 新 String（大写）
s.to_lowercase()                 // → 新 String（小写）
```

### 类型转换（见末尾汇总表）
```rust
&s                               // → &str（最常用）
s.as_str()                       // → &str（语义更明确）
s.as_bytes()                     // → &[u8]（借用，不消耗）
s.into_bytes()                   // → Vec<u8>（消耗，零拷贝）
s.chars().collect::<Vec<char>>() // → Vec<char>
```

---

## 二、&str（借用切片·UTF-8·生命周期跟随来源）

> `&str` 是只读的，绝大多数"查询"方法与 `String` 相同（因为 `String` 实现了 `Deref<Target=str>`）。

### 查询（与 String 共享）
```rust
s.len() / s.is_empty()
s.contains("x") / s.starts_with("x") / s.ends_with("x")
s.find("x") / s.rfind("x")
```

### 迭代 / 切片
```rust
s.chars()                        // 字符迭代器
s.bytes()                        // u8 迭代器
s.lines()                        // 行迭代器
s.split('/') / s.split_whitespace()
s.trim() / s.trim_start() / s.trim_end()
&s[2..5]                         // 字节切片（必须落在 UTF-8 字符边界！）
```

### 转换
```rust
s.to_string()                    // → String（堆分配，拷贝数据）
s.to_owned()                     // → String（语义：我要拥有这份数据）
s.parse::<i32>()                 // → Result<i32, _>（字符串转数字最常用）
s.parse::<f64>()
s.as_bytes()                     // → &[u8]
```

---

## 三、Tuple（元组·固定长度·异构·值类型）

> 元组没有通用方法，所有操作都通过语法完成。

### 访问与解构
```rust
let t = (42, "hi", 3.14);

t.0                              // 取第一个元素：42
t.1                              // 取第二个元素："hi"

let (n, s, f) = t;              // 解构绑定
let (n, ..) = t;                // 只取第一个，忽略其余

// 函数返回多值的惯用法
fn min_max(v: &[i32]) -> (i32, i32) { (*v.iter().min().unwrap(), *v.iter().max().unwrap()) }
let (lo, hi) = min_max(&[3,1,4,1,5]);
```

### 特殊形式
```rust
()                               // 单元类型（unit），函数无返回值时隐式返回
(42,)                            // 单元素元组（注意末尾逗号，否则只是括号）

struct Point(f64, f64);         // 元组结构体：有名字的元组
let p = Point(1.0, 2.0);
println!("{}", p.0);
```

### 自动实现的 trait（≤12 元素）
```rust
// 所有元素实现 PartialEq 时，元组也实现 PartialEq
// 所有元素实现 Debug 时，元组也实现 Debug
// Clone、Copy、Hash 同理
println!("{:?}", (1, "a", true)); // (1, "a", true)
```

---

## 四、[T; N]（数组·固定长度·同构·栈分配）

> `N` 是类型的一部分，`[i32; 3]` 和 `[i32; 4]` 是两个不同类型。

### 创建
```rust
let a = [1, 2, 3];              // 字面量，N 由编译器推断
let a = [0i32; 5];              // 重复初始化：[0, 0, 0, 0, 0]
```

### 访问
```rust
a[i]                             // 索引访问（越界 panic）
a.get(i)                         // → Option<&T>（安全访问）
a.first()                        // → Option<&T>
a.last()                         // → Option<&T>
a.len()                          // 编译期常量（也可 a.len() 运行时调用）
```

### 迭代
```rust
a.iter()                         // → &T 迭代器（借用）
a.iter_mut()                     // → &mut T（可变借用）
a.into_iter()                    // → T（消耗，Rust 2021+）
a.map(|x| x * 2)                // → 新等长数组（不是迭代器！）
```

### 查询 / 操作
```rust
a.contains(&3)                   // 线性查找
a.windows(2)                     // 滑动窗口，每次返回长度为 2 的 &[T]
a.chunks(2)                      // 分块，每块最多 2 个元素
a.sort()                         // 原地排序（元素需实现 Ord）
a.sort_by(|a, b| a.cmp(b))     // 自定义排序
```

### 转换
```rust
&a                               // → &[T]（切片，最常用）
a.as_slice()                     // → &[T]
Vec::from(a)                     // → Vec<T>
```

---

## 五、&[T]（切片·任意长度·借用视图）

> 切片是数组、Vec 等连续内存的统一借用视图，方法最丰富。

### 访问
```rust
s[i]                             // 索引（越界 panic）
s.get(i)                         // → Option<&T>
s.first() / s.last()            // → Option<&T>
s.len() / s.is_empty()
```

### 查找
```rust
s.contains(&v)                   // 线性查找
s.iter().position(|x| *x == v)  // → Option<usize>
s.binary_search(&v)              // → Result<usize, usize>（要求已排序）
```

### 分割 / 窗口
```rust
s.split_at(3)                    // → (&[T], &[T])
s.windows(n)                     // 长度为 n 的滑动窗口
s.chunks(n)                      // 每块最多 n 个，最后一块可能更短
s.chunks_exact(n)                // 每块恰好 n 个，不足的在 .remainder()
s.split(|x| *x == 0)            // 按条件切分
```

### 排序（需 &mut [T]）
```rust
s.sort()
s.sort_by(|a, b| a.partial_cmp(b).unwrap())
s.sort_by_key(|x| x.abs())
s.reverse()                      // 原地反转
```

---

## 六、Vec\<T\>（动态数组·堆分配·连续内存）

### 创建
```rust
Vec::new()                       // 空 Vec，容量为 0
vec![1, 2, 3]                   // 宏，最常用
Vec::with_capacity(16)           // 预分配 16 个槽位
iter.collect::<Vec<_>>()         // 从迭代器收集
vec![0; 10]                      // 10 个 0
```

### 增 / 删 / 改
```rust
v.push(x)                        // 末尾追加
v.pop()                          // 移除末尾，→ Option<T>
v.insert(2, x)                   // 在位置 2 插入（O(n)，后续元素右移）
v.remove(2)                      // 移除位置 2 并返回（O(n)）
v.swap_remove(2)                 // 用末尾元素填坑后移除（O(1)，不保顺序）
v.retain(|x| *x > 0)           // 就地保留满足条件的元素
v.dedup()                        // 移除连续重复（需先排序才能去全部重复）
v.sort() / v.sort_by(...)       // 原地排序
v.sort_by_key(|x| x.len())     // 按 key 排序
v.reverse()                      // 原地反转
v.truncate(5)                    // 截断到前 5 个元素
v.clear()                        // 清空，保留容量
v.resize(10, 0)                  // 调整长度，不足则填 0
v.extend(iter)                   // 批量追加
v.append(&mut v2)               // 将 v2 全部移入 v（v2 变空）
v.drain(1..3)                    // 移除并迭代范围内元素
```

### 访问 / 查询
```rust
v[i]                             // 索引（越界 panic）
v.get(i)                         // → Option<&T>
v.first() / v.last()            // → Option<&T>
v.len() / v.is_empty()
v.capacity()                     // 当前已分配的槽位数
v.contains(&x)                   // 线性查找
v.iter().position(|x| ...)      // → Option<usize>
v.binary_search(&x)              // 有序时 O(log n)
v.windows(n) / v.chunks(n)      // 滑动窗口 / 分块
v.split_at(n)                    // → (&[T], &[T])
```

### 迭代（三种语义）
```rust
v.iter()                         // → &T（借用，v 仍可用）
v.iter_mut()                     // → &mut T（可变借用）
v.into_iter()                    // → T（消耗 v，转移所有权）

// 惯用遍历写法
for x in &v { ... }             // 等价于 v.iter()
for x in &mut v { ... }        // 等价于 v.iter_mut()
for x in v { ... }              // 等价于 v.into_iter()
```

### 转换
```rust
&v / v.as_slice()               // → &[T]
v.as_mut_slice()                 // → &mut [T]
v.into_boxed_slice()             // → Box<[T]>（缩容到恰好合适）
```

---

## 七、HashMap\<K, V\>（哈希表·无序·均摊 O(1)）

### 创建
```rust
use std::collections::HashMap;

HashMap::new()
HashMap::with_capacity(16)
// 从迭代器收集
let m: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
```

### 增 / 删
```rust
m.insert("key", 42)             // 插入，→ Option<旧值>（存在则替换）
m.remove("key")                  // 删除，→ Option<V>
m.remove_entry("key")            // 删除，→ Option<(K, V)>
m.clear()                        // 清空所有项
m.retain(|k, v| *v > 0)        // 就地过滤
m.extend(iter)                   // 批量插入 (K, V) 对
```

### 访问 / 查询
```rust
m["key"]                         // 直接索引，键不存在则 panic
m.get("key")                     // → Option<&V>
m.get_mut("key")                 // → Option<&mut V>
m.get_key_value("key")           // → Option<(&K, &V)>
m.contains_key("key")
m.len() / m.is_empty()
```

### Entry API（核心惯用法）
```rust
// 不存在则插入，存在则直接用——只查一次哈希，最高效
m.entry("key").or_insert(0);

// 计数器经典写法
*m.entry(word).or_insert(0) += 1;

// 不存在则惰性构造
m.entry("key").or_insert_with(|| expensive_fn());

// 不存在则插入默认值
m.entry("key").or_default();

// 存在则修改，不存在则不管
m.entry("key").and_modify(|v| *v += 1);

// 组合：不存在插 0，存在加 1
m.entry("key").and_modify(|v| *v += 1).or_insert(1);
```

### 迭代
```rust
m.keys()                         // 键迭代器
m.values()                       // 值迭代器
m.values_mut()                   // 可变值迭代器
m.iter()                         // → (&K, &V)
m.iter_mut()                     // → (&K, &mut V)
m.into_iter()                    // → (K, V)（消耗）
```

> **BTreeMap\<K, V\>**：接口几乎相同，但按 Key 有序排列，并额外支持：
> ```rust
> use std::collections::BTreeMap;
> m.range("a".."z")               // 范围查询
> m.first_key_value()             // → Option<(&K, &V)>
> m.last_key_value()              // → Option<(&K, &V)>
> ```

---

## 八、HashSet\<T\>（集合·无序·均摊 O(1)）

```rust
use std::collections::HashSet;

let mut s: HashSet<i32> = HashSet::new();
```

### 增 / 删 / 查
```rust
s.insert(42)                     // → bool（是否是新插入的）
s.remove(&42)                    // → bool（是否存在并删除）
s.contains(&42)                  // → bool
s.len() / s.is_empty()
s.get(&42)                       // → Option<&T>
```

### 集合运算（返回迭代器，惰性）
```rust
s1.union(&s2)                    // 并集 s1 ∪ s2
s1.intersection(&s2)             // 交集 s1 ∩ s2
s1.difference(&s2)               // 差集 s1 − s2（在 s1 不在 s2）
s1.symmetric_difference(&s2)     // 对称差集（只在其中一个的元素）

// 若要得到新 HashSet：
let union: HashSet<_> = s1.union(&s2).cloned().collect();
```

### 关系判断
```rust
s1.is_subset(&s2)                // s1 ⊆ s2
s1.is_superset(&s2)              // s1 ⊇ s2
s1.is_disjoint(&s2)              // 无交集
```

---

## 九、Option\<T\>（可空值·编译期强制处理）

### 解包（取出值）
```rust
opt.unwrap()                     // Some(v) → v；None → panic
opt.expect("必须有值")            // 同上，panic 时带自定义信息
opt.unwrap_or(0)                 // None 时返回默认值
opt.unwrap_or_else(|| compute()) // None 时惰性计算默认值
opt.unwrap_or_default()          // None 时返回 Default::default()
```

### 检查
```rust
opt.is_some()
opt.is_none()

if let Some(v) = opt { ... }    // 模式匹配（最惯用）
match opt {
    Some(v) => ...,
    None => ...,
}
```

### 变换（不解包，保持 Option）
```rust
opt.map(|v| v + 1)              // Some(v) → Some(f(v))；None → None
opt.map_or(0, |v| v + 1)        // Some → f(v)；None → 默认值
opt.map_or_else(default, f)      // 惰性版本

opt.and_then(|v| some_fn(v))    // flatMap：f 返回 Option，避免 Option<Option<T>>
opt.filter(|v| *v > 0)          // 不满足条件 → None
opt.or(Some(42))                 // None 时用另一个 Option 替换
opt.or_else(|| Some(compute()))  // None 时惰性替换
opt.zip(other)                   // (Some(a), Some(b)) → Some((a, b))；否则 None
```

### 转换
```rust
?                                // 在返回 Option 的函数中，None 提前返回（最惯用）
opt.ok_or("error msg")           // → Result<T, &str>
opt.ok_or_else(|| make_err())    // → Result<T, E>（惰性）
opt.as_ref()                     // Option<T> → Option<&T>（不消耗）
opt.as_mut()                     // Option<T> → Option<&mut T>
opt.take()                       // 取出值，原地置为 None（需 &mut）
opt.replace(new_val)             // 替换值，返回旧值
opt.flatten()                    // Option<Option<T>> → Option<T>
opt.transpose()                  // Option<Result<T,E>> ↔ Result<Option<T>,E>
```

---

## 十、Result\<T, E\>（可错值·编译期强制处理）

### 解包
```rust
res.unwrap()                     // Ok(v) → v；Err → panic
res.expect("操作失败")            // 同上，带信息
res.unwrap_or(default)
res.unwrap_or_else(|e| handle(e))
res.unwrap_or_default()
```

### 检查
```rust
res.is_ok() / res.is_err()
if let Ok(v) = res { ... }
match res { Ok(v) => ..., Err(e) => ... }
```

### 变换
```rust
res.map(|v| v + 1)              // Ok → Ok(f(v))；Err 透传
res.map_err(|e| MyErr(e))       // Err → Err(g(e))；Ok 透传
res.map_or(0, |v| v + 1)

res.and_then(|v| another_op(v)) // 链式操作，f 返回 Result
res.or_else(|e| recover(e))     // Err 时尝试恢复
res.or(Ok(42))                   // Err 时替换
```

### 转换
```rust
?                                // 最重要：Err(e) → return Err(e.into())，Ok(v) → v
                                 // 等价于：match res { Ok(v) => v, Err(e) => return Err(e.into()) }

res.ok()                         // → Option<T>（丢弃 Err）
res.err()                        // → Option<E>（丢弃 Ok）
res.as_ref()                     // → Result<&T, &E>
res.transpose()                  // Result<Option<T>, E> ↔ Option<Result<T, E>>
```

---

## 十一、Iterator（惰性·零成本·可组合）

> 核心心智模型：**适配器是惰性的，消费者才触发计算**。
> 链式管线 `.filter(..).map(..).collect()` 只遍历集合一次。

### 获取迭代器
```rust
v.iter()                         // &T
v.iter_mut()                     // &mut T
v.into_iter()                    // T
(0..10)                          // Range，本身就是迭代器
"a,b,c".split(',')              // str::Split，也是迭代器
```

### 转换适配器（惰性，不求值）
```rust
.map(|x| x * 2)                 // 逐元素变换
.filter(|x| *x > 0)            // 过滤
.filter_map(|x| f(x))          // filter + map 合一（f 返回 Option）
.flat_map(|x| vec![x, x])      // 映射后展平一层
.flatten()                       // 展平一层嵌套迭代器
.take(n)                         // 只取前 n 个
.skip(n)                         // 跳过前 n 个
.take_while(|x| *x < 10)       // 满足条件时持续取
.skip_while(|x| *x < 10)       // 满足条件时持续跳过
.zip(other)                      // 拉链合并，→ (a, b)
.chain(other)                    // 串联两个迭代器
.enumerate()                     // → (index, &T)
.rev()                           // 反转（需实现 DoubleEndedIterator）
.peekable()                      // 包装为可偷看下一个元素的迭代器
.step_by(n)                      // 每 n 步取一个
.cloned()                        // &T → T（需 T: Clone）
.copied()                        // &T → T（需 T: Copy，比 cloned 快）
```

### 消费者（立即求值，驱动迭代）
```rust
.collect::<Vec<_>>()            // 收集为集合（Vec、HashMap、HashSet、String 等）
.collect::<String>()            // 把字符迭代器收集为 String
.count()                         // 元素总数
.sum::<i32>()                   // 求和
.product::<i32>()               // 求积
.min() / .max()                 // → Option<T>
.min_by_key(|x| x.len())       // 按 key 求最小
.max_by(|a, b| a.cmp(b))       // 自定义比较求最大
.fold(0, |acc, x| acc + x)     // 通用归约（有初始值）
.reduce(|acc, x| acc + x)       // 无初始值的 fold，→ Option<T>
.for_each(|x| println!("{x}")) // 遍历执行副作用（无返回值）
.any(|x| x > 0)                 // 存在一个满足？（短路）
.all(|x| x > 0)                 // 全部满足？（短路）
.find(|x| *x > 5)              // → Option<&T>（找到第一个）
.find_map(|x| f(x))            // 找到第一个 Some(v)，→ Option<T>
.position(|x| *x > 5)          // → Option<usize>
.last()                          // → Option<T>（消耗整个迭代器）
.nth(n)                          // 取第 n 个，→ Option<T>
.partition(|x| *x > 0)         // 按条件分成两个集合 (Vec, Vec)
.unzip()                         // Iterator<(A,B)> → (Vec<A>, Vec<B>)
```

---

## 十二、类型转换速查表

### 字符串相关转换

| 从 → 到 | 方法 / 写法 | 是否拷贝 |
|---|---|---|
| `&str` → `String` | `s.to_string()` 或 `s.to_owned()` | 是 |
| `String` → `&str` | `&s` 或 `s.as_str()` | 否（借用） |
| `String` → `Vec<u8>` | `s.into_bytes()` | 否（零拷贝，消耗 s） |
| `String` → `&[u8]` | `s.as_bytes()` | 否（借用） |
| `Vec<u8>` → `String` | `String::from_utf8(v)?` | 否（零拷贝，消耗 v） |
| `Vec<u8>` → `String`（含非 UTF-8）| `String::from_utf8_lossy(&v)` | 是（Cow） |
| `&[u8]` → `&str` | `std::str::from_utf8(bytes)?` | 否（借用） |
| `&str` → `&[u8]` | `s.as_bytes()` | 否（借用） |
| `char` → `String` | `c.to_string()` | 是 |
| `String` → `Vec<char>` | `s.chars().collect()` | 是 |
| `Vec<char>` → `String` | `chars.iter().collect::<String>()` | 是 |

### 集合相关转换

| 从 → 到 | 方法 / 写法 | 备注 |
|---|---|---|
| `Vec<T>` → `&[T]` | `&v` 或 `v.as_slice()` | 借用 |
| `Vec<T>` → `Box<[T]>` | `v.into_boxed_slice()` | 缩容到恰好大小 |
| `&[T]` → `Vec<T>` | `s.to_vec()` | 拷贝（需 T: Clone） |
| `[T; N]` → `Vec<T>` | `Vec::from(arr)` 或 `arr.to_vec()` | 拷贝 |
| `[T; N]` → `&[T]` | `&arr` 或 `arr.as_slice()` | 借用 |
| `Vec<T>` → `HashSet<T>` | `v.into_iter().collect()` | 去重 |
| `HashSet<T>` → `Vec<T>` | `s.into_iter().collect()` | 顺序不定 |
| `Vec<(K,V)>` → `HashMap<K,V>` | `v.into_iter().collect()` | — |
| `HashMap<K,V>` → `Vec<(K,V)>` | `m.into_iter().collect()` | 顺序不定 |

### 数值相关转换

| 从 → 到 | 方法 / 写法 | 备注 |
|---|---|---|
| `i32` → `f64` | `x as f64` | 无 panic，可能精度损失 |
| `f64` → `i32` | `x as i32` | 截断小数，溢出饱和 |
| `i32` → `i64` | `x as i64` 或 `i64::from(x)` | 后者更安全 |
| `i64` → `i32` | `x as i32` 或 `i32::try_from(x)?` | try_from 溢出返回 Err |
| `usize` → `u32` | `u32::try_from(x)?` | 推荐用 try_from |
| `&str` → `i32` | `"42".parse::<i32>()?` | → Result |
| `i32` → `String` | `42.to_string()` | — |
| `f64` → `String` | `format!("{:.2}", x)` | 控制精度 |

### Option / Result 互转

| 从 → 到 | 方法 | 备注 |
|---|---|---|
| `Option<T>` → `Result<T, E>` | `opt.ok_or(err)` | None → Err |
| `Option<T>` → `Result<T, E>` | `opt.ok_or_else(\|\| err)` | 惰性版本 |
| `Result<T, E>` → `Option<T>` | `res.ok()` | Err 被丢弃 |
| `Result<T, E>` → `Option<E>` | `res.err()` | Ok 被丢弃 |
| `Option<Result<T,E>>` → `Result<Option<T>,E>` | `.transpose()` | — |
| `Result<Option<T>,E>` → `Option<Result<T,E>>` | `.transpose()` | — |

### 通用转换 trait

```rust
// From / Into：无损转换（不会失败）
let s = String::from("hello");      // From<&str> for String
let n = i64::from(42i32);           // From<i32> for i64
let n: i64 = 42i32.into();          // Into<i64> for i32（自动由 From 推导）

// TryFrom / TryInto：可能失败的转换
let n = i32::try_from(1000i64)?;    // → Result<i32, TryFromIntError>
let n: i32 = 1000i64.try_into()?;  // 同上

// AsRef / AsMut：廉价借用转换
fn print(s: impl AsRef<str>) {      // 同时接受 &str 和 &String
    println!("{}", s.as_ref());
}

// ToString / Display
42.to_string()                       // 任何实现 Display 的类型都自动有 to_string()
```

---

## 附：常用 trait 速查

| Trait | 含义 | 典型用途 |
|---|---|---|
| `Clone` | 显式深拷贝 `.clone()` | 需要复制堆数据时 |
| `Copy` | 隐式位拷贝（栈上赋值不移动） | 数值、bool、char、引用 |
| `Debug` | `{:?}` 格式化 | 调试打印 |
| `Display` | `{}` 格式化 | 用户可见输出 |
| `PartialEq` / `Eq` | `==` 运算符 | 相等判断 |
| `PartialOrd` / `Ord` | `<` `>` 运算符 | 比较、排序 |
| `Hash` | 可用作 `HashMap` 的 Key | 哈希计算 |
| `Default` | `T::default()` | 提供零值 |
| `From` / `Into` | 无损类型转换 | 构造函数替代 |
| `TryFrom` / `TryInto` | 可能失败的转换 | 数值范围检查 |
| `Deref` | 自动解引用（`String` → `&str`）| 智能指针、字符串 |
| `Iterator` | 实现 `.next()` 即获得全部迭代器方法 | 自定义迭代器 |
