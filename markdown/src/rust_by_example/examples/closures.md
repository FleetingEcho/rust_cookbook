# 闭包

## 基础闭包

```rust
fn create_fn() -> impl Fn(i32) -> i32 {
    let x = 1;
    move |y| x + y
}

fn main() {
    let closure = create_fn();
    println!("{}", closure(2)); // 3
}
```

## 闭包与 Fn 特征

- `Fn` — 不可变借用捕获的变量
- `FnMut` — 可变借用捕获的变量
- `FnOnce` — 获取捕获变量的所有权
