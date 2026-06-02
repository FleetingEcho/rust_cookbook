# Rust 数据结构实用技巧

---

## 一、Vec

### 1. 提前分配容量

```rust
// ❌ 频繁 push 导致多次扩容
let mut v = Vec::new();
for i in 0..1000 { v.push(i); }

// ✅ 提前分配，一次搞定
let mut v = Vec::with_capacity(1000);
for i in 0..1000 { v.push(i); }
```

扩容是 O(n) 的（要拷贝所有已有元素），提前分配能避免。

### 2. 用 swap_remove 快速删除（不关心顺序时）

```rust
let mut v = vec![1, 2, 3, 4, 5];

// remove：删除中间元素，后面的全往前移 O(n)
v.remove(2);  // [1, 2, 4, 5] — 把 3 删了，4 和 5 前移

// swap_remove：把要删的和最后一个交换，再 pop 掉 O(1)
v.swap_remove(2);  // [1, 2, 5, 4] — 把 3 和 5 交换，再 pop 掉 3
```

不关心元素顺序时，`swap_remove` 比 `remove` 快得多。

### 3. retain：原地过滤

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

// 保留偶数
v.retain(|&x| x % 2 == 0);  // v 变成 [2, 4, 6]

// 对比：filter + collect 会创建新 vec，retain 是原地修改
```

### 4. dedup：去重（连续重复的）

```rust
let mut v = vec![1, 1, 2, 2, 2, 3, 1, 1];
v.dedup();  // [1, 2, 3, 1] — 只去连续重复的，最后一个 1 没去掉

// 先排序再去重才能去掉所有重复
let mut v = vec![1, 1, 2, 2, 2, 3, 1, 1];
v.sort();
v.dedup();  // [1, 2, 3]
```

### 5. extend 批量追加 vs push 单个

```rust
let mut v = vec![1, 2, 3];

// 追加单个
v.push(4);

// 追加迭代器（省去循环）
v.extend([5, 6, 7]);

// 追加另一个 vec（other 被清空，所有权转移）
let mut other = vec![8, 9];
v.append(&mut other);
```

### 6. split_off：从中间切开

```rust
let mut v = vec![1, 2, 3, 4, 5];
let v2 = v.split_off(3);  // v = [1,2,3], v2 = [4,5]
```

### 7. resize：快速填充

```rust
let mut v = vec![1, 2, 3];
v.resize(5, 0);   // [1, 2, 3, 0, 0] — 不够的补 0
v.resize(2, 0);   // [1, 2] — 多余的截掉
```

### 8. Vec 做栈

```rust
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.pop();   // Some(2) — stack 剩 [1]
stack.last();  // Some(&1) — 看一眼栈顶不移除
```

### 9. windows / chunks：滑动窗口 / 分块

```rust
let v = vec![1, 2, 3, 4, 5];

// chunks：分成大小为 2 的块
for chunk in v.chunks(2) {
    // [1,2], [3,4], [5]
}

// windows：大小为 2 的滑动窗口
for win in v.windows(2) {
    // [1,2], [2,3], [3,4], [4,5]
}
```

### 10. Vec 和数组的相互转换

```rust
// 数组 → Vec
let arr: [i32; 3] = [1, 2, 3];
let v: Vec<i32> = arr.to_vec();

// Vec → 数组（长度不匹配会 Err）
let v = vec![1, 2, 3];
let arr: [i32; 3] = v.try_into().unwrap();
```

---

## 二、HashMap 与 BTreeMap

### 11. entry 模式：最优雅的"不存在才插入"

```rust
let mut map: HashMap<&str, i32> = HashMap::new();

// 经典场景：词频统计
for word in ["a", "b", "a", "c"] {
    *map.entry(word).or_insert(0) += 1;
}
// map = {"a": 2, "b": 1, "c": 1}
```

### 12. and_modify：存在就改，不存在就不管

```rust
let mut map = HashMap::from([("x", 1), ("y", 2)]);

// 如果 "x" 存在就加 1
map.entry("x").and_modify(|v| *v += 1);
// map = {"x": 2, "y": 2}

// 如果 "z" 存在就加 1，不存在就插入 1
map.entry("z").and_modify(|v| *v += 1).or_insert(1);
// map = {"x": 2, "y": 2, "z": 1}
```

### 13. entry 返回的是引用，可以继续操作

```rust
let mut map: HashMap<&str, Vec<i32>> = HashMap::new();

