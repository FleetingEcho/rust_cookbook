# Rust 新手踩坑指南

## 一、看返回值类型决定怎么用

这是最核心的规律，遇到不会用的方法，先看它返回什么。

```
方法返回值
├── 直接值 (String / usize / bool)   → 直接用
├── Option<T>                         → unwrap 系 / if let / match
├── Result<T, E>                      → ? / unwrap 系 / match
├── &T / &str                         → 注意生命周期，跨域要 .to_string()
├── Iterator                          → 链式处理，最后 collect / sum / count
└── impl Trait                        → 能用不能命名，存储要 Box<dyn Trait>
```

---

## 二、Option 和 Result

### Option — 可能没有值

```rust
"hello".find('z')       // Option<usize>，找不到返回 None
vec![1,2,3].first()     // Option<&i32>，空 vec 返回 None
```

**处理方式：**

```rust
let result = "hello".find('l');

result.unwrap()               // 直接拿值，None 时 panic（调试用）
result.unwrap_or(0)           // None 给默认值
result.expect("找不到")       // panic 并附上说明
result.unwrap_or_default()    // None 给类型默认值（0 / "" / false）

if let Some(i) = result {     // 推荐：安全解包
    println!("{i}");
}
```

### Result — 可能失败，且知道原因

```rust
"abc".parse::<i32>()    // Result<i32, ParseIntError>
File::open("a.txt")     // Result<File, io::Error>
```

**处理方式：**

```rust
let n = "42".parse::<i32>();

n.unwrap()              // 失败 panic
n.unwrap_or(0)          // 失败给默认值
n.ok()                  // 转成 Option，丢弃错误信息

// 在返回 Result 的函数里，? 是最常用的
fn parse() -> Result<i32, _> {
    let n = "42".parse::<i32>()?;  // 失败自动向上传播
    Ok(n)
}
```

### Option 和 Result 互转

```rust
// Option → Result
let opt: Option<i32> = Some(1);
opt.ok_or("没有值")          // Result<i32, &str>，None 变成 Err
opt.ok_or_else(|| "没有值")  // 同上，错误值懒求值

// Result → Option
let res: Result<i32, &str> = Ok(1);
res.ok()    // Option<i32>，丢弃错误，Ok → Some，Err → None
res.err()   // Option<&str>，丢弃正常值，Err → Some，Ok → None
```

实际场景：在返回 `Option` 的函数里，遇到 `Result` 用 `.ok()?` 处理：

```rust
fn first_number(s: &str) -> Option<i32> {
    s.split(',').next()?.parse().ok()  // 找不到或解析失败都返回 None
}
```

---

## 三、需要 collect() 的情况

**split / chars / lines / iter 这类"拆成多个"的方法，返回的是懒迭代器，不是数组。**

```rust
// ❌ 新手误区：以为 split 直接返回 Vec
let parts = "a,b,c".split(',');   // 这是 Split 迭代器，不是 Vec

// ✅ 正确：加 collect
let parts: Vec<&str> = "a,b,c".split(',').collect();
```

需要 collect 的常见方法：

```rust
"a,b,c".split(',').collect::<Vec<_>>()
"hello".chars().collect::<Vec<_>>()
"a\nb\nc".lines().collect::<Vec<_>>()
vec![1,2,3].iter().map(|x| x * 2).collect::<Vec<_>>()
```

**为什么是懒的？** 因为链式操作中间不需要创建临时数组，性能更好，最后一次性 collect。

### collect() 返回什么？

collect 本身不固定返回某种类型，**你指定什么类型它就收集成什么**：

```rust
let v: Vec<&str>          = "a,b,c".split(',').collect();  // Vec
let s: String             = ['h','i'].into_iter().collect(); // String
let set: HashSet<i32>     = vec![1,1,2,3].into_iter().collect(); // HashSet，自动去重
let map: HashMap<&str,i32> = vec![("a",1),("b",2)].into_iter().collect(); // HashMap
```

