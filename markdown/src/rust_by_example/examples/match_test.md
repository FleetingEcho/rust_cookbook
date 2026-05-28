# 模式匹配

## 基本 match

```rust
pub fn test() {
    let number = 13;
    match number {
        1 => println!("一！"),
        2 | 3 | 5 | 7 | 11 => println!("这是个质数"),
        13..=19 => println!("一个青少年"),
        _ => println!("没什么特别的"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}
```

## 解构元组

```rust
fn test1() {
    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("第一个是 `0`,`y` 是 {:?},`z` 是 {:?}", y, z),
        (1, ..) => println!("第一个是 `1`,其余的不重要"),
        (.., 2) => println!("最后一个是 `2`,其余的不重要"),
        (3, .., 4) => println!("第一个是 `3`,最后一个是 `4`,其余的不重要"),
        _ => println!("它们是什么并不重要"),
    }
}
```

## 解构枚举

```rust
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}

fn test3() {
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("颜色是红色！"),
        Color::RGB(r, g, b) => println!("红：{}，绿：{}，蓝：{}！", r, g, b),
        _ => println!("其他颜色"),
    }
}
```

## 其他模式

- `if let` — 简洁的单分支匹配
- `while let` — 循环匹配
- `@` 绑定 — 将值绑定到变量同时匹配条件
- 模式守卫 — `=>` 前添加条件表达式
