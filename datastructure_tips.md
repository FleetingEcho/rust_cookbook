# Rust 数据结构实用技巧 50 则
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

// 追加另一个 vec
let mut other = vec![8, 9];
v.append(&mut other);  // other 被清空！所有权转移了

// append 比 extend 快，因为它直接移动了内存块
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

// chunks_mut / windows 也有可变版本
```

### 10. Vec 和数组的相互转换

```rust
// 数组 → Vec
let arr: [i32; 3] = [1, 2, 3];
let v: Vec<i32> = arr.to_vec();
// 或 arr.into_iter().collect()，但 to_vec 更简洁

// Vec → 数组（如果长度匹配）
let v = vec![1, 2, 3];
let arr: [i32; 3] = v.try_into().unwrap();  // 长度不对会 Err
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
    *v = format!("new_{}", v);
}

// 方式二：remove 再 insert，返回旧值
let old = map.insert("x", "new_value");
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
let bmap: BTreeMap<_, _> = map.into_iter().collect();
for (k, v) in &bmap { println!("{k}: {v}"); }  // 总是 a, b, c
```

### 18. get 的常见组合

```rust
let map = HashMap::from([("x", 42)]);

// 取值，没有给默认
map.get("x").copied().unwrap_or(0);

// 取值，引用转值（引用太麻烦时用 copied）
map.get("x");                    // Option<&i32>
map.get("x").copied();           // Option<i32> — 如果 i32: Copy
map.get("x").cloned();           // Option<i32> — 如果 i32: Clone
```

### 19. 只想检查 key 是否存在

```rust
// 用 contains_key，别用 get
if map.contains_key("x") { ... }
// 比 get().is_some() 更语义化，而且不 borrow value
```

---

## 三、String 与 &str

### 20. 字符串拼接的各种姿势

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

### 21. 字符串切片小心！是字节索引不是字符索引

```rust
let s = "你好";

// 中文字符 1 个字 = 3 字节
println!("{}", s.len());          // 6（字节数）
println!("{}", s.chars().count()); // 2（字符数）

// ❌ 直接按字节索引会崩溃
// &s[0..1] → panic!（切到了字符中间）

// ✅ 安全方式：按字符取
for ch in s.chars() { ... }
println!("{}", s.chars().nth(1).unwrap());  // '好'

// ✅ 取前 N 个字符
let first_n: String = s.chars().take(1).collect();  // "你"
```

### 22. 字符串反转

```rust
let s = "hello";
let rev: String = s.chars().rev().collect();  // "olleh"

// ❌ 不能用 s.bytes().rev() — 那会反转字节，中文就坏了
```

### 23. trim 的各种变体

```rust
let s = "  hello world  ";

s.trim();              // "hello world"           首尾
s.trim_start();        // "hello world  "         左边
s.trim_end();          // "  hello world"         右边
s.trim_matches('h');  // "  hello world  "        匹配字符

// 去掉特定字符
"---hello---".trim_matches('-');  // "hello"
```

### 24. split 的多种模式

```rust
"a,b,c".split(',');            // ["a", "b", "c"]
"a,b,c".splitn(2, ',');        // ["a", "b,c"] 只分割前 n 次
"a,b,c".rsplit(',');           // ["c", "b", "a"] 从右往左
"a1b2c3".split(|c: char| c.is_ascii_digit()); // ["a", "b", "c"]

// 分割后别忘了 collect
let parts: Vec<&str> = "a,b,c".split(',').collect();
```

### 25. contains / starts_with / ends_with

```rust
"hello world".contains("world");  // true
"hello".starts_with("he");        // true
"hello".ends_with("lo");          // true

// 注意是字节搜索，不是正则
```

### 26. 查找与替换

```rust
"hello".find('e');               // Some(1)
"hello".rfind('l');              // Some(3) — 从右找
"hello".replace("l", "L");       // "heLLo" — 全部替换
"hello".replacen("l", "L", 1);  // "heLlo" — 只替换第一个
"hello".contains('x');           // false
```

### 27. 判空与修剪

```rust
"".is_empty();              // true
"  ".is_empty();            // false（有空格）
"  ".trim().is_empty();     // true
```

### 28. 数字转字符串的字符串格式控制

```rust
let n = 255;

format!("{n}");              // "255"
format!("{n:#x}");           // "0xff"  带 0x 前缀的十六进制
format!("{n:x}");            // "ff"    小写十六进制
format!("{n:X}");            // "FF"    大写十六进制
format!("{n:o}");            // "377"   八进制
format!("{n:b}");            // "11111111" 二进制
format!("{n:#010b}");        // "0b11111111" 带前缀，10位宽