所以 collect 必须有类型提示，否则编译器不知道收集成哪种容器。

### 迭代器的终结操作（消费迭代器，不需要 collect）

不是所有迭代器操作都要 collect，以下方法直接返回最终值：

```rust
let v = vec![1, 2, 3, 4, 5];

v.iter().count()                    // usize，元素个数
v.iter().sum::<i32>()               // i32，求和
v.iter().product::<i32>()           // i32，求积
v.iter().max()                      // Option<&i32>，最大值
v.iter().min()                      // Option<&i32>，最小值
v.iter().any(|&x| x > 3)           // bool，有没有满足条件的
v.iter().all(|&x| x > 0)           // bool，是否全部满足
v.iter().find(|&&x| x > 3)         // Option<&i32>，第一个满足的
v.iter().position(|&x| x == 3)     // Option<usize>，第一个满足的下标
v.iter().for_each(|x| println!("{x}")); // 遍历，无返回值
```

### 链式适配器（中间操作，返回仍是迭代器）

```rust
let v = vec![1, 2, 3, 4, 5];

// filter_map：filter + map 合一，返回 None 的直接跳过
let result: Vec<i32> = v.iter()
    .filter_map(|&x| if x > 2 { Some(x * 10) } else { None })
    .collect();  // [30, 40, 50]

// enumerate：带下标
for (i, val) in v.iter().enumerate() {
    println!("{i}: {val}");
}

// zip：两个迭代器合并成元组
let keys = vec!["a", "b", "c"];
let vals = vec![1, 2, 3];
let map: HashMap<_,_> = keys.into_iter().zip(vals).collect();

// flatten：嵌套展开
let nested = vec![vec![1,2], vec![3,4]];
let flat: Vec<i32> = nested.into_iter().flatten().collect(); // [1,2,3,4]

// take / skip：取前 n 个 / 跳过前 n 个
let first3: Vec<i32> = v.iter().copied().take(3).collect();  // [1,2,3]
let after2: Vec<i32> = v.iter().copied().skip(2).collect();  // [3,4,5]

// chain：拼接两个迭代器
let a = vec![1, 2];
let b = vec![3, 4];
let all: Vec<i32> = a.iter().chain(b.iter()).copied().collect(); // [1,2,3,4]
```

---

## 四、Vec 的 map / filter — 要先转迭代器

TS 的数组直接有 `.map()` `.filter()`，Rust 的 Vec 没有，要先 `.iter()`：

```rust
let v = vec![1, 2, 3, 4, 5];

// TS: v.filter(x => x > 2).map(x => x * 10)
let result: Vec<i32> = v.iter()
    .filter(|&&x| x > 2)
    .map(|&x| x * 10)
    .collect();
```

固定套路：**`vec` → `.iter()` → 链式操作 → `.collect()`**

### iter() 的三种形式

| 方法 | 元素类型 | 用途 |
|------|---------|------|
| `.iter()` | `&T` | 只读借用，原 vec 之后还能用 |
| `.iter_mut()` | `&mut T` | 原地修改 |
| `.into_iter()` | `T` | 消耗 vec，拿到所有权 |

```rust
// iter()：v 之后还能用，但闭包参数要写 &x
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();

// into_iter()：v 之后不能用，但闭包参数直接写 x，更简洁
let doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
```

**新手建议：先统一用 `into_iter()`，闭包参数不用加 `&`，少一个心智负担。**

---

## 五、字符串：&str vs String

| | `&str` | `String` |
|--|--------|---------|
| 存储 | 栈上的引用（指向某处内存） | 堆上，有所有权 |
| 字面量 | `"hello"` 就是 `&str` | 需要 `.to_string()` 或 `String::from()` |
| 修改 | 不可变 | 可变（mut） |

**函数参数推荐用 `&str`，更通用：**

```rust
fn greet(s: &str) { println!("{s}"); }

let owned = String::from("hello");
greet(&owned);    // ✅ String 自动 deref 成 &str
greet("world");   // ✅ &str 直接传

// 如果参数是 &String，就只能传 &String，字面量不行
```

