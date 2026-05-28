# Crate 与模块可见性

## 1. Crate 结构

`crate` 是 Rust 编译的基本单元，可以是一个二进制程序或一个库。每个 crate 包含一个模块树，通过 `use` 和 `pub` 控制可见性。

## 2. use 引入路径

```rust
use std::collections::HashMap;
```

`use` 关键字将路径引入当前作用域，简化代码调用。

## 3. 嵌套模块

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

模块可以嵌套，形成层级结构，对应文件系统目录。

## 4. 路径规则

- **绝对路径**：从 crate 根开始，使用 `crate::` 前缀。
- **相对路径**：从当前模块开始，使用 `self::`、`super::` 或直接子模块名。

## 5. pub 关键字

使用 `pub` 使模块、函数、结构体等对外可见：

```rust
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
```

## 6. pub use 重导出

`pub use` 可以将路径重新导出为公共 API：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## 7. 外部 Crate

在 `Cargo.toml` 中引入外部依赖：

```toml
[dependencies]
rand = "0.8.5"
```

然后在代码中使用：

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

## 8. 自定义结构体

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer() -> Breakfast {
            Breakfast {
                toast: String::from("Rye"),
                seasonal_fruit: String::from("Blueberries"),
            }
        }
    }
}

fn main() {
    let meal = back_of_house::Breakfast::summer();
    println!("I'd like {} please", meal.toast);
}
```

当结构体是 `pub` 时，其字段仍然默认为私有，需单独标注 `pub`。

## 9. 枚举默认公开

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let order = back_of_house::Appetizer::Soup;
}
```

与结构体不同，枚举的变体默认跟随枚举本身的可见性。

## 10. 可见性修饰符

```rust
mod a {
    const I: i32 = 3;

    fn private_function(i: i32) -> i32 {
        i + I
    }

    // a 模块外部可见
    pub fn bar(z: i32) -> i32 {
        private_function(I) * z
    }

    pub fn foo(y: i32) -> i32 {
        private_function(I) + y
    }

    mod b {
        // c 仅 a 内部可见
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}

fn main() {
    println!("bar: {}", a::bar(2)); // 3 + 4 = 7, 7 * 2 = 14
    println!("foo: {}", a::foo(2)); // 3 + 4 = 7, 7 + 2 = 9
}
```

### 可见性修饰符对照表

| 修饰符 | 说明 |
|--------|------|
| `pub` | 无限制，任何地方都可以访问 |
| `pub(crate)` | 限制在整个 crate（包）内可见 |
| `pub(self)` | 仅当前模块可见 |
| `pub(super)` | 仅父模块可见 |
| `pub(in path)` | 限制可见性到 `path` 指定的模块内 |

### 关键点

- `pub(in crate::a)` 使 `b::c::J` 只能在 `a` 模块内访问，避免了 `main` 或其他模块直接调用它。
- `private_function` 作为 `a` 内部的私有函数，确保 `bar` 和 `foo` 的内部逻辑对外部不可见，仅提供 `bar` 和 `foo` 作为接口。
- `pub(super)` 可以使子模块的函数对父模块可见，但不对更外层可见。
- `pub(in path)` 允许更精准地控制可见性范围。

## 11. 模块可见性示例

```rust
mod my_mod {
    // 私有函数，仅 my_mod 内可见
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // pub 允许外部访问
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 允许内部调用私有函数
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 嵌套模块
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        // 仅 my_mod 内可见
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_function_in_my_mod()`");
        }

        // 仅 nested 内可见
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // 仅父模块 my_mod 内可见
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_functions() {
        print!("called `my_mod::call_functions()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // 限制在 crate 内 (pub(crate)) 仅在当前 crate 内可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }
}

fn main() {
    my_mod::function();
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_functions();
    my_mod::public_function_in_crate();
}
```

---

## 📘 TypeScript 对比

Rust 模块系统 ≈ TS 的 ES modules。

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 导出 | `pub fn` / `pub struct` | `export function` / `export class` |
| 导入 | `use crate::module::Item` | `import { Item } from './module'` |
| 默认可见性 | 私有（`pub` 公开） | 导出才可见 |
| 文件映射 | 模块树 = 文件树 | import 路径 |
| 包管理器 | Cargo + crates.io | npm/yarn + registry |
| 重导出 | `pub use` | `export * from` / `re-export` |

> ⚠️ 关键差异：
>
> - Rust 的模块默认**私有**，显式 `pub` 才公开。
> - TS 的模块默认**不导出**，显式 `export` 才公开。
> - Rust 模块树对应文件系统结构（`mod.rs` 或同名目录）。
> - TS 用文件夹路径 + `index.ts` 组织。

详细对照 → [rust_vs_typescript.rs §23 "模块与包管理"](../rust_vs_typescript.rs)
