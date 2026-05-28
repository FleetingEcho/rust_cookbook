# 变量绑定

## 基础绑定

```rust
fn main() {
    let x = 1;
    let y: i32 = 2;
    println!("x = {}, y = {}", x, y);
}
```

## if let / while let

```rust
let mut optional = Some(0);

while let Some(i) = optional {
    if i > 9 {
        println!("大于 9,退出！");
        optional = None;
    } else {
        println!("`i` 是 `{:?}`。再试一次。", i);
        optional = Some(i + 1);
    }
}

if let Some(i) = Some(7) {
    println!("匹配到 {:?}！", i);
} else {
    println!("没有匹配到数字。");
}
```
