// Rust 单元测试与集成测试
// Rust 提供了 单元测试 (unit tests) 和 集成测试 (integration tests) 两种方式，以确保代码的正确性。两者的核心测试技术相同，但在 代码组织方式 上存在显著区别：

// 单元测试：测试单个代码单元（通常是函数），与被测试代码放在同一文件中。
// 集成测试：测试功能或 API 的整体行为，存放在 tests 目录 中，每个测试文件被视为一个独立的包。
// 1. 单元测试
// 1.1 编写单元测试
// 单元测试的目标是测试单个代码单元，确保其按预期运行。通常，测试代码会和被测试代码放在同一个文件，但封装在 #[cfg(test)] 标注的 tests 模块中。

// 示例: 单元测试

// // src/lib.rs

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*; //引入父模块

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
// 运行测试
// $ cargo test
// 输出
// running 1 test
// test tests::it_works ... ok



// 1.2 #[cfg(test)] 条件编译
// #[cfg(test)]
// 作用: 仅在 cargo test 时编译和运行 tests 模块，避免测试代码影响正常构建 (cargo build)。
// 优点:
// 减少编译时间
// 减小二进制文件体积




// 1.3 测试私有函数
// Rust 允许直接测试私有函数，只需在测试模块中 use super::*; 引入父模块的内容。

// 示例: 测试私有函数

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}



// 2. 集成测试
// 2.1 什么是集成测试？

// 单独的测试文件，位于 项目根目录下的 tests/ 目录。

// 只能测试 pub API（不能直接访问 private 代码）。
// 每个测试文件都是独立的包（需要 use <crate> 引入待测试代码）。



// 2.2 编写集成测试
// 步骤：

// 创建 tests/ 目录
// 在 tests/ 目录下创建测试文件
// 使用 use <crate> 引入 lib 库
// 示例: tests/integration_test.rs

// // tests/integration_test.rs
use adder; //adder是package name; 引入 src/lib.rs 作为库

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
// 这里 adder 对应 src/lib.rs 中定义的 crate 名称。

// 2.3 运行集成测试

// $ cargo test
// 输出
// Running unittests (target/debug/deps/adder-xxxxxx)
// test tests::it_works ... ok

// Running tests/integration_test.rs (target/debug/deps/integration_test-xxxxxx)
// test it_adds_two ... ok
// Rust 自动识别 tests/ 目录并运行其中的测试文件。


// 2.4 运行特定的集成测试
// 可以指定 tests/ 目录中的某个测试文件：
// $ cargo test --test integration_test
// 输出
// Running tests/integration_test.rs (target/debug/deps/integration_test-xxxxxx)
// test it_adds_two ... ok



// 3. 共享测试代码 (共享 setup 方法)
// 问题：

// 每个 tests/*.rs 文件是独立的包，无法直接共享代码。
// 需要在 tests/ 目录下创建 common 目录，放置可共享的测试代码。


// 3.1 共享 setup 函数
// 目录结构
// project/
// │── src/
// │   ├── lib.rs
// │── tests/
// │   ├── common/       # 共享代码目录
// │   │   ├── mod.rs    # 共享模块入口
// │   ├── integration_test.rs
// 编写共享 setup

// tests/common/mod.rs
pub fn setup() {
    // 初始化一些测试状态
    println!("Setting up test environment...");
}
// 在测试中使用 setup


// // tests/integration_test.rs
use adder;//package name
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// 这里使用 mod common; 声明 common 目录中的 mod.rs 作为共享模块。

// 4. Rust 对二进制项目的限制
// Rust 只支持对 lib 类型包 进行集成测试，对于 bin 类型 (如 src/main.rs) 无法直接测试。

// 解决方案：

// 将业务逻辑移入 lib.rs，仅在 main.rs 中调用。
// 这样 lib.rs 可被 tests/ 目录中的测试文件导入，从而进行集成测试。


// 5. 总结
// 类型	作用	位置	可访问的代码	运行方式
// 单元测试	测试单个函数	src/lib.rs 内部	pub & private	cargo test
// 集成测试	测试 API/功能	tests/ 目录	仅 pub API	cargo test --test <file>
// 共享测试代码	共享 setup	tests/common/mod.rs	仅在 tests/ 内部	mod common;

// 核心要点
// ✅ #[cfg(test)] 让单元测试代码仅在 cargo test 时编译。
// ✅ 集成测试位于 tests/ 目录，每个文件是独立的包，必须 use <crate> 导入。
// ✅ 共享代码放 tests/common/mod.rs，避免被 Rust 误认为是测试文件。
// ✅ 对二进制 (src/main.rs) 进行测试，需将逻辑移动到 lib.rs。

// Rust 提供了完善的测试体系，通过单元测试与集成测试结合，可以确保代码质量，并提高项目的可靠性。