let pi = 3.1415926;
format!("{pi:.2}");          // "3.14"
format!("{pi:>8.2}");        // "    3.14" 右对齐，总宽8
format!("{pi:<8.2}");        // "3.14    " 左对齐
format!("{pi:^8.2}");        // "  3.14  " 居中对齐
```

### 29. String 和 &str 作为函数参数

```rust
// ✅ 推荐：参数用 &str，最通用
fn process(s: &str) { ... }

process("hello");              // &str 直接传
process(&String::from("hi"));  // &String 自动 deref 成 &str

// ❌ 参数用 &String 会限制调用方
fn process(s: &String) { ... }

process(&String::from("hi")); // ✅
// process("hello");            // ❌ &str 不能直接转 &String
```

---

## 四、数字操作

### 30. 数字字面量加分隔符

```rust
let n = 1_000_000;       // 100 万，_ 是视觉分隔符
let m = 0.000_001;       // 理解成 0.000001
let hex = 0xFF_FF;       // 十六进制也可以用
```

### 31. 类型后缀

```rust
let n = 42;              // 默认 i32
let n = 42_i64;          // 指定 i64
let n = 42_u8;           // u8
let f = 3.14;            // 默认 f64
let f = 3.14_f32;        // f32

// 什么时候必须加？编译器无法推断时
vec![1, 2, 3].iter().sum::<i32>();  // turbofish
let v: Vec<i64> = vec![1, 2, 3].iter().map(|&x| x as i64).collect();
```

### 32. 安全算术三兄弟

```rust
let a: u32 = 100;
let b: u32 = 200;

// 普通加法：debug 模式溢出 panic，release 模式回绕
a + b;

// wrapping：overflow 时回绕（不会 panic）
a.wrapping_add(b);

// saturating：overflow 时停在最大值/最小值
a.saturating_add(b);  // u32::MAX

// checked：返回 Option，None 表示溢出了
a.checked_add(b);     // None
```

### 33. as 转换 vs TryFrom

```rust
// as：无条件转换（可能截断）
let x: i32 = 255;
let y: u8 = x as u8;      // 255，安全
let z: u8 = 256 as u8;    // 0！截断了，编译器不会告诉你

// TryFrom：安全转换，可能失败
let w = u8::try_from(256);  // Err("out of range")
```

`as` 是"我就要这么转，截断我也认"；`TryFrom` 是"转不了就报错"。

### 34. 字符串解析成数字

```rust
"42".parse::<i32>();         // Ok(42)
"42".parse::<i32>().unwrap_or(0);
"abc".parse::<i32>();        // Err(ParseIntError)

// 带前缀解析
i32::from_str_radix("ff", 16);  // Ok(255)，十六进制
```

### 35. 绝对值 / 幂 / 平方根

```rust
(-5).abs();         // 5
2_i32.pow(10);      // 1024
4_f64.sqrt();       // 2.0
4_f64.cbrt();       // 1.587...
```

i32 没有 sqrt 方法，需要转 f64。

### 36. 四舍五入

```rust
let f = 3.14159;
f.round();        // 3.0
f.floor();        // 3.0
f.ceil();         // 4.0
f.trunc();        // 3.0（截断小数部分）
(f * 100.0).round() / 100.0;  // 保留 2 位小数

// 取整后转 i32
f as i32;         // 3（直接截断，不是四舍五入）
f.round() as i32; // 3（先四舍五入再转）
```

### 37. 数字比较的坑

```rust
// f64 不能直接 == 比较（浮点精度问题）
let a = 0.1 + 0.2;
let b = 0.3;
assert!(a != b);  // 真的！浮点精度坑

// ✅ 应该用差值比较
(a - b).abs() < 1e-10;

// 排序时 NaN 的问题
let v = vec![1.0, f64::NAN, 3.0];
// v.sort();  // ❌ panics! NaN 无法比较
v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
```

---

## 五、数组与切片

### 38. 数组初始化

```rust
// 相同值填充
let arr = [0; 10];      // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// 逐字列出
let arr = [1, 2, 3, 4, 5];

// 多维数组
let matrix = [[0; 3]; 3];  // 3x3 的零矩阵
```

### 39. 切片取部分

```rust
let v = vec![1, 2, 3, 4, 5];

&v[1..3];           // [2, 3]   下标 1 到 2
&v[..3];            // [1, 2, 3] 从头到下标 2
&v[2..];            // [3, 4, 5] 从下标 2 到尾
&v[..];             // 整条切片

