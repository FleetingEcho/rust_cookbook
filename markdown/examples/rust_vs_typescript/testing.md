# Rust vs TypeScript：测试系统

> **运行测试命令**：`cargo test -p learning_notes --example rts_testing`
> **运行演示命令**：`cargo run -p learning_notes --example rts_testing`

---

## TypeScript 参考版本

```ts
// TypeScript (vitest)
import { describe, it, expect, beforeAll, afterAll } from "vitest";

describe("数学运算", () => {
    it("加法", () => {
        expect(1 + 2).toBe(3);
    });

    it("减法", () => {
        expect(5 - 3).toBe(2);
    });
});

// 测试前后钩子
beforeAll(() => { /* 设置测试环境 */ });
afterAll(() => { /* 清理 */ });

// 异步测试
it("异步测试", async () => {
    const data = await fetchData();
    expect(data).toBeDefined();
});

// 异常测试
it("应该抛出错误", () => {
    expect(() => { throw new Error("fail"); }).toThrow("fail");
});
```

**关键差异**：
- Rust 用属性（`#[test]`）标记测试，TS 用函数包装（`describe`/`it`）
- Rust 的断言是宏（`assert_eq!`），TS 用 expect API
- Rust 测试可以出现在代码文件的任何位置（通过 `#[cfg(test)]` 条件编译）
- Rust 的测试是并行执行的（默认行为）

---

## 被测试的函数

```rust
/// 加法函数
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
    if x.len() >= y.len() { x } else { y }
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
```

---

## 一、基本测试

**TS**: `it("add", () => { expect(1+2).toBe(3) })`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // TS: expect(add(1, 2)).toBe(3)
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(9, 3), Ok(3));
    }

    #[test]
    fn test_divide_by_zero() {
        // TS: expect(() => divide(10, 0)).toThrow("除数不能为零")
        assert!(divide(10, 0).is_err());
        assert_eq!(
            divide(10, 0).unwrap_err(),
            String::from("除数不能为零")
        );
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
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
}
```

---

## 二、should_panic 测试

**TS**: `expect(() => { ... }).toThrow()`

```rust
#[test]
#[should_panic(expected = "索引越界")]
fn test_panic_on_out_of_bounds() {
    let v = vec![1, 2, 3];
    // TS: expect(() => { v[100] }).toThrow()
    let _ = v[100];  // 越界访问，触发 panic
}
```

---

## 三、ignore 测试（跳过测试）

**TS**: `it.skip("...", ...)`

```rust
#[test]
#[ignore]
fn test_very_slow() {
    std::thread::sleep(std::time::Duration::from_secs(5));
    assert_eq!(1, 1);
}

// 运行被忽略的测试：cargo test -- --ignored
```

---

## 四、测试模块与辅助函数

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 辅助函数（非测试，不被执行）
    fn setup_test_data() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    #[test]
    fn test_with_helper() {
        let data = setup_test_data();
        assert_eq!(data.len(), 5);
    }

    // 辅助函数也可以组合使用
    fn sorted_descending(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_by(|a, b| b.cmp(a));
        v
    }

    #[test]
    fn test_sorted_descending() {
        let data = setup_test_data();
        let sorted = sorted_descending(data);
        assert_eq!(sorted, vec![5, 4, 3, 2, 1]);
    }
}
```

---

## 五、子模块分组测试（模拟 TS 的 describe）

**TS**: `describe("数学", () => { ... })`

```rust
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
        assert!(!validate_email(""));  // 空字符串
        assert!(!validate_email("@.")); // 只有特殊字符
        assert!(validate_email("a@b.c"));    // 最小有效格式
        assert!(validate_email("test@test.co.uk"));
    }
}
```

---

## 六、文档测试（Doc Tests）

**TS** 没有直接对应，vitest 有 inline snapshots 但不同。

**文档中的代码块也会作为测试运行！**

```rust
/// ```
/// // 这是文档测试，会作为测试运行
/// assert_eq!(2 + 2, 4);
/// ```
pub fn doc_test_example() {
    // 文档测试确保文档中的代码示例总是最新的
}
```

---

## 运行命令

```bash
cargo test -p learning_notes --example rts_testing     # 运行此文件测试
cargo test -p learning_notes                           # 运行所有测试
cargo test -p learning_notes -- --nocapture            # 显示 println! 输出
cargo test -p learning_notes -- --test-threads=1       # 单线程运行
cargo test -p learning_notes -- --ignored              # 运行被忽略的测试
```

---

## 总结对照表

| TypeScript (vitest) | Rust |
|---|---|
| `describe`/`it` | `mod tests` / `#[test]` |
| `expect(x).toBe(y)` | `assert_eq!(x, y)` |
| `expect(fn).toThrow()` | `#[should_panic]` |
| `beforeAll` / `afterAll` | 在测试函数中手动调用 |
| `*.test.ts` 单独文件 | 被测试代码旁的 `#[cfg(test)]` |
| `toBe` / `toEqual` / `toMatch` | `assert!` / `assert_eq!` / `assert_ne!` |
| `async test` | `async fn test` + `tokio::test` |
| `it.skip` | `#[ignore]` |
| `describe.each` | 没有原生参数化测试（用宏模拟） |
| 文档测试（无） | doc tests：文档注释中的代码块 |
| 并行/串行可配置 | 默认并行，`--test-threads` 控制 |
| 覆盖率工具（c8/istanbul） | tarpaulin / grcov |