// 不存在就插入空 vec，然后 push
map.entry("nums").or_default().push(1);
// 等价于：
map.entry("nums").or_insert_with(Vec::new).push(1);
```

### 14. 不存在时想执行复杂逻辑：or_insert_with

```rust
// or_insert：参数是值，不管用不用都算好
map.entry("key").or_insert(expensive_computation());

// or_insert_with：参数是闭包，只有需要时才执行
map.entry("key").or_insert_with(|| expensive_computation());
```

### 15. 根据旧值更新

```rust
// 方式一：取出改
if let Some(v) = map.get_mut("x") {
    *v += 1;
}

// 方式二：remove 再 insert，返回旧值
let old = map.insert("x", 99);  // old = Some(旧值) 或 None
```

### 16. HashMap 的一行初始化

```rust
use std::collections::HashMap;

// 最简洁（Rust 1.56+）
let map = HashMap::from([("x", 1), ("y", 2)]);

// collect 方式（需要类型提示）
let map: HashMap<&str, i32> = [("x", 1), ("y", 2)].into_iter().collect();
```

### 17. 遍历顺序

```rust
let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

// HashMap：不保证顺序（每次可能不一样）
for (k, v) in &map { println!("{k}: {v}"); }

// BTreeMap：按键排序（确定顺序）
use std::collections::BTreeMap;
let bmap: BTreeMap<_, _> = map.into_iter().collect();
for (k, v) in &bmap { println!("{k}: {v}"); }  // 总是 a, b, c
```

### 18. get 的常见组合

```rust
let map = HashMap::from([("x", 42)]);

// 取值，没有给默认
map.get("x").copied().unwrap_or(0);

// 引用转值
map.get("x");           // Option<&i32>
map.get("x").copied();  // Option<i32> — 如果 i32: Copy
map.get("x").cloned();  // Option<i32> — 如果 i32: Clone
```

### 19. 只想检查 key 是否存在

```rust
// 用 contains_key，别用 get
if map.contains_key("x") { /* ... */ }
// 比 get().is_some() 更语义化，而且不 borrow value
```

---

## 三、HashSet 与 BTreeSet

### 20. HashSet 基本操作

```rust
use std::collections::HashSet;

let mut set: HashSet<i32> = HashSet::new();

set.insert(1);    // true  — 插入成功
set.insert(1);    // false — 已存在，不重复插入
set.contains(&1); // true
set.remove(&1);   // true  — 删除成功，元素存在才返回 true

// 一行初始化
let set = HashSet::from([1, 2, 3]);
let set: HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect();  // 自动去重
```

### 21. 集合运算：并集、交集、差集

```rust
let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
let b: HashSet<i32> = [2, 3, 4].into_iter().collect();

// 并集（在 a 或在 b）
let union: HashSet<_> = a.union(&b).copied().collect();        // {1,2,3,4}

// 交集（同时在 a 和 b）
let inter: HashSet<_> = a.intersection(&b).copied().collect(); // {2,3}

// 差集（在 a 不在 b）
let diff: HashSet<_> = a.difference(&b).copied().collect();    // {1}

// 对称差（只在其中一个里）
let sym: HashSet<_> = a.symmetric_difference(&b).copied().collect(); // {1,4}
```

### 22. 子集与超集判断

```rust
let small: HashSet<i32> = [1, 2].into_iter().collect();
let big: HashSet<i32> = [1, 2, 3].into_iter().collect();

small.is_subset(&big);     // true
big.is_superset(&small);   // true
small.is_disjoint(&big);   // false — 有交集
```

### 23. BTreeSet：有序集合

```rust
use std::collections::BTreeSet;

let mut bset: BTreeSet<i32> = BTreeSet::from([3, 1, 4, 1, 5, 9]);
// 自动去重且保持升序排列
for x in &bset { print!("{x} "); }  // 1 3 4 5 9

// 范围查询（BTreeSet 独有）
use std::ops::Bound::Included;
for x in bset.range(3..=6) { print!("{x} "); }  // 3 4 5

// 最小/最大值
bset.iter().next();      // Some(&1) — 最小
bset.iter().next_back(); // Some(&9) — 最大（BTreeSet 支持双端迭代）
```

### 24. 用 HashSet 快速去重 Vec

```rust
let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

