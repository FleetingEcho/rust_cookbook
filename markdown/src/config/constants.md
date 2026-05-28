# 常量配置

## 1. 定义常量

```rust
pub const MAX_POINTS: u32 = 100_000;
```

使用 `const` 关键字定义常量，需要在声明时指定类型并立即赋值。常量在整个编译期保持不变，可通过 `pub` 对外公开。

## 2. 常量命名约定

常量使用 `SCREAMING_SNAKE_CASE`（全大写加下划线）命名，如 `MAX_POINTS`。
