// 运行测试命令：cargo test -p learning_notes --example rts_testing
// 运行演示命令：cargo run -p learning_notes --example rts_testing
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // TypeScript (vitest)
// import { describe, it, expect, beforeAll, afterAll } from "vitest";
//
// describe("数学运算", () => {
//     it("加法", () => {
//         expect(1 + 2).toBe(3);
//     });
//
//     it("减法", () => {
//         expect(5 - 3).toBe(2);
//     });
// });
//
// // 测试前后钩子
// beforeAll(() => { /* 设置测试环境 */ });
// afterAll(() => { /* 清理 */ });
//
// // 异步测试
// it("异步测试", async () => {
//     const data = await fetchData();
//     expect(data).toBeDefined();
// });
//
// // 异常测试
// it("应该抛出错误", () => {
//     expect(() => { throw new Error("fail"); }).toThrow("fail");
// });
// ============================================================
//
// 关键差异：
// - Rust 用属性（#[test]）标记测试，TS 用函数包装（describe/it）
// - Rust 的断言是宏（assert_eq!），TS 用 expect API
// - Rust 测试可以出现在代码文件的任何位置（通过 #[cfg(test)] 条件编译）
// - Rust 的测试是并行执行的（默认行为）

// ============================================================
// 被测试的函数（模拟一个简单的工具库）
// ============================================================

/// 加法函数（这个注释也是文档测试）
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 除法函数（返回 Result，处理除零）
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

/// 查找最长的字符串
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

/// 斐波那契数列
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// 验证邮箱地址（简单版）
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

// ============================================================
// 测试模块：用 #[cfg(test)] 条件编译
// TS 中测试文件通常单独存放（如 *.test.ts）
// Rust 中测试写在被测试代码旁边，编译时用 cfg(test) 排除
// ============================================================

// 这个模块只在 cargo test 时编译
// TS 对比：vitest 会自动发现 *.test.ts 文件
#[cfg(test)]
mod tests {
    // 导入父模块的所有内容
    use super::*;

    // ============================================================
    // 一、基本测试
    // TS: it("add", () => { expect(1+2).toBe(3) })
    // ============================================================

    #[test]
    fn test_add() {
        // TS: expect(add(1, 2)).toBe(3)
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_divide_ok() {
        // TS: expect(divide(10, 2)).toBe(5)
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(9, 3), Ok(3));
    }

    #[test]
    fn test_divide_by_zero() {
        // TS: expect(() => divide(10, 0)).toThrow("除数不能为零")
        assert!(divide(10, 0).is_err());
        assert_eq!(divide(10, 0).unwrap_err(), String::from("除数不能为零"));
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "de"), "abc");
        assert_eq!(longest("ab", "cde"), "cde");
        assert_eq!(longest("same", "same"), "same");
    }

    #[test]
    fn test_fibonacci() {
        // TS: expect(fibonacci(0)).toBe(0)
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com"));
        assert!(validate_email("alice@gmail.com"));
        assert!(!validate_email("noatsign"));
        assert!(!validate_email("no@dot"));
        assert!(!validate_email(""));
    }

    // ============================================================
    // 二、should_panic 测试
    // TS: expect(() => { ... }).toThrow()
    // ============================================================

    #[test]
    #[should_panic(expected = "索引越界")]
    fn test_panic_on_out_of_bounds() {
        let v = vec![1, 2, 3];
        // 尝试越界访问，预期 panic
        // TS: expect(() => { v[100] }).toThrow()
        if v[100] == 0 {
            // 不会执行到这里
        }
    }

    // 自定义 panic 的函数
    fn trigger_panic() {
        panic!("索引越界：访问了不存在的元素");
    }

    #[test]
    #[should_panic(expected = "索引越界")]
    fn test_custom_panic() {
        trigger_panic();
    }

    // ============================================================
    // 三、Result<T, E> 返回值测试（另一种写法）
    // TS: 异步测试返回 Promise
    // ============================================================

    #[test]
    fn test_divide_with_result_type() -> Result<(), String> {
        // 返回 Result，测试失败时返回 Err
        // TS: 测试函数返回 void，用 expect 断言
        assert_eq!(divide(10, 2)?, 5); // ? 如果 Err 则测试失败
        assert_eq!(divide(9, 3)?, 3);
        Ok(()) // 测试通过
    }

    // ============================================================
    // 四、忽略某个测试
    // TS: it.skip("skip test", ...)
    // ============================================================

    #[test]
    #[ignore = "这个测试太慢了，暂时忽略"]
    fn test_slow_fibonacci() {
        // 用 cargo test --include-ignored 可以运行
        assert_eq!(fibonacci(40), 102334155);
    }

    // ============================================================
    // 五、测试中的 setup/teardown（通过函数组合）
    // TS: beforeAll / afterAll / beforeEach / afterEach
    // ============================================================

    // 辅助函数：创建测试数据
    fn setup_test_data() -> Vec<i32> {
        // TS: beforeAll(() => { testData = [1,2,3,4,5] })
        vec![1, 2, 3, 4, 5]
    }

    #[test]
    fn test_with_setup() {
        let data = setup_test_data();
        assert_eq!(data.len(), 5);
        assert_eq!(data.iter().sum::<i32>(), 15);
    }

    #[test]
    fn test_using_helper() {
        // 测试也支持组合使用辅助函数
        fn sorted_descending(mut v: Vec<i32>) -> Vec<i32> {
            v.sort_by(|a, b| b.cmp(a));
            v
        }

        let data = setup_test_data();
        let sorted = sorted_descending(data);
        assert_eq!(sorted, vec![5, 4, 3, 2, 1]);
    }

    // ============================================================
    // 六、子模块分组测试（模拟 TS 的 describe）
    // TS: describe("数学", () => { ... })
    // ============================================================

    mod math_tests {
        use super::*;

        #[test]
        fn test_add_negative() {
            assert_eq!(add(-1, -2), -3);
            assert_eq!(add(-5, 5), 0);
        }

        #[test]
        fn test_add_large_numbers() {
            assert_eq!(add(1_000_000, 2_000_000), 3_000_000);
        }
    }

    mod string_tests {
        use super::*;

        #[test]
        fn test_longest_equal_length() {
            assert_eq!(longest("abc", "def"), "abc"); // >= 取第一个
        }

        #[test]
        fn test_longest_empty() {
            assert_eq!(longest("", "a"), "a");
            assert_eq!(longest("a", ""), "a");
        }

        #[test]
        fn test_validate_email_edge_cases() {
            assert!(!validate_email("")); // 空字符串
            assert!(!validate_email("@.")); // 只有特殊字符
                                            // 有效的 email 必须有 @ 和 .
            assert!(validate_email("a@b.c")); // 最小有效格式
            assert!(validate_email("test@test.co.uk"));
        }
    }
}