// 方式一：通过 HashSet（不保留顺序）
let unique: Vec<i32> = v.iter().copied().collect::<HashSet<_>>().into_iter().collect();

// 方式二：保留顺序去重（用 seen 集合）
let mut seen = HashSet::new();
let unique: Vec<_> = v.into_iter().filter(|x| seen.insert(*x)).collect();
// [3, 1, 4, 5, 9, 2, 6] — 保留第一次出现的顺序
```

---

## 四、VecDeque 与 BinaryHeap

### 25. VecDeque：高效双端队列

```rust
use std::collections::VecDeque;

let mut dq: VecDeque<i32> = VecDeque::new();

// 两端都可以高效插入/删除（O(1)）
dq.push_back(1);   // [1]
dq.push_back(2);   // [1, 2]
dq.push_front(0);  // [0, 1, 2]

dq.pop_front();    // Some(0) — 队头取出（FIFO）
dq.pop_back();     // Some(2) — 队尾取出

// 随机访问也支持，但 Vec 更快
dq[0];             // 0（index 访问）

// Vec ↔ VecDeque 互转
let v = vec![1, 2, 3];
let dq: VecDeque<_> = v.into();
let v: Vec<_> = dq.into();
```

`Vec` 只有尾部 O(1)，头部操作是 O(n)；需要频繁头部操作时用 `VecDeque`。

### 26. BinaryHeap：最大堆（优先队列）

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);
heap.push(2);

heap.peek();  // Some(&5) — 查看堆顶（最大值），不移除
heap.pop();   // Some(5)  — 取出最大值
heap.pop();   // Some(3)

// 用 extend 批量插入
let mut heap: BinaryHeap<_> = vec![3, 1, 4, 1, 5].into_iter().collect();

// 按优先级取出所有元素（降序）
while let Some(top) = heap.pop() {
    print!("{top} ");  // 5 4 3 1 1
}
```

### 27. BinaryHeap 转最小堆：用 Reverse

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.push(Reverse(5));

min_heap.pop();  // Some(Reverse(1)) — 最小值优先
// 取出真实值：
let Reverse(val) = min_heap.pop().unwrap();
```

---

## 五、String 与 &str

### 28. 字符串拼接的各种姿势

```rust
let a = "hello";
let b = "world";

// &str + &str → String
let s = format!("{a} {b}");

// String + &str → String（会移动第一个字符串）
let s1 = String::from("hello");
let s = s1 + " world";  // s1 被移动了！之后不可用

// 多个字符串拼接
let parts = ["a", "b", "c"];
let s = parts.join(",");  // "a,b,c"

// 用 collect 拼接字符
let s: String = ['h', 'e', 'l', 'l', 'o'].into_iter().collect();
```

### 29. 字符串切片小心！是字节索引不是字符索引

```rust
let s = "你好";

// 中文字符 1 个字 = 3 字节
println!("{}", s.len());           // 6（字节数）
println!("{}", s.chars().count()); // 2（字符数）

// ❌ 直接按字节索引会崩溃
// &s[0..1] → panic!（切到了字符中间）

// ✅ 安全方式：按字符取
for ch in s.chars() { print!("{ch} "); }
println!("{}", s.chars().nth(1).unwrap());  // '好'

// ✅ 取前 N 个字符
let first_n: String = s.chars().take(1).collect();  // "你"
```

### 30. 字符串反转

```rust
let s = "hello";
let rev: String = s.chars().rev().collect();  // "olleh"

// ❌ 不能用 s.bytes().rev() — 那会反转字节，中文就坏了
```

### 31. trim 的各种变体

```rust
let s = "  hello world  ";

s.trim();                         // "hello world"         首尾
s.trim_start();                   // "hello world  "       左边
s.trim_end();                     // "  hello world"       右边
"---hello---".trim_matches('-');  // "hello"               匹配字符
```

### 32. split 的多种模式

```rust
"a,b,c".split(',');            // ["a", "b", "c"]
"a,b,c".splitn(2, ',');        // ["a", "b,c"]  只分割前 n-1 次
"a,b,c".rsplit(',');           // ["c", "b", "a"]  从右往左
"a1b2c3".split(|c: char| c.is_ascii_digit()); // ["a", "b", "c"]

