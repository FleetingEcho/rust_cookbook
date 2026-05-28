# Utils 工具模块

## 1. 子模块声明

```rust
pub mod helper;
pub mod math;
pub mod string;
```

`utils` 模块导出三个子模块：

- `helper` — 辅助函数，如 `print_max_points`。
- `math` — 数学运算，如 `add`。
- `string` — 字符串操作，如 `to_uppercase`。

## 2. 重导出

```rust
pub use math::add;
pub use string::to_uppercase;
```

通过 `pub use` 将 `add` 和 `to_uppercase` 重导出，调用方可以直接使用 `utils::add` 和 `utils::to_uppercase`。

## 3. 混合颜色函数

```rust
use crate::kinds::*;

/// 混合两个基础颜色，返回一个二级颜色。
/// 这里目前是学习模块示例，后续可以把完整颜色匹配逻辑补齐。
///
/// 示例：
/// ```rust
/// use learning_notes::kinds::{PrimaryColor, SecondaryColor};
/// use learning_notes::utils::mix;
///
/// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
/// ```
pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
    SecondaryColor::Green
}
```

`mix` 函数演示了如何跨模块引用 `kinds` 模块中的枚举类型。当前实现返回固定的 `SecondaryColor::Green`，作为学习模块系统的示例。