// ============================================================
// 文档测试（Doc Tests）
// TS 没有直接对应，vitest 有 inline snapshots 但不同
// 文档中的代码块也会作为测试运行！
// 运行: cargo test -p learning_notes
// ============================================================

/// ```
/// // 这是文档测试，会作为测试运行
/// // 使用 learning_notes::example_rts_testing 中的函数... 但实际上不行
/// // 因为 examples 不是 lib，所以我们在这里放一个简单的文档测试
/// assert_eq!(2 + 2, 4);
/// ```
pub fn doc_test_example() {
    // 文档测试确保文档中的代码示例总是最新的
}

// ============================================================
// main 函数：展示测试概念（仅演示）
// ============================================================

fn main() {
    println!("=== Rust 测试系统（运行用 cargo test，不是 cargo run）===");
    println!();
    println!("运行测试的命令：");
    println!("  cargo test -p learning_notes --example rts_testing");
    println!("  cargo test -p learning_notes                      # 运行所有测试");
    println!("  cargo test -p learning_notes -- --nocapture       # 显示 println! 输出");
    println!("  cargo test -p learning_notes -- --test-threads=1  # 单线程运行");
    println!("  cargo test -p learning_notes -- --ignored         # 运行被忽略的测试");
    println!();
    println!("=== TS vs Rust 测试对比 ===");
    println!("┌─────────────────────────────┬──────────────────────────────────┐");
    println!("│ TypeScript (vitest)         │ Rust                            │");
    println!("├─────────────────────────────┼──────────────────────────────────┤");
    println!("│ describe/it                 │ mod tests / #[test]             │");
    println!("│ expect(x).toBe(y)           │ assert_eq!(x, y)               │");
    println!("│ expect(fn).toThrow()        │ #[should_panic]                │");
    println!("│ beforeAll / afterAll        │ 在测试函数中手动调用            │");
    println!("│ *.test.ts 单独文件           │ 被测试代码旁的 #[cfg(test)]    │");
    println!("│ toBe / toEqual / toMatch    │ assert! / assert_eq! / assert_ne! │");
    println!("│ async test                  │ async fn test + tokio::test     │");
    println!("│ it.skip                     │ #[ignore]                      │");
    println!("│ describe.each               │ 没有原生参数化测试（用宏模拟）  │");
    println!("│ 文档测试（无）               │ doc tests：文档注释中的代码块  │");
    println!("│ 并行/串行可配置              │ 默认并行，--test-threads 控制  │");
    println!("│ 覆盖率工具（c8/istanbul）    │ tarpaulin / grcov             │");
    println!("└─────────────────────────────┴──────────────────────────────────┘");
}
