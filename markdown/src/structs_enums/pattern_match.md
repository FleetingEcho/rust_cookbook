# Rust 模式匹配实战

## 1. Vec 与 while let

`Vec` 是 Rust 的动态数组。使用 `while let` 可以在条件不满足时自动退出循环：

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## 2. 遍历与枚举

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

## 3. if let 与 let-else

### 3.1 if let

```rust
let some_option_value: Option<i32> = Some(42);
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

### 3.2 let-else

`let-else` 在模式不匹配时执行 `else` 分支（通常 `return` 或 `panic`）：

```rust
let Some(y) = some_option_value else {
    return;
};
println!("{}", y);
```

## 4. 解析字符串示例

解析 `"3 chairs"` 这种格式，返回 (数量, 物品)：

```rust
use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("无法解析计数项对: '{s}'");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("无法解析整数: '{count_str}'");
    };

    (count, item)
}

assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
```

## 5. Result 枚举

`Result<T, E>` 表示成功或失败：

```rust
enum Result<T, E> {
    Ok(T),  // 表示成功，包含成功的值
    Err(E), // 表示失败，包含错误信息
}

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("The divisor cannot be zero"))
    } else {
        Ok(x / y)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("结果: {}", result),
    Err(error) => println!("错误: {}", error),
}

match divide(10.0, 0.0) {
    Ok(result) => println!("结果: {}", result),
    Err(error) => println!("错误: {}", error),
}
```

### 5.1 unwrap 注意

直接获取结果：

```rust
let result = divide(10.0, 2.0);
println!("直接获取结果: {}", result.unwrap()); // 5.0
```

注意：如果 `result` 是 `Err`，`unwrap()` 会触发 `panic!`，导致程序崩溃。

### 5.2 使用 if let

```rust
if let Ok(value) = divide(10.0, 2.0) {
    println!("运算成功，值为 {}", value);
}
```

### 5.3 使用 `?` 运算符

```rust
fn divide_and_print(x: f64, y: f64) -> Result<(), String> {
    let result = divide(x, y)?; // 如果失败，直接返回 Err
    println!("Calculation result: {}", result);
    Ok(())
}

let _ = divide_and_print(10.0, 2.0);
let _ = divide_and_print(10.0, 0.0);
```

### 5.4 使用 map

```rust
let result = divide(10.0, 2.0);
result.map(|val| println!("成功的值: {}", val));
```

如果 `result` 是 `Ok(val)`，则 `map()` 执行闭包，否则什么都不做。

---

## TypeScript 对比

模式匹配 = 解构 + switch + 条件守卫 三合一

| 模式 | Rust 示例 | TS 对应 |
|------|-----------|---------|
| 解构枚举 | `Message::Move{x,y} =>` | `switch(msg.kind) + 解构` |
| 解构结构体 | `User { name, age } =>` | 对象解构 `const { name, age } = user` |
| 解构元组 | `(a, b, ..) =>` | 数组解构 `const [a, b] = arr` |
| 范围匹配 | `1..=5 =>` | `if (x >= 1 && x <= 5)` |
| 匹配守卫 | `n if n > 0 =>` | `if (n > 0)` |

详细对照 → [rust_vs_typescript.rs §5](../rust_vs_typescript.rs) "枚举与模式匹配"
