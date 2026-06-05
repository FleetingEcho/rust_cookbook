# 🕵️ Rust 练习题：侦探推理系统

> **难度**：中级 | **涉及概念**：`enum`、`struct`、`trait`、`Display`、`Result`、状态机

---

## 题目背景

> trait 的本质是：定义类型之间共享的行为契约
>
> 不用 trait：每个类型都是孤岛，代码耦合度高，难以测试
>
> 用 trait：系统灵活可扩展，易于测试，符合开闭原则

实现一个迷你侦探推理游戏的核心逻辑（模拟状态即可，不需要真实 AI）。

---

## 数据结构要求

### 枚举 `CaseStatus`

| 变体 | 说明 |
|------|------|
| `Unopened` | 案件未开启 |
| `Investigating { location: String, days_elapsed: u32 }` | 调查中，记录当前地点和已过天数 |
| `Interrogating { suspect_name: String, tension: u8 }` | 审讯中，记录嫌疑人姓名和紧张度（0–100） |
| `Solved { culprit: String }` | 已破案，记录真凶姓名 |
| `Failed { reason: String }` | 调查失败，记录原因 |

---

### 枚举 `ClueType`

- `Physical` — 实物证据（凶器、指纹等）
- `Testimony` — 证词（目击者陈述）
- `Alibi` — 不在场证明（可能是伪造的）

**要求**：派生 `Debug`，并实现 `Display`（显示中文类型名）

---

### 枚举 `InterrogationResult`

- `Confession` — 认罪
- `Denial` — 否认
- `NewClue(String)` — 提供了新线索，附带线索描述
- `Silent` — 保持沉默

---

### 结构体 `Clue`

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `u32` | 线索编号 |
| `description` | `String` | 线索描述 |
| `clue_type` | `ClueType` | 线索类型 |
| `credibility` | `u8` | 可信度 0–100 |

**`Display` 输出格式**：

```
[#001][实物证据] 厨房发现一把沾血的刀 (可信度: 95%)
```

---

### 结构体 `Suspect`

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 姓名 |
| `motive` | `String` | 作案动机 |
| `is_culprit` | `bool` | 是否是真凶（不在 Display 中暴露） |
| `alibi` | `String` | 不在场证明 |
| `interrogated` | `bool` | 是否已被审讯过 |

**`Display` 输出格式**：

```
[嫌疑人] 张伟 | 动机: 财产纠纷 | 不在场证明: 称当晚在家睡觉
```

---

### 结构体 `Case`

| 字段 | 类型 | 说明 |
|------|------|------|
| `title` | `String` | 案件名称 |
| `description` | `String` | 案件描述 |
| `status` | `CaseStatus` | 当前案件状态 |
| `clues` | `Vec<Clue>` | 已收集的线索 |
| `suspects` | `Vec<Suspect>` | 嫌疑人列表 |
| `max_days` | `u32` | 最大调查天数，超过则自动失败 |

**`Display`** 需打印完整案件报告，包括：当前状态、已收集线索数量、所有嫌疑人列表。

---

## Trait 要求

### `Detective`

定义侦探行为，需为 `Case` 实现：

```rust
fn open_case(&mut self) -> Result<(), String>;
fn investigate(&mut self, location: &str) -> Result<Clue, String>;
fn interrogate(&mut self, suspect_name: &str) -> Result<InterrogationResult, String>;
fn accuse(&mut self, suspect_name: &str) -> Result<(), String>;
fn review_clues(&self) -> Vec<&Clue>;

// 默认实现：线索数 >= 3 且至少一条可信度 >= 80
fn has_enough_evidence(&self) -> bool;
```

---

## 状态流转规则

| 操作 | 允许的前置状态 | 非法时行为 |
|------|--------------|-----------|
| `open_case()` | `Unopened` | 返回 `Err`，给出中文错误信息 |
| `investigate()` | `Investigating` | 非调查状态返回 `Err` |
| `interrogate()` | `Investigating` 或 `Interrogating` | `Unopened` / `Solved` / `Failed` 返回 `Err` |
| `accuse()` | `has_enough_evidence()` 为 `true` | 证据不足返回 `Err` |

---

## 业务逻辑细节

- `investigate()` 每次调用让 `days_elapsed + 1`；超过 `max_days` 时，自动将状态改为 `Failed { reason: "超过最大调查天数".to_string() }`
- `interrogate()` 若该嫌疑人 `interrogated == true`（已审讯过），则 `tension + 20`，并可返回不同的 `InterrogationResult`
- `accuse()` 检查指控对象的 `is_culprit` 字段：`true` → 状态变为 `Solved`，`false` → 状态变为 `Failed`
- 所有非法操作必须返回 `Err(String)`，错误信息使用中文

---

## `main` 函数演示要求

1. **创建案件**："豪宅谋杀案"，至少 **3 名嫌疑人**，其中恰好 1 人是真凶
2. **遍历打印**所有嫌疑人信息
3. **正常破案流程**：
   - 开案 → 勘察至少 2 个地点 → 审讯嫌疑人 → 指控真凶 → 打印破案结果
4. **非法操作演示**：
   - 未开案时调用 `investigate()`，打印错误
   - 证据不足时调用 `accuse()`，打印错误
5. **`has_enough_evidence()`** 在收集线索前后各调用一次，打印对比结果
6. **超时失败演示**：创建第二个案件，`max_days` 设为 `2`，连续调用 `investigate()` 3 次，观察自动失败

---

## 提示

- `PlayState` 的思路同样适用于 `CaseStatus`，状态机是关键
- `match` + `&self.state` 处理借用时注意生命周期
- `interrogate()` 需要同时读取 `suspects` 和修改 `state`，注意借用冲突的处理方式
- `has_enough_evidence()` 是 trait 的**默认实现**，思考为什么要放在 trait 里而不是 `impl Case`

---

祝编码顺利，破案愉快！🔍