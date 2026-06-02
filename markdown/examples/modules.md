# Rust vs TypeScript：模块系统

> **运行命令**：`cargo run -p learning_notes --example rts_modules`

---

## TypeScript 参考版本

```ts
// 导出（export）
export const PI = 3.14;
export function greet(name: string): string { return `Hi, ${name}`; }
export class User { ... }
export default class MainClass { ... }  // 默认导出
export type { User };                   // 只导出类型

// 导入（import）
import { greet, PI } from "./utils";
import * as utils from "./utils";       // 命名空间导入
import User from "./user";             // 默认导入
import type { User } from "./types";   // 只导入类型

// 重导出（re-export）
export { greet } from "./utils";
export * from "./utils";

// 动态导入
const mod = await import("./module");
```

**Rust 的模块系统与 TS 的文件模块有根本区别**：
- TS: 每个文件是一个模块，import/export 声明依赖关系
- Rust: 模块树由 `mod` 关键字显式构建，文件只是实现载体

---

## 一、内联模块（mod 关键字）

**TS** 没有内联模块，每个文件就是一个模块。

**Rust** 可以在单个文件内用 `mod {}` 定义子模块。

```rust
mod math {
    // 默认所有内容是私有的（TS 默认也是私有，但需要 export 才能跨文件访问）
    const E: f64 = 2.71828;

    // pub 使其对外可见（TS: export）
    pub const PI: f64 = 3.14159;

    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    // 私有函数，只能在 math 模块内使用（TS: 不导出）
    fn internal_helper() -> &'static str {
        "内部辅助函数"
    }

    // 公开函数可以调用私有函数
    pub fn describe() -> String {
        format!("PI={PI}, E={E}, helper={}", internal_helper())
    }

    // 嵌套模块（TS: 命名空间 namespace，或嵌套文件夹）
    pub mod geometry {
        // 子模块可以访问父模块的私有内容（通过 super::）
        pub fn circle_area(r: f64) -> f64 {
            super::PI * r * r   // super:: 访问父模块（TS: 没有对应，通常直接访问）
        }

        pub fn rectangle_area(w: f64, h: f64) -> f64 {
            super::multiply(w, h)
        }
    }
}
```

---

## 二、可见性修饰符

**TS**: `public`（默认）、`private`、`protected`（类成员）

**Rust**: `pub`（公开）、`pub(crate)`、`pub(super)`、无修饰（私有）

```rust
mod visibility_demo {
    pub struct PublicStruct {
        pub public_field: i32,      // 任何地方都可访问
        pub(crate) crate_field: i32, // 只在当前 crate 内可访问（TS 没有对应）
        super_field: i32,           // 只有父模块可访问（TS 类似 protected）
        private_field: i32,         // 只在本模块内（TS: private）
    }

    impl PublicStruct {
        pub fn new(val: i32) -> Self {
            PublicStruct {
                public_field: val,
                crate_field: val * 2,
                super_field: val * 3,
                private_field: val * 4,
            }
        }

        pub fn get_private(&self) -> i32 {
            self.private_field   // 通过公开方法暴露私有字段
        }
    }
}
```

---

## 三、use 语句（类似 TS 的 import { ... }）

```rust
// 引入路径，避免每次写全名
use math::geometry;     // TS: import { geometry } from "./math"
use std::collections::HashMap;  // TS: import { HashMap } from "std/collections"

// 重命名（as）
use math::PI as MATH_PI;  // TS: import { PI as MATH_PI } from "./math"

// 引入多个（用 {} 组合）
use std::fmt::{self, Display, Formatter};  // TS: import { Display, Formatter } from "std/fmt"
```

---

## 四、结构体的字段可见性

**TS**: `class` 字段用 `private`/`public` 修饰

```rust
pub struct User {
    pub name: String,      // 公开字段
    pub age: u32,
    password: String,      // 私有字段（TS: private password: string）
}

impl User {
    pub fn new(name: &str, age: u32, password: &str) -> Self {
        User {
            name: name.to_string(),
            age,
            password: password.to_string(),
        }
    }

    pub fn verify_password(&self, input: &str) -> bool {
        self.password == input
    }
}
```

---

## 五、pub use（重导出）

**TS**: `export { greet } from "./utils"` 或 `export * from "./utils"`

```rust
mod shapes {
    pub struct Circle { pub radius: f64 }
    pub struct Square { pub side: f64 }
}

// 重导出，让外部可以直接 use crate::Circle
// TS: export { Circle, Square } from "./shapes"
pub use shapes::Circle;
pub use shapes::Square;
```

---

## 六、Display trait（类似 TS 的 toString()）

```rust
use std::fmt::{self, Display, Formatter};

struct Point { x: f64, y: f64 }

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}
```

---

## 七、模块系统与文件系统的关系

```
// TS 文件结构：
//   src/
//     math.ts         → import { add } from "./math"
//     utils/
//       string.ts     → import { trim } from "./utils/string"

// Rust 文件结构：
//   src/
//     lib.rs          → mod math;       // 声明模块
//                       mod utils;      // 声明模块
//     math.rs         → pub fn add() {} // 实现
//     utils/
//       mod.rs        → pub mod string; // 声明子模块
//       string.rs     → pub fn trim() {} // 实现

// 或者 Rust 2018+ 风格：
//     utils.rs        → pub mod string; // 代替 utils/mod.rs

// 关键区别：
// TS: 文件路径 = 模块路径，import 引用文件
// Rust: mod 声明构建模块树，文件只是实现载体
//       必须在父模块中用 mod xxx; 声明，子模块才存在
```

---

## 核心区别

| 特性 | TypeScript | Rust |
|---|---|---|
| 模块定义 | 每个文件自动是模块 | `mod` 关键字显式构建模块树 |
| 导出 | `export default` / `named export` | `pub fn` / `pub struct` / `pub use` |
| 导入 | `import { a, b } from './x'` | `use crate::x::{a, b}` 或 `mod x; use x::{a,b}` |
| 重导出 | `export { greet } from "./utils"` | `pub use shapes::Circle;` |
