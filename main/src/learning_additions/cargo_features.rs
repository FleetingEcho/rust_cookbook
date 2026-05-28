// Cargo features（特性标志）让一个 crate 可以有可选功能，
// 使用方按需开启，避免引入不需要的依赖或代码。
//
// ── Cargo.toml 里怎么声明 ────────────────────────────────────────────────────
//
// [features]
// default = ["logging"]       # 默认开启 logging
// logging = ["dep:log"]       # logging feature 依赖 log crate
// async   = ["dep:tokio"]     # async feature 依赖 tokio
// full    = ["logging","async"] # full 同时开启两者
//
// [dependencies]
// log   = { version = "0.4", optional = true }  # optional 依赖
// tokio = { version = "1",   optional = true, features = ["full"] }
//
// ── 使用方如何开启 ───────────────────────────────────────────────────────────
//
// Cargo.toml（使用方）：
//   [dependencies]
//   my-crate = { version = "1", features = ["async"] }
//
// 命令行：
//   cargo build --features async
//   cargo build --all-features
//   cargo build --no-default-features
//
// ── 在代码里使用 cfg 属性控制编译 ────────────────────────────────────────────

// #[cfg(feature = "...")] 让某段代码只在指定 feature 开启时编译。
// 下面的 "logging" 是演示用的虚构 feature，真实项目里需要在 Cargo.toml [features] 里声明。
// 这里用 allow(unexpected_cfgs) 消除警告，表明这是教学示例的有意为之。

// 模拟一个"logging" feature 存在时才编译的函数
#[cfg(feature = "logging")]
pub fn log_message(msg: &str) {
    // 真实场景：这里会调用 log::info!(...) 之类的宏
    println!("[LOG] {msg}");
}

// feature 没开启时提供一个什么都不做的替代实现，保持 API 一致
#[cfg(not(feature = "logging"))]
pub fn log_message(_msg: &str) {
    // 没有 logging feature 时，静默忽略
}

// ── cfg! 宏：运行时仍存在，但编译器会优化掉死分支 ───────────────────────────

pub fn show_cfg_macro() {
    // cfg! 返回 bool，编译器知道结果在编译期固定，会优化掉不可能执行的分支。
    if cfg!(target_os = "macos") {
        println!("运行在 macOS 上");
    } else if cfg!(target_os = "linux") {
        println!("运行在 Linux 上");
    } else {
        println!("其他平台");
    }

    // 区分 debug 和 release 构建（cargo build --release）
    if cfg!(debug_assertions) {
        println!("debug 构建（cargo build）");
    } else {
        println!("release 构建（cargo build --release）");
    }
}

// ── #[cfg(test)] ──────────────────────────────────────────────────────────────
// 最常见的 cfg 用法：只在 cargo test 时编译测试代码。
// 这你已经用过了，每个测试模块都有 #[cfg(test)]。

// ── 平台相关代码 ──────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
pub fn get_config_dir() -> &'static str {
    "C:\\AppData\\Roaming"
}

#[cfg(not(target_os = "windows"))]
pub fn get_config_dir() -> &'static str {
    "~/.config"
}

// ── target_arch / target_pointer_width ───────────────────────────────────────

pub fn show_arch_info() {
    println!("指针宽度: {} bits", std::mem::size_of::<usize>() * 8);

    // 只在 64 位平台编译这段代码
    #[cfg(target_pointer_width = "64")]
    println!("当前是 64 位平台");

    #[cfg(target_pointer_width = "32")]
    println!("当前是 32 位平台");
}

// ── 常用 cfg 条件速查 ─────────────────────────────────────────────────────────
//
//  条件                              含义
//  ──────────────────────────────    ──────────────────────────────────────────
//  feature = "name"                  指定 cargo feature 已开启
//  debug_assertions                  debug 构建（非 --release）
//  test                              cargo test 运行中
//  target_os = "macos/linux/windows" 目标操作系统
//  target_arch = "x86_64/aarch64"   CPU 架构
//  target_pointer_width = "64/32"   指针宽度
//  unix                              所有 Unix-like 系统（macOS、Linux 等）
//  windows                           Windows
//
//  组合写法：
//  #[cfg(all(unix, not(target_os = "macos")))]  -- Linux 但不是 macOS
//  #[cfg(any(target_os = "ios", target_os = "android"))]  -- 移动平台

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_message_does_not_panic() {
        // 无论 logging feature 是否开启，这个函数都应该可以调用。
        log_message("test message");
    }

    #[test]
    fn config_dir_returns_something() {
        let dir = get_config_dir();
        assert!(!dir.is_empty());
    }

    #[test]
    fn cfg_macro_returns_bool() {
        let is_debug = cfg!(debug_assertions);
        // 在 cargo test 里通常是 debug 构建，所以 is_debug 是 true。
        println!("is_debug: {is_debug}");
    }
}