// 注意：切片索引越界时会 panic
// 安全取前 N 个
v.get(0..3);        // Some(&[1, 2, 3]) — 不会 panic
v.get(0..10);       // None — 不会 panic！
```

### 40. split_at：从中间安全分割

```rust
let v = [1, 2, 3, 4, 5];
let (left, right) = v.split_at(3);
// left = [1, 2, 3], right = [4, 5]

// 不会 panic 的版本：split_at_checked（Rust 1.80+）
```

### 41. 二分查找

```rust
let v = vec![1, 3, 5, 7, 9];

v.binary_search(&5);      // Ok(2)  — 找到了，返回下标
v.binary_search(&4);      // Err(2) — 没找到，返回该插入的位置

// 注意：必须有序！否则结果不对
```

### 42. 首尾元素

```rust
let v = vec![1, 2, 3];

v.first();       // Some(&1)
v.last();        // Some(&3)

v.first_mut();   // Some(&mut 1)
v.last_mut();    // Some(&mut 3)
```

### 43. contains 检查是否存在

```rust
let v = vec![1, 2, 3];
v.contains(&2);  // true — 注意是引用！

let arr = [1, 2, 3];
arr.contains(&2);  // 数组也有 contains（Rust 1.47+）
```

---

## 六、Option 与 Result 进阶

### 44. Option 的 map / and_then / or_else 链条

```rust
// map：Some 时变换，None 保持 None
Some(1).map(|x| x + 1);           // Some(2)
None.map(|x: i32| x + 1);         // None

// and_then：返回 Option 的变换（flatten 版 map）
Some(1).and_then(|x| if x > 0 { Some(x + 1) } else { None });
// 和 map 的区别：map 的闭包返回 T，and_then 返回 Option<T>

// or_else：None 时提供备选
None.or_else(|| Some(42));         // Some(42)
Some(1).or_else(|| Some(42));      // Some(1) — or_else 不会执行
```

### 45. Result 的 map / map_err / and_then

```rust
// map：Ok 时变换
Ok(1).map(|x| x + 1);                     // Ok(2)

// map_err：Err 时变换错误类型
Err("error").map_err(|e| format!("{e}")); // Err(String)

// and_then：返回 Result 的变换
Ok(1).and_then(|x| Ok(x + 1));

// 实际场景：解析字符串，出错返回默认值
"42".parse::<i32>()
    .map(|n| n * 2)          // 成功就加倍
    .unwrap_or(0);            // 失败给 0

// 或者区分错误
"42".parse::<i32>()
    .map_err(|e| format!("Parse error: {e}"));
```

### 46. Option 和 Result 互转实战

```rust
// 需求：从字符串数组中解析数字，跳过无效的
let inputs = vec!["1", "abc", "2", "xyz"];

// 用 filter_map：过滤掉 None
let nums: Vec<i32> = inputs.iter()
    .filter_map(|s| s.parse::<i32>().ok())  // Result → Option, 丢掉 Err
    .collect();  // [1, 2]

// 用 flatten 也可以
let nums: Vec<i32> = inputs.iter()
    .map(|s| s.parse::<i32>().ok())
    .flatten()
    .collect();
```

### 47. ok_or / ok_or_else：Option → Result

```rust
// 场景：从配置中取值，没有就报错
let config: Option<&str> = None;

// Option → Result，带错误信息
config.ok_or("config missing");         // Err("config missing")
config.ok_or_else(|| format!("config {} missing", "db")); // 懒求值

// 搭配 ? 使用
fn get_config() -> Result<&'static str, String> {
    let val = Some("value");
    val.ok_or_else(|| "config missing".to_string())
}
```

### 48. transpose：Option<Result<T>> ↔ Result<Option<T>>

```rust
// 场景：从 HashMap 取值，key 可能不存在（None），值也可能解析失败（Err）
let val: Option<Result<i32, _>> = Some("42".parse());

// 转成 Result<Option<i32>, _> — 外层错误优先
let val: Result<Option<i32>, _> = val.transpose();
// Ok(Some(42))

// 这样可以用 ? 同时处理"没有值"和"解析失败"
fn parse_val() -> Result<i32, ParseIntError> {
    let raw: Option<&str> = Some("42");
    raw.map(|s| s.parse::<i32>()).transpose()?.unwrap_or(0)
    //                            ^ transpose 把 Option<Result> 翻转为 Result<Option>
    //                      ^ ? 取出 Option，错误直接 return Err
}
```

---

## 七、迭代器技巧

### 49. filter_map：filter + map 合一

```rust
let v = vec!["1", "abc", "2", "xyz"];

// filter + map 两遍
let nums: Vec<i32> = v.iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();  // [1, 2]