// 分割后别忘了 collect
let parts: Vec<&str> = "a,b,c".split(',').collect();
```

### 33. contains / starts_with / ends_with

```rust
"hello world".contains("world");  // true
"hello".starts_with("he");        // true
"hello".ends_with("lo");          // true

// 注意是字节搜索，不是正则
```

### 34. 查找与替换

```rust
"hello".find('e');              // Some(1)
"hello".rfind('l');             // Some(3) — 从右找
"hello".replace("l", "L");      // "heLLo" — 全部替换
"hello".replacen("l", "L", 1); // "heLlo" — 只替换第一个
```

### 35. 判空与修剪

```rust
"".is_empty();              // true
"  ".is_empty();            // false（有空格）
"  ".trim().is_empty();     // true
```

### 36. 数字转字符串的格式控制

```rust
let n = 255;

format!("{n}");       // "255"
format!("{n:#x}");    // "0xff"      带 0x 前缀十六进制
format!("{n:x}");     // "ff"        小写十六进制
format!("{n:X}");     // "FF"        大写十六进制
format!("{n:o}");     // "377"       八进制
format!("{n:b}");     // "11111111"  二进制
format!("{n:#010b}"); // "0b11111111" 带前缀 10 位宽

let pi = 3.1415926;
format!("{pi:.2}");   // "3.14"
format!("{pi:>8.2}"); // "    3.14"  右对齐总宽 8
format!("{pi:<8.2}"); // "3.14    "  左对齐
format!("{pi:^8.2}"); // "  3.14  "  居中
```

### 37. String 和 &str 作为函数参数

```rust
// ✅ 推荐：参数用 &str，最通用
fn process(s: &str) { let _ = s; }

process("hello");               // &str 直接传
process(&String::from("hi"));   // &String 自动 deref 成 &str

// ❌ 参数用 &String 会限制调用方
fn process_bad(s: &String) { let _ = s; }
// process_bad("hello");         // ❌ &str 不能直接转 &String
```

---

## 六、数字操作

### 38. 数字字面量加分隔符

```rust
let n = 1_000_000;   // 100 万，_ 是视觉分隔符
let m = 0.000_001;
let hex = 0xFF_FF;   // 十六进制也可以用
```

### 39. 类型后缀

```rust
let n = 42;        // 默认 i32
let n = 42_i64;    // 指定 i64
let n = 42_u8;     // u8
let f = 3.14;      // 默认 f64
let f = 3.14_f32;  // f32

// 什么时候必须加？编译器无法推断时
vec![1, 2, 3].iter().sum::<i32>();
```

### 40. 安全算术三兄弟

```rust
let a: u32 = u32::MAX;
let b: u32 = 1;

// 普通加法：debug 溢出 panic，release 回绕
// a + b;

// wrapping：overflow 时回绕（不 panic）
a.wrapping_add(b);  // 0

// saturating：overflow 时停在边界值
a.saturating_add(b);  // u32::MAX

// checked：返回 Option，None 表示溢出
a.checked_add(b);  // None
```

### 41. as 转换 vs TryFrom

```rust
// as：无条件转换（可能截断）
let x: i32 = 256;
let y: u8 = x as u8;  // 0！截断了，编译器不会告诉你

// TryFrom：安全转换，可能失败
let w = u8::try_from(256_i32);  // Err("out of range")
let w = u8::try_from(255_i32);  // Ok(255)
```

`as` 是"截断我也认"；`TryFrom` 是"转不了就报错"。

### 42. 字符串解析成数字

```rust
"42".parse::<i32>();                   // Ok(42)
"42".parse::<i32>().unwrap_or(0);
"abc".parse::<i32>();                  // Err(ParseIntError)

// 带进制解析
i32::from_str_radix("ff", 16);        // Ok(255)
i32::from_str_radix("1010", 2);       // Ok(10)
```

### 43. 绝对值 / 幂 / 平方根

```rust
(-5_i32).abs();  // 5
2_i32.pow(10);   // 1024
4_f64.sqrt();    // 2.0
4_f64.cbrt();    // 1.587...
// i32 没有 sqrt，需要先转 f64
let n: i32 = 16;
(n as f64).sqrt() as i32;  // 4
```

### 44. 四舍五入

```rust
let f = 3.14159;
f.round();  // 3.0
f.floor();  // 3.0
f.ceil();   // 4.0
f.trunc();  // 3.0（截断小数部分）
(f * 100.0).round() / 100.0;  // 保留 2 位小数

