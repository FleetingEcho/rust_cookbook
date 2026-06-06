# 📦 Rust 练习题：快递分拣派送

> **难度**：中级 | **涉及概念**：`enum`、`struct`、`trait`

---

## 题目背景

实现一个简化的快递分拣系统，追踪包裹从入库到签收的状态流转。

---

## 数据结构

### 枚举 `PackageStatus`

| 变体 | 说明 |
|------|------|
| `Received` | 已入库 |
| `InTransit { courier: String }` | 派送中，记录快递员姓名 |
| `Delivered` | 已签收 |
| `Failed { reason: String }` | 派送失败 |

### 结构体 `Package`

| 字段 | 类型 |
|------|------|
| `id` | `u32` |
| `recipient` | `String` |
| `status` | `PackageStatus` |

**`Display` 输出格式**：
```
[#001] 张伟 | 状态: 派送中(李师傅)
```

---

## Trait `Dispatch`

为 `Vec<Package>` 实现以下方法：

```rust
fn assign(&mut self, id: u32, courier: &str) -> Result<(), String>;
fn deliver(&mut self, id: u32) -> Result<(), String>;
fn fail(&mut self, id: u32, reason: &str) -> Result<(), String>;
```

**状态流转**：
- `assign()`：仅 `Received` → `InTransit`，否则返回 `Err`
- `deliver()`：仅 `InTransit` → `Delivered`，否则返回 `Err`
- `fail()`：`InTransit` → `Failed`，否则返回 `Err`

---

## `main` 函数演示

1. 创建 3 个包裹并打印
2. 正常流程：`assign` → `deliver` 一个包裹，打印结果
3. 让另一个包裹派送失败，打印结果
4. 非法操作：对已签收的包裹调用 `assign()`，打印错误信息