// 等价写法：filter 后再 map
let nums: Vec<i32> = v.iter()
    .map(|s| s.parse::<i32>())
    .filter(|r| r.is_ok())
    .map(|r| r.unwrap())
    .collect();  // [1, 2] — 但 filter_map 更简洁
```

### 50. flatten：展平嵌套

```rust
let nested = vec![
    vec![1, 2],
    vec![3, 4, 5],
    vec![],
];

nested.iter().flatten().collect::<Vec<_>>();  // [1, 2, 3, 4, 5]

// 和 flat_map 等价：
nested.iter().flat_map(|v| v.iter()).collect::<Vec<_>>();

// flatten 用于 Option 也很方便：
let v = vec![Some(1), None, Some(2)];
v.into_iter().flatten().collect::<Vec<_>>();  // [1, 2]
```

### 51. partition：一次遍历分成两组

```rust
let v = vec![1, 2, 3, 4, 5];

let (even, odd): (Vec<i32>, Vec<i32>) = v.into_iter()
    .partition(|x| x % 2 == 0);
// even = [2, 4], odd = [1, 3, 5]

// 比 filter 两次快（只遍历一次）
```

### 52. group_by：相邻分组

```rust
use std::iter::Iterator;  // 需要 nightly 或 itertools

let data = vec![1, 1, 1, 2, 2, 3, 1, 1];
// group_by 把连续相同的分在一组
```

稳定版用 `itertools` crate 的 `group_by` 或 `coalesce`。

### 53. scan：带状态扫描

```rust
let v = vec![1, 2, 3, 4, 5];

// 计算累积和
let sums: Vec<i32> = v.iter()
    .scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    })
    .collect();  // [1, 3, 6, 10, 15]
```

### 54. Iterator 的 inspect：调试链式调用

```rust
let result: Vec<i32> = vec![1, 2, 3]
    .into_iter()
    .inspect(|x| println!("before map: {x}"))
    .map(|x| x * 10)
    .inspect(|x| println!("after map: {x}"))
    .collect();
// 打印：
// before map: 1
// after map: 10
// before map: 2
// ...
```

### 55. 迭代器的短路操作

```rust
let v = vec![1, 2, 3, 4, 5];

v.iter().any(|x| x > 3);   // true — 到 4 就停，不继续
v.iter().all(|x| x > 0);   // true
v.iter().find(|&&x| x > 3); // Some(&4) — 找到就停
```

`any` / `all` / `find` / `position` 都是短路求值的，对于大集合能省很多时间。

---

## 八、实用小技巧

### 56. 默认值的各种写法

```rust
let x: Option<i32> = None;

x.unwrap_or(0);              // 直接给默认值
x.unwrap_or_default();       // 类型默认值（0 / "" / false / empty vec）
x.unwrap_or_else(|| { ... }) // 闭包懒求值
x.or(Some(0));               // Option 本身，x 是 None 就给 Some(0)
x.or_else(|| Some(0));       // 闭包版
```

### 57. swap 交换两个变量

```rust
let mut a = 1;
let mut b = 2;
std::mem::swap(&mut a, &mut b);  // a=2, b=1

// 或者用 swap 方法（某些类型有）
let mut v = vec![1, 2, 3];
v.swap(0, 2);  // [3, 2, 1]
```

### 58. take / replace 替换值

```rust
let mut x = String::from("hello");

// take：把值换成默认值（"" 对于 String），返回原值
let old = std::mem::take(&mut x);  // old = "hello", x = ""

// replace：把值换成新值，返回原值
let old = std::mem::replace(&mut x, String::from("world"));
// old = ""（上一行的空字符串）, x = "world"
```

### 59. 排序和比较

```rust
let mut v = vec![3, 1, 4, 1, 5];

v.sort();               // [1, 1, 3, 4, 5] — 自然排序
v.sort_by(|a, b| b.cmp(a)); // [5, 4, 3, 1, 1] — 逆序
v.sort_by_key(|k| *k); // 按键排序（用于结构体）

// 不修改原数组的排序
let sorted: Vec<_> = v.iter().copied().sorted().collect();  // 需要 itertools
// 或用：
let mut sorted = v.clone();
sorted.sort();
```

### 60. 类型转换的常用模式

```rust
// &str → String
"hello".to_string();
String::from("hello");
"hello".to_owned();         // to_owned() 更语义化

// String → &str
let s = String::from("hello");
let s: &str = &s;          // deref 转换

// 数字 → String
42.to_string();
format!("{}", 42);

// String → 数字
"42".parse::<i32>().unwrap();
"42".parse().unwrap_or(0);  // 有时能自动推导类型
```
