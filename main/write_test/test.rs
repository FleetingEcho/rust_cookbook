// Rust 测试指南
// Rust 提供了强大的测试框架，允许开发者验证代码的正确性。测试函数通常包括以下步骤：

// 设置所需的数据或状态
// 运行待测试的代码
// 使用断言 (assert) 检查返回值是否符合预期
// 本指南涵盖 Rust 测试的编写、运行方式，以及如何控制测试的执行。

// 1. 测试函数编写
// Rust 的测试函数需要使用 #[test] 进行标注，cargo new <package> --lib 会自动生成测试模块：

// 示例: 基本测试
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
// 运行测试
// $ cargo test
// 输出
// running 1 test
// test tests::it_works ... ok



// 2. 断言 (assert_eq!)
// Rust 提供多个断言宏:

// assert_eq!(a, b)：断言 a == b
// assert!(condition)：断言 condition 为 true
// assert_ne!(a, b)：断言 a != b

// 示例: 失败的测试
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
// 输出


// test tests::another ... FAILED
// failures:
//     tests::another


// 3. 自定义失败信息
// 默认的错误信息可能不够清晰，可以使用格式化参数增强可读性：

// 示例: 自定义断言消息

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("AAA");
        let target = "BBB";
        assert!(
            result.contains(target),
            "你的问候中没有包含目标姓名 {}，你的问候是 `{}`",
            target,
            result
        );
    }
}
// 失败输出
// thread 'tests::greeting_contains_name' panicked at '你的问候中没有包含目标姓名 BBB， 你的问候是 `Hello AAA!`'




// 4. 预期 panic 测试
// 使用 #[should_panic] 断言某个函数会 panic。

// 示例: 断言 panic

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }//实例化
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
// 输出
// test tests::greater_than_100 - should panic ... ok


// 5. 返回 Result<T, E> 进行测试
// 测试函数可以返回 Result<(), String> 以支持 ? 语法：


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// 6. 控制测试执行
// 6.1 运行单线程测试
// $ cargo test -- --test-threads=1

// 6.2 运行时显示 println! 输出
// $ cargo test -- --show-output

// 6.3 运行单个测试
// $ cargo test one_hundred

// 6.4 通过名称过滤测试
// $ cargo test add
// 运行包含 "add" 的测试。

// 6.5 忽略部分测试

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要很长时间运行的测试
}
// 运行被忽略的测试

// $ cargo test -- --ignored


// 6.6 组合过滤
// 运行 tests 模块中所有被忽略的测试：
// $ cargo test tests -- --ignored

// 运行 run 相关的被忽略测试：
// $ cargo test run -- --ignored

// 7. 开发依赖 (dev-dependencies)
// Rust 允许在测试中使用 dev-dependencies，如 pretty_assertions 提供更好的输出格式：

// 在 Cargo.toml 添加:
// [dev-dependencies]
// pretty_assertions = "1"


// 使用 pretty_assertions

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // 仅用于测试

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

// 8. 生成测试二进制文件
// cargo test 运行时会编译出测试二进制文件:

// $ cargo test
//  Finished test [unoptimized + debuginfo] target(s) in 0.00s
//  Running unittests (target/debug/deps/study_cargo-0d693f72a0f49166)
// 可以直接运行该文件:

// $ target/debug/deps/study_cargo-0d693f72a0f49166
// 只生成测试文件，不执行:

// $ cargo test --no-run

// 总结

// Rust 提供了强大的测试框架：
// #[test] 标注测试函数
// assert_eq!, assert!, assert_ne! 进行断言
// #[should_panic] 测试 panic
// Result<T, E> 允许使用 ?
// cargo test -- --show-output 显示 println!
// #[ignore] 忽略长时间运行的测试
// cargo test -- --ignored 运行被忽略的测试
// dev-dependencies 允许测试专用的依赖项
// 使用这些方法，可以有效管理和运行 Rust 测试，提高代码质量。