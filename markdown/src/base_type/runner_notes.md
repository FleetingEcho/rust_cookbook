# Runner Notes

这是旧 `rust-learning` crate 的 main 入口笔记。现在 beginner iteration 示例已经合并到 `base_type::iteration`。

```rust
const MY_CONST_STR: &str = "thinking";
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut min_val = 5;
    min_val = 6;
    println!("The value is:{} {}", min_val, MY_CONST_STR);
    iteration::basic_iteration()
}

fn tuple_note() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```