**字符串互转：**

```rust
"hello".to_string()          // &str → String
String::from("hello")        // &str → String
owned.as_str()               // String → &str
&owned                       // &String，大多数场景自动 deref 成 &str
```

### 常用字符串方法对比

| 操作 | TypeScript | Rust |
|------|-----------|------|
| 替换全部 | `replaceAll("l","L")` | `replace("l","L")` |
| 只替换第一个 | `replace("l","L")` | `replacen("l","L",1)` |
| 分割 | `split(',')` → 数组 | `split(',').collect()` → Vec |
| 包含 | `includes("lo")` | `contains("lo")` |
| 开头 | `startsWith("he")` | `starts_with("he")` |
| 结尾 | `endsWith("lo")` | `ends_with("lo")` |
| 去空格 | `trim()` | `trim()` |
| 转大写 | `toUpperCase()` | `to_uppercase()` |

---

## 六、HashMap 初始化

```rust
use std::collections::HashMap;

// 推荐：最简洁，Rust 1.56+
let map = HashMap::from([("x", 1_i32), ("y", 2)]);

// 也可以：collect，但必须给类型提示（collect 太泛了）
let map: HashMap<_, _> = [("x", 1_i32), ("y", 2)].into_iter().collect();
```

**为什么 collect 必须加类型提示？** 同一个迭代器可以 collect 成 Vec / HashMap / HashSet 等，编译器不知道你要哪个。

### 常用操作

| 操作 | TypeScript | Rust |
|------|-----------|------|
| 取值 | `map.get("x")` | `map.get("x")` → `Option<&V>` |
| 默认值 | `map.get("x") ?? 0` | `map.get("x").copied().unwrap_or(0)` |
| 有无 key | `map.has("x")` | `map.contains_key("x")` |
| 不存在才插入 | 手动判断 | `map.entry("x").or_insert(0)` |
| 删除 | `map.delete("x")` | `map.remove("x")` |

**entry 模式，统计词频经典写法：**

```rust
let mut freq: HashMap<&str, i32> = HashMap::new();
for word in ["a", "b", "a", "c", "a"] {
    *freq.entry(word).or_insert(0) += 1;
}
// {"a": 3, "b": 1, "c": 1}
```

---

## 七、类型推导规律

**函数体内基本不用写，函数签名必须写：**

```rust
// 函数签名：必须写
fn add(x: i32, y: i32) -> i32 { x + y }

// 函数体内：大多不用写
let name = "hello";           // 推导出 &str
let flag = true;              // 推导出 bool
let nums = vec![1, 2, 3];    // 推导出 Vec<i32>

// 跨行推导也行
let mut v = Vec::new();
v.push(1_i32);   // 这里才确定类型，上面不用标注
```

**必须手动指定类型的情况：**

1. 函数签名（强制）
2. `collect()` — 容器类型不唯一
3. 数字字面量有歧义 — `1` 默认 i32，需要其他类型时加后缀 `1_i64`
4. 空容器 — `Vec::new()` 后面没有 push，编译器无从推导

**`_` 占位符让编译器填：**

```rust
let map: HashMap<_, _> = ...   // 只告诉编译器是 HashMap，具体类型推导
.collect::<Vec<_>>()           // turbofish 写法，内部类型推导
```

---

## 八、总结速查

```
遇到方法不知道怎么用？

1. 返回 Option/Result？
   → 有失败可能，用 unwrap_or / ? / if let 处理

2. 返回 Iterator？
   → 懒的，链式处理完加 .collect() 得到 Vec

3. 返回 &str / &T？
   → 是借用，想独立存活要 .to_string() / .clone()

4. 编译器说类型不够？
   → collect() 加类型提示，或数字字面量加后缀

5. Vec 想 map/filter？
   → 先 .iter() 或 .into_iter()，最后 .collect()
```