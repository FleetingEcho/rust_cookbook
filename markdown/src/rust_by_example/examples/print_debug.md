# Debug 格式化

## {:?} 占位符

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn test() {
    let person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:?}", person);
}
```

- `{:?}` — Debug 格式化
- `#` — 美化输出：`{:#?}`
