# 属性

属性是应用于模块、crate 或条目的元数据，可用于以下目的：

- 代码的条件编译
- 设置 crate 的名称、版本和类型
- 禁用代码检查（警告）
- 启用编译器特性
- 链接外部库
- 将函数标记为单元测试或基准测试
- 类属性宏

## 属性语法

- `#[outer_attribute]` — 应用于紧随其后的条目
- `#![inner_attribute]` — 应用于包含它的条目（通常是模块或 crate）

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

```rust
#![allow(unused_variables)]

fn main() {
    let x = 3; // 这通常会警告未使用的变量
}
```

属性可以使用不同的语法接受参数：

```rust
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

## 示例：dead_code lint

```rust
fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ 添加一个属性来抑制警告

fn main() {
    used_function();
}
```

## cfg 条件编译

```rust
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("你正在运行 Linux！");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("你**不是**在运行 Linux！");
}

fn main() {
    are_you_on_linux();
    println!("你确定吗？");
    if cfg!(target_os = "linux") {
        println!("是的，这绝对是 Linux！");
    } else {
        println!("是的，这绝对**不是** Linux！");
    }
}
```

- `#[cfg(...)]` 启用条件编译
- `cfg!(...)` 在运行时条件性地求值为 `true` 或 `false`