f as i32;         // 3（直接截断）
f.round() as i32; // 3（先四舍五入再转）
```

### 45. 数字比较的坑

```rust
// f64 不能直接 == 比较（浮点精度问题）
let a = 0.1 + 0.2;
let b = 0.3;
// a == b → false！

// ✅ 应该用差值比较
(a - b).abs() < 1e-10;

// 排序时 NaN 的问题
let mut v = vec![1.0_f64, f64::NAN, 3.0];
// v.sort();  // ❌ panic! f64 不实现 Ord
v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
```

---

## 七、数组与切片

### 46. 数组初始化

```rust
// 相同值填充
let arr = [0; 10];      // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// 逐字列出
let arr = [1, 2, 3, 4, 5];

// 多维数组
let matrix = [[0; 3]; 3];  // 3x3 的零矩阵
```

### 47. 切片取部分

```rust
let v = vec![1, 2, 3, 4, 5];

&v[1..3];   // [2, 3]    下标 1 到 2
&v[..3];    // [1, 2, 3] 从头到下标 2
&v[2..];    // [3, 4, 5] 从下标 2 到尾
&v[..];     // 整条切片

// 安全取（不会 panic）
v.get(0..3);   // Some(&[1, 2, 3])
v.get(0..10);  // None
```

### 48. split_at：从中间安全分割

```rust
let v = [1, 2, 3, 4, 5];
let (left, right) = v.split_at(3);
// left = [1, 2, 3], right = [4, 5]

// Rust 1.80+：不会 panic 的版本
// v.split_at_checked(3)
```

### 49. 二分查找

```rust
let v = vec![1, 3, 5, 7, 9];

v.binary_search(&5);  // Ok(2)  — 找到了，返回下标
v.binary_search(&4);  // Err(2) — 没找到，返回该插入的位置

// 注意：必须有序！否则结果不对
```

### 50. 首尾元素

```rust
let v = vec![1, 2, 3];

v.first();       // Some(&1)
v.last();        // Some(&3)
v.first_mut();   // Some(&mut 1)
v.last_mut();    // Some(&mut 3)
```

### 51. contains 检查是否存在

```rust
let v = vec![1, 2, 3];
v.contains(&2);  // true — 注意是引用！

// 大量查找时，用 HashSet 更快（O(1) vs O(n)）
let set: HashSet<_> = v.iter().collect();
set.contains(&2);  // true
```

---

## 八、Option 与 Result 进阶

### 52. Option 的 map / and_then / or_else 链条

```rust
// map：Some 时变换，None 保持 None
Some(1).map(|x| x + 1);  // Some(2)

// and_then：返回 Option 的变换（flatMap）
Some(1).and_then(|x| if x > 0 { Some(x + 1) } else { None });
// map 的闭包返回 T，and_then 返回 Option<T>

// or_else：None 时提供备选
None::<i32>.or_else(|| Some(42));  // Some(42)
Some(1).or_else(|| Some(42));      // Some(1) — 不执行
```

### 53. Result 的 map / map_err / and_then

```rust
// map：Ok 时变换
Ok::<i32, &str>(1).map(|x| x + 1);  // Ok(2)

// map_err：Err 时变换错误类型
Err::<i32, &str>("error").map_err(|e| format!("{e}"));  // Err(String)

// 实际场景：解析字符串
"42".parse::<i32>()
    .map(|n| n * 2)   // 成功就加倍
    .unwrap_or(0);    // 失败给 0
```

### 54. Option 和 Result 互转实战

```rust
// 从字符串数组中解析数字，跳过无效的
let inputs = vec!["1", "abc", "2", "xyz"];

// filter_map：过滤掉 None
let nums: Vec<i32> = inputs.iter()
    .filter_map(|s| s.parse::<i32>().ok())  // Result → Option，丢掉 Err
    .collect();  // [1, 2]
```

### 55. ok_or / ok_or_else：Option → Result

```rust
let config: Option<&str> = None;

// Option → Result，带错误信息
config.ok_or("config missing");                           // Err("config missing")
config.ok_or_else(|| format!("config {} missing", "db")); // 懒求值

