做出这道题
# 📦 Rust 练习题：快递仓库分拣系统

> **难度**：中级 | **涉及概念**：`enum`、`struct`、`trait`、`Display`、`Result`、状态机

---

## 题目背景

一个快递仓库每天接收大量包裹，需要对包裹进行**分拣、分配快递员、派送**，并追踪每个包裹的状态。

---

## 数据结构要求

### 枚举 `PackageStatus`

| 变体 | 说明 |
|------|------|
| `Received` | 已入库，等待分拣 |
| `Sorting { belt_id: u32 }` | 分拣中，记录所在传送带编号 |
| `Assigned { courier_name: String }` | 已分配快递员 |
| `InTransit { courier_name: String, eta_hours: u32 }` | 派送中，记录快递员和预计剩余小时数 |
| `Delivered` | 已签收 |
| `Failed { reason: String }` | 派送失败，记录原因 |

---

### 枚举 `PackageSize`

- `Small` — 小件（≤1kg）
- `Medium` — 中件（1–10kg）
- `Large` — 大件（>10kg）

**要求**：派生 `Debug`，实现 `Display`（显示中文名称）

---

### 枚举 `DeliveryResult`

- `Success` — 签收成功
- `NoOneHome` — 无人接收，转存代收点
- `AddressNotFound` — 地址有误
- `Refused` — 拒收

---

### 结构体 `Package`

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `u32` | 包裹编号 |
| `recipient` | `String` | 收件人姓名 |
| `address` | `String` | 收件地址 |
| `size` | `PackageSize` | 包裹尺寸 |
| `status` | `PackageStatus` | 当前状态 |
| `fragile` | `bool` | 是否易碎 |

**`Display` 输出格式**：

```
[#0042][中件] 张伟 | 地址: 北京市朝阳区xx路1号 | 状态: 派送中(李师傅, 预计2小时) | 易碎: 是
```

---

### 结构体 `Courier`

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 快递员姓名 |
| `max_capacity` | `u32` | 最多同时携带包裹数 |
| `current_load` | `u32` | 当前携带包裹数 |
| `accepts_large` | `bool` | 是否接受大件 |

**`Display` 输出格式**：

```
[快递员] 李师傅 | 负载: 3/5 | 接受大件: 否
```

---

### 结构体 `Warehouse`

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 仓库名称 |
| `packages` | `Vec<Package>` | 所有包裹 |
| `couriers` | `Vec<Courier>` | 所有快递员 |
| `belt_count` | `u32` | 传送带数量 |

**`Display`** 需打印完整仓库报告，包括：包裹总数、各状态数量统计、快递员列表。

---

## Trait 要求

### `Dispatch`

定义仓库调度行为，需为 `Warehouse` 实现：

```rust
fn receive(&mut self, package: Package) -> Result<(), String>;
fn sort(&mut self, package_id: u32) -> Result<(), String>;
fn assign(&mut self, package_id: u32, courier_name: &str) -> Result<(), String>;
fn dispatch(&mut self, package_id: u32, eta_hours: u32) -> Result<(), String>;
fn deliver(&mut self, package_id: u32, result: DeliveryResult) -> Result<(), String>;
fn query(&self, package_id: u32) -> Option<&Package>;

// 默认实现：返回所有状态为 Failed 的包裹
fn failed_packages(&self) -> Vec<&Package>;
```

---

## 状态流转规则

| 操作 | 允许的前置状态 | 非法时行为 |
|------|--------------|-----------|
| `receive()` | 无（新包裹） | 编号重复返回 `Err` |
| `sort()` | `Received` | 其他状态返回 `Err` |
| `assign()` | `Sorting` | 非分拣状态返回 `Err` |
| `dispatch()` | `Assigned` | 非已分配状态返回 `Err` |
| `deliver()` | `InTransit` | 非派送中返回 `Err` |

---

## 业务逻辑细节

- `sort()` 自动选择编号最小的空闲传送带（`belt_id` 从 1 开始，同时使用中的传送带数不超过 `belt_count`），若传送带全满则返回 `Err`
- `assign()` 检查快递员是否存在、是否有空余容量；若包裹是 `Large` 且快递员 `accepts_large == false`，返回 `Err`；成功后 `current_load + 1`
- `deliver()` 根据 `DeliveryResult` 决定最终状态：`Success` → `Delivered`，其余 → `Failed { reason }`；同时将对应快递员的 `current_load - 1`
- 所有非法操作必须返回 `Err(String)`，错误信息使用中文

---

## `main` 函数演示要求

1. **创建仓库**："北京东城配送中心"，`belt_count = 2`，至少 **2 名快递员**（其中一人不接受大件）
2. **入库 4 个包裹**，尺寸各异，其中至少 1 个易碎、1 个大件
3. **正常派送流程**：
   - 入库 → 分拣 → 分配快递员 → 派送 → 签收，打印每步结果
4. **非法操作演示**：
   - 对已分拣的包裹再次调用 `sort()`，打印错误
   - 将大件分配给不接受大件的快递员，打印错误
5. **传送带满载演示**：连续入库并分拣 3 个包裹（`belt_count = 2`），观察第 3 次分拣失败
6. **`failed_packages()`** 在所有操作结束后调用，打印所有失败包裹汇总