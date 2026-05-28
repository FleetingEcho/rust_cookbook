# 表达式

## 块表达式

```rust
pub fn test() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x 是 {:?}", x);
    println!("y 是 {:?}", y);
    println!("z 是 {:?}", z);
}
```

- 没有 `;` 的最后一个表达式是返回值
- 有 `;` 则返回 `()`