// 搭配 ? 使用
fn get_config() -> Result<&'static str, String> {
    let val: Option<&'static str> = Some("value");
    val.ok_or_else(|| "config missing".to_string())
}
```

### 56. transpose：Option<Result<T>> ↔ Result<Option<T>>

```rust
// Option<Result> → Result<Option>，外层错误优先
let val: Option<Result<i32, _>> = Some("42".parse());
let val: Result<Option<i32>, _> = val.transpose();
// Ok(Some(42))

// 实用场景：key 可能不存在，值也可能解析失败
fn parse_val(raw: Option<&str>) -> Result<Option<i32>, std::num::ParseIntError> {
    raw.map(|s| s.parse::<i32>()).transpose()
}
```

---

## 九、迭代器技巧

### 57. filter_map：filter + map 合一

```rust
let v = vec!["1", "abc", "2", "xyz"];

// 一步过滤+转换
let nums: Vec<i32> = v.iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();  // [1, 2]
```

### 58. enumerate：带下标遍历

```rust
let fruits = vec!["apple", "banana", "cherry"];

for (i, fruit) in fruits.iter().enumerate() {
    println!("{i}: {fruit}");  // 0: apple, 1: banana, 2: cherry
}

// 从非零下标开始
for (i, fruit) in fruits.iter().enumerate().map(|(i, v)| (i + 1, v)) {
    println!("{i}: {fruit}");  // 1: apple, 2: banana, 3: cherry
}
```

### 59. zip：两个迭代器并行走

```rust
let names = vec!["Alice", "Bob", "Carol"];
let scores = vec![90, 80, 95];

// 配对遍历（长度取短的那个）
for (name, score) in names.iter().zip(scores.iter()) {
    println!("{name}: {score}");
}

// collect 成 Vec of tuples
let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();

// unzip：zip 的逆操作
let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
let (keys, vals): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
// keys = ["a","b","c"], vals = [1,2,3]
```

### 60. chain：连接多个迭代器

```rust
let a = [1, 2, 3];
let b = [4, 5, 6];

let combined: Vec<_> = a.iter().chain(b.iter()).collect();
// [1, 2, 3, 4, 5, 6]

// 多个连接
let c = [7, 8];
let all: Vec<_> = a.iter().chain(b.iter()).chain(c.iter()).collect();
```

### 61. take_while / skip_while

```rust
let v = vec![1, 2, 3, 4, 5, 1, 2];

// take_while：遇到不满足条件立刻停，不看后面了
let taken: Vec<_> = v.iter().take_while(|&&x| x < 4).collect();
// [1, 2, 3] — 到 4 就停，后面的 1, 2 也没了

// skip_while：跳过满足条件的，之后全部保留
let skipped: Vec<_> = v.iter().skip_while(|&&x| x < 4).collect();
// [4, 5, 1, 2] — 跳过前三个，后面的 1, 2 也保留了
```

注意和 `filter` 的区别：`take_while` / `skip_while` 是**位置敏感**的，条件一旦不满足就停止。

### 62. peekable：向前看一步

```rust
let v = vec![1, 2, 3];
let mut iter = v.iter().peekable();

// peek 看下一个但不消费
if iter.peek() == Some(&&1) {
    println!("starts with 1");
}

// 此时再 next 才真正消费
iter.next();  // Some(&1)
iter.next();  // Some(&2)

// 实用场景：解析时需要提前判断下一个 token
```

### 63. flatten：展平嵌套

```rust
let nested = vec![
    vec![1, 2],
    vec![3, 4, 5],
    vec![],
];

nested.iter().flatten().collect::<Vec<_>>();
// [&1, &2, &3, &4, &5]

// Option 也能 flatten
let v = vec![Some(1), None, Some(2)];
v.into_iter().flatten().collect::<Vec<_>>();  // [1, 2]
```

### 64. partition：一次遍历分成两组

```rust
let v = vec![1, 2, 3, 4, 5];

let (even, odd): (Vec<i32>, Vec<i32>) = v.into_iter()
    .partition(|x| x % 2 == 0);
// even = [2, 4], odd = [1, 3, 5]

// 比 filter 两次快（只遍历一次）
```

### 65. scan：带状态扫描（累积值逐步输出）

```rust
let v = vec![1, 2, 3, 4, 5];

// 计算累积和
let sums: Vec<i32> = v.iter()
    .scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    })
    .collect();
