# 字符串、布尔、单元类型

## char 类型

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
}
```

`char` 占 4 字节，表示一个 Unicode 标量值。

## bool 类型

```rust
fn main2() {
    let t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }
}
```

## 宏示例

```rust
macro_rules! my_print {
    ($msg:expr) => {
        println!(">>> {}", $msg);
    };
}

fn main3() {
    my_print!("Hello Rust!"); // 输出 >>> Hello Rust!
}
```

`println!` 是**宏（macro）**，不是函数，所以需要 `!`。宏在编译期展开，支持可变参数。

## 📘 TypeScript 对比

| 特性 | Rust | TypeScript |
|------|------|-----------|
| char | `char`（4 字节 Unicode） | 无 char，是 `string` |
| bool | `bool` | `boolean` |
| unit | `()` | `void` / `undefined` |

> ⚠️ Rust 的 `char` 是 4 字节 Unicode 标量值（UTF-32 码点），TS 没有单独的字符类型，字符就是长度为 1 的 string。
