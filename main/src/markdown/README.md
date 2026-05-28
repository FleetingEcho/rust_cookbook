# main/src/markdown/ 文档说明

本目录包含 Rust ↔ TypeScript 对照学习的 Markdown 文档。

---

| 文件 | 定位 | 掌握程度 |
|------|------|----------|
| [`rust_vs_typescript.md`](rust_vs_typescript.md) | 27 个主题全景对照 | `████████░░░░░░░░░░░░` 知道有什么、在哪查即可 |
| [`lifetimes_from_ts_basics.md`](lifetimes_from_ts_basics.md) | 生命周期核心：为什么需要、三大场景、消除规则、编译错误解读、实战 | `████████████████████` **必须掌握** |
| [`lifetimes_advanced.md`](lifetimes_advanced.md) | 生命周期进阶：Variance / HRTB / GAT / Pin / async 等 | `████░░░░░░░░░░░░░░░░` 理解概念，遇到时回来翻 |

---

### 阅读建议

1. **先通读** `lifetimes_from_ts_basics.md` — 这是写 Rust 每天都要用的知识
2. **当字典用** `rust_vs_typescript.md` — 知道 27 个主题各自在哪，需要时翻
3. **有需要再看** `lifetimes_advanced.md` — 遇到 Variance / HRTB / Pin 等编译错误时回来查