// [1, 3, 6, 10, 15]
```

### 66. inspect：调试链式调用

```rust
let result: Vec<i32> = vec![1, 2, 3]
    .into_iter()
    .inspect(|x| println!("before map: {x}"))
    .map(|x| x * 10)
    .inspect(|x| println!("after map: {x}"))
    .collect();
// before map: 1 → after map: 10 → before map: 2 → ...
```

### 67. 迭代器的短路操作

```rust
let v = vec![1, 2, 3, 4, 5];

v.iter().any(|x| *x > 3);     // true  — 到 4 就停
v.iter().all(|x| *x > 0);     // true
v.iter().find(|&&x| x > 3);   // Some(&4) — 找到就停
v.iter().position(|&x| x > 3);// Some(3) — 返回下标
```

`any` / `all` / `find` / `position` 都是短路求值，对大集合能省很多时间。

### 68. fold / reduce：自定义聚合

```rust
let v = vec![1, 2, 3, 4, 5];

// fold：提供初始值
let sum = v.iter().fold(0, |acc, &x| acc + x);  // 15
let product = v.iter().fold(1, |acc, &x| acc * x); // 120

// reduce：第一个元素做初始值（返回 Option）
let sum = v.iter().copied().reduce(|acc, x| acc + x);  // Some(15)
[].iter().copied().reduce(|acc, x| acc + x);            // None
```

---

## 十、实用小技巧

### 69. 默认值的各种写法

```rust
let x: Option<i32> = None;

x.unwrap_or(0);              // 直接给默认值
x.unwrap_or_default();       // 类型默认值（0 / "" / false / empty vec）
x.unwrap_or_else(|| 1 + 1);  // 闭包懒求值
x.or(Some(0));               // 返回 Option，x 是 None 就给 Some(0)
x.or_else(|| Some(0));       // 闭包版
```

### 70. swap 交换两个变量

```rust
let mut a = 1;
let mut b = 2;
std::mem::swap(&mut a, &mut b);  // a=2, b=1

// Vec 内交换（按下标）
let mut v = vec![1, 2, 3];
v.swap(0, 2);  // [3, 2, 1]
```

### 71. take / replace 替换值

```rust
let mut x = String::from("hello");

// take：把值换成默认值（"" 对于 String），返回原值
let old = std::mem::take(&mut x);  // old = "hello", x = ""

// replace：把值换成新值，返回原值
let old = std::mem::replace(&mut x, String::from("world"));
// old = ""（上一行的空字符串）, x = "world"
```

### 72. 排序和比较

```rust
let mut v = vec![3, 1, 4, 1, 5];

v.sort();                      // [1, 1, 3, 4, 5] — 自然排序（稳定）
v.sort_by(|a, b| b.cmp(a));   // [5, 4, 3, 1, 1] — 逆序
v.sort_by_key(|&k| k);        // 按键排序（用于结构体排序）
v.sort_unstable();             // 更快，但不稳定（相等元素顺序可能变）

// 只找最大/最小，不排序
v.iter().max();  // Some(&5)
v.iter().min();  // Some(&1)
```

### 73. 类型转换的常用模式

```rust
// &str → String
"hello".to_string();
String::from("hello");
"hello".to_owned();

// String → &str
let s = String::from("hello");
let s_ref: &str = &s;

// 数字 → String
42.to_string();
format!("{}", 42);

// String → 数字
"42".parse::<i32>().unwrap();
"42".parse().unwrap_or(0i32);
```

### 74. 用 matches! 宏简化模式匹配

```rust
let x = Some(42);

// ❌ 啰嗦
if let Some(n) = x { if n > 10 { println!("yes"); } }

// ✅ matches! 宏
if matches!(x, Some(n) if n > 10) {
    println!("yes");
}

// 在 filter 里特别好用
let v = vec![Some(1), None, Some(20), Some(5)];
let big: Vec<_> = v.iter().filter(|x| matches!(x, Some(n) if *n > 10)).collect();
```

### 75. 用 dbg! 代替 println! 调试

```rust
let a = 2;
let b = dbg!(a * 2) + 1;  // 打印：[src/main.rs:2] a * 2 = 4，并返回值

// dbg! 返回值本身，可以插进表达式中间
let v: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|&x| dbg!(x * 10))  // 调试每一步
    .collect();
```

`println!` 只打印，`dbg!` 打印文件名、行号、表达式、值，调试完删掉即可。
