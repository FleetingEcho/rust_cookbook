// 🔑 要点：外部依赖通过 Cargo.toml 添加
// 在 Cargo.toml 中添加 [dependencies] anyhow = "1.0"
// 然后就可以 use anyhow::Error
// 独立编译时需要：`extern crate anyhow;` 或通过 cargo 编译

// 在完整项目中的用法：
// use anyhow::Error;
