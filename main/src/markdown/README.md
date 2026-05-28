# 📚 markdown/ 文档索引

> Rust ↔ TypeScript 对照学习文档集合。点击文件名跳转。

---

## 一、文档速览

| 文件 | 描述 | 使用频率 | 掌握程度 |
|------|------|----------|----------|
| [`rust_vs_typescript.md`](./rust_vs_typescript.md) | 27 个主题全景对照，从 TS 视角看 Rust | `████████░░░░░░░░░░░░` 随时查阅 | 知道有什么、在哪查即可 |
| [`lifetimes_from_ts_basics.md`](./lifetimes_from_ts_basics.md) | 生命周期核心：为什么需要、三大场景、消除规则、编译错误解读、实战 | `████████████████████` **每天用** | **必须掌握** |
| [`lifetimes_advanced.md`](./lifetimes_advanced.md) | 生命周期进阶：10 个 TS 无法对应的概念 | `████░░░░░░░░░░░░░░░░` 偶尔查 | 理解概念，遇到回来翻 |

---

## 二、`lifetimes_advanced.md` 各章节频率速览

进阶篇内每个主题的实际使用频率：

| 章节 | 使用频率 |
|------|----------|
| [Variance（协变/逆变/不变）](./lifetimes_advanced.md#1-variance协变逆变不变) | `██░░░░░░░░░░░░░░░░░░` 极少，除非你在写集合库 |
| [NLL 深入](./lifetimes_advanced.md#2-nll-深入借用作用域分析) | `██████░░░░░░░░░░░░░░` 偶尔，调试借用冲突时 |
| [生命周期约束 `'a: 'b`](./lifetimes_advanced.md#3-生命周期约束-a-b-深入) | `████░░░░░░░░░░░░░░░░` 偶尔，嵌套结构体时 |
| [HRTB（高阶 trait bound）](./lifetimes_advanced.md#4-hrtb高阶-trait-bound) | `████░░░░░░░░░░░░░░░░` 偶尔，闭包参数复杂时 |
| [trait 对象 + 生命周期](./lifetimes_advanced.md#5-生命周期--trait-对象) | `██████░░░░░░░░░░░░░░` 偶尔，tokio::spawn + Box\<dyn Trait\> |
| [async + 生命周期](./lifetimes_advanced.md#6-生命周期--async) | `██████░░░░░░░░░░░░░░` 偶尔，写 async 代码时 |
| [closure + 生命周期](./lifetimes_advanced.md#7-生命周期--closure) | `██████░░░░░░░░░░░░░░` 偶尔，闭包捕获引用时 |
| [Implied lifetime bounds](./lifetimes_advanced.md#8-implied-lifetime-bounds) | `██░░░░░░░░░░░░░░░░░░` 极少，奇怪编译错误时 |
| [GAT + 生命周期](./lifetimes_advanced.md#9-gat--生命周期) | `█░░░░░░░░░░░░░░░░░░░` 极少，除非你在写库 |
| [Pin + 自引用 + 生命周期](./lifetimes_advanced.md#10-pin--自引用--生命周期) | `█░░░░░░░░░░░░░░░░░░░` 极少，除非手写 async runtime |

---

## 三、阅读建议

```
第 1 步 ── lifetimes_from_ts_basics.md  ← 必读，写 Rust 天天打交道
     ↓
第 2 步 ── rust_vs_typescript.md        ← 通读目录，之后当字典查
     ↓
第 3 步 ── lifetimes_advanced.md        ← 遇到相关编译错误时回来翻
